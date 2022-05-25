#![no_std]

use embedded_storage::{ReadStorage, Storage};

#[cfg_attr(feature = "esp32c3", path = "esp32c3.rs")]
#[cfg_attr(feature = "esp32", path = "esp32.rs")]
#[cfg_attr(feature = "esp32s2", path = "esp32s2.rs")]
#[cfg_attr(feature = "esp32s3", path = "esp32s3.rs")]
mod chip_specific;

const FLASH_SECTOR_SIZE: u32 = 4096;

fn esp_rom_spiflash_read(src_addr: u32, data: *const u32, len: u32) -> i32 {
    unsafe {
        let esp_rom_spiflash_read: unsafe extern "C" fn(u32, *const u32, u32) -> i32 =
            core::mem::transmute(chip_specific::ESP_ROM_SPIFLASH_READ);
        esp_rom_spiflash_read(src_addr, data, len)
    }
}

fn esp_rom_spiflash_unlock() -> i32 {
    unsafe {
        let esp_rom_spiflash_unlock: unsafe extern "C" fn() -> i32 =
            core::mem::transmute(chip_specific::ESP_ROM_SPIFLASH_UNLOCK);
        esp_rom_spiflash_unlock()
    }
}

fn esp_rom_spiflash_erase_sector(sector_number: u32) -> i32 {
    unsafe {
        let esp_rom_spiflash_erase_sector: unsafe extern "C" fn(u32) -> i32 =
            core::mem::transmute(chip_specific::ESP_ROM_SPIFLASH_ERASE_SECTOR);
        esp_rom_spiflash_erase_sector(sector_number)
    }
}

fn esp_rom_spiflash_write(dest_addr: u32, data: *const u8, len: u32) -> i32 {
    unsafe {
        let esp_rom_spiflash_write: unsafe extern "C" fn(u32, *const u8, u32) -> i32 =
            core::mem::transmute(chip_specific::ESP_ROM_SPIFLASH_WRITE);
        esp_rom_spiflash_write(dest_addr, data, len)
    }
}

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

        let mut buffer = [0u8; 8];
        storage.read(0x0000, &mut buffer).ok();
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

impl ReadStorage for FlashStorage {
    type Error = FlashStorageError;

    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        if bytes.len() % 4 != 0 {
            return Err(FlashStorageError::Other(9999)); // TODO make this work - shouldn't be a requirement
        }
        let res = esp_rom_spiflash_read(
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

    /// The SPI flash size is configured by writing a field in the software bootloader image header.
    /// This is done during flashing in espflash / esptool.
    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl Storage for FlashStorage {
    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        if bytes.len() % 4 != 0 {
            return Err(FlashStorageError::Other(9999)); // TODO make this work - shouldn't be a requirement
        }

        if !self.unlocked {
            let res = esp_rom_spiflash_unlock();
            if res != 0 {
                return Err(FlashStorageError::Other(res));
            }
            self.unlocked = true;
        }

        let res = esp_rom_spiflash_erase_sector(offset / FLASH_SECTOR_SIZE);
        if res != 0 {
            return Err(FlashStorageError::Other(res));
        }

        let res = esp_rom_spiflash_write(offset, bytes.as_ptr() as *const u8, bytes.len() as u32);
        if res != 0 {
            Err(FlashStorageError::Other(res))
        } else {
            Ok(())
        }
    }
}
