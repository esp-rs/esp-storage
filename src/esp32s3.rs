use crate::maybe_with_critical_section;
use core::mem::transmute;

const ESP_ROM_SPIFLASH_READ: unsafe extern "C" fn(u32, *const u32, u32) -> i32 =
    transmute(0x40000a20);
const ESP_ROM_SPIFLASH_UNLOCK: unsafe extern "C" fn() -> i32 = transmute(0x40000a2c);
const ESP_ROM_SPIFLASH_ERASE_SECTOR: unsafe extern "C" fn(u32) -> i32 = transmute(0x400009fc);
const ESP_ROM_SPIFLASH_WRITE: unsafe extern "C" fn(u32, *const u8, u32) -> i32 =
    transmute(0x40000a14);

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_read(src_addr: u32, data: *const u32, len: u32) -> i32 {
    maybe_with_critical_section(|| unsafe { ESP_ROM_SPIFLASH_READ(src_addr, data, len) })
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_unlock() -> i32 {
    maybe_with_critical_section(|| unsafe { ESP_ROM_SPIFLASH_UNLOCK() })
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_erase_sector(sector_number: u32) -> i32 {
    maybe_with_critical_section(|| unsafe { ESP_ROM_SPIFLASH_ERASE_SECTOR(sector_number) })
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_write(dest_addr: u32, data: *const u8, len: u32) -> i32 {
    maybe_with_critical_section(|| unsafe { ESP_ROM_SPIFLASH_WRITE(dest_addr, data, len) })
}
