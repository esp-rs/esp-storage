#![no_std]

use embedded_storage::{ReadStorage, Storage};

#[cfg_attr(feature = "esp32c2", path = "esp32c2.rs")]
#[cfg_attr(feature = "esp32c3", path = "esp32c3.rs")]
#[cfg_attr(feature = "esp32c6", path = "esp32c6.rs")]
#[cfg_attr(feature = "esp32", path = "esp32.rs")]
#[cfg_attr(feature = "esp32s2", path = "esp32s2.rs")]
#[cfg_attr(feature = "esp32s3", path = "esp32s3.rs")]
#[cfg_attr(
    not(any(
        feature = "esp32c2",
        feature = "esp32c3",
        feature = "esp32c6",
        feature = "esp32",
        feature = "esp32s2",
        feature = "esp32s3"
    )),
    path = "stub.rs"
)]
mod chip_specific;

const FLASH_SECTOR_SIZE: u32 = 4096;

#[derive(Debug)]
pub enum FlashStorageError {
    Other(i32),
}

#[derive(Debug)]
pub struct FlashStorage {
    capacity: usize,
    unlocked: bool,
}

impl FlashStorage {
    pub fn new() -> FlashStorage {
        let mut storage = FlashStorage {
            capacity: 0,
            unlocked: false,
        };

        #[cfg(any(feature = "esp32c3", feature = "esp32s3"))]
        const ADDR: u32 = 0x0000;
        #[cfg(not(any(feature = "esp32c3", feature = "esp32s3")))]
        const ADDR: u32 = 0x1000;

        let mut buffer = [0u8; 8];
        storage.read(ADDR, &mut buffer).ok();
        let mb = match buffer[3] & 0xf0 {
            0x00 => 1,
            0x10 => 2,
            0x20 => 4,
            0x30 => 8,
            0x40 => 16,
            _ => 0,
        };
        storage.capacity = mb * 1024 * 1024;

        storage
    }
}

#[inline(never)]
#[link_section = ".rwtext"]
fn internal_read(offset: u32, bytes: &mut [u8]) -> Result<(), FlashStorageError> {
    if bytes.len() % 4 != 0 {
        return Err(FlashStorageError::Other(9999)); // TODO make this work - shouldn't be a requirement
    }

    let res = chip_specific::esp_rom_spiflash_read(
        offset,
        bytes.as_ptr() as *mut u8 as *mut u32,
        bytes.len() as u32,
    );

    if res != 0 {
        Err(FlashStorageError::Other(res))
    } else {
        Ok(())
    }
}

#[inline(never)]
#[link_section = ".rwtext"]
fn internal_write(
    storage: &mut FlashStorage,
    offset: u32,
    bytes: &[u8],
) -> Result<(), FlashStorageError> {
    if bytes.len() % 4 != 0 {
        return Err(FlashStorageError::Other(9999)); // TODO make this work - shouldn't be a requirement
    }

    if !storage.unlocked {
        if chip_specific::esp_rom_spiflash_unlock() != 0 {
            return Err(FlashStorageError::Other(9998));
        }
        storage.unlocked = true;
    }

    let res = chip_specific::esp_rom_spiflash_erase_sector(offset / FLASH_SECTOR_SIZE);
    if res != 0 {
        return Err(FlashStorageError::Other(res));
    }

    let res = chip_specific::esp_rom_spiflash_write(
        offset,
        bytes.as_ptr() as *const u8,
        bytes.len() as u32,
    );

    if res != 0 {
        Err(FlashStorageError::Other(res))
    } else {
        Ok(())
    }
}

impl ReadStorage for FlashStorage {
    type Error = FlashStorageError;

    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        let mut sector_start = (offset / FLASH_SECTOR_SIZE) * FLASH_SECTOR_SIZE;
        let mut data_offset = offset - sector_start;
        let mut dst_offset = 0;
        loop {
            let mut sector_data = [0u8; FLASH_SECTOR_SIZE as usize];
            internal_read(sector_start, &mut sector_data)?;

            let len = u32::min(
                FLASH_SECTOR_SIZE - data_offset,
                (bytes.len() - dst_offset) as u32,
            );

            bytes[dst_offset..][..len as usize]
                .copy_from_slice(&sector_data[data_offset as usize..][..len as usize]);

            sector_start += FLASH_SECTOR_SIZE;
            data_offset = 0;
            dst_offset += len as usize;

            if dst_offset >= bytes.len() {
                break Ok(());
            }
        }
    }

    /// The SPI flash size is configured by writing a field in the software bootloader image header.
    /// This is done during flashing in espflash / esptool.
    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl Storage for FlashStorage {
    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        let mut sector_start = (offset / FLASH_SECTOR_SIZE) * FLASH_SECTOR_SIZE;
        let mut data_offset = offset - sector_start;
        let mut dst_offset = 0;
        loop {
            let mut sector_data = [0u8; FLASH_SECTOR_SIZE as usize];
            internal_read(sector_start, &mut sector_data)?;

            let len = u32::min(
                FLASH_SECTOR_SIZE - data_offset,
                (bytes.len() - dst_offset) as u32,
            );

            sector_data[data_offset as usize..][..len as usize]
                .copy_from_slice(&bytes[dst_offset..][..len as usize]);
            internal_write(self, sector_start, &sector_data)?;

            sector_start += FLASH_SECTOR_SIZE;
            data_offset = 0;
            dst_offset += len as usize;

            if dst_offset >= bytes.len() {
                break Ok(());
            }
        }
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
fn maybe_with_critical_section<R>(f: impl FnOnce() -> R) -> R {
    #[cfg(feature = "critical-section")]
    return critical_section::with(|_| f());

    #[cfg(not(feature = "critical-section"))]
    f()
}

#[cfg(feature = "low-level")]
/// Low-level API
///
/// This gives you access to the underlying low level functionality.
/// These operate on raw pointers and all functions here are unsafe.
/// No pre-conditions are checked by any of these functions.
pub mod ll {
    pub unsafe fn spiflash_read(src_addr: u32, data: *const u32, len: u32) -> Result<(), i32> {
        match crate::chip_specific::esp_rom_spiflash_read(src_addr, data, len) {
            0 => Ok(()),
            value => Err(value),
        }
    }

    pub unsafe fn spiflash_unlock() -> Result<(), i32> {
        match crate::chip_specific::esp_rom_spiflash_unlock() {
            0 => Ok(()),
            value => Err(value),
        }
    }

    pub unsafe fn spiflash_erase_sector(sector_number: u32) -> Result<(), i32> {
        match crate::chip_specific::esp_rom_spiflash_erase_sector(sector_number) {
            0 => Ok(()),
            value => Err(value),
        }
    }

    pub unsafe fn spiflash_write(dest_addr: u32, data: *const u8, len: u32) -> Result<(), i32> {
        match crate::chip_specific::esp_rom_spiflash_write(dest_addr, data, len) {
            0 => Ok(()),
            value => Err(value),
        }
    }
}
