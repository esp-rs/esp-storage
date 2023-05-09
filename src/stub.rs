use crate::maybe_with_critical_section;

#[cfg(not(doc))]
compile_error!("Select a target using feature: esp32c2, esp32c3, esp32c6, esp32, esp32s2, esp32s3");

pub(crate) fn esp_rom_spiflash_read(_src_addr: u32, _data: *const u32, _len: u32) -> i32 {
    maybe_with_critical_section(|| unimplemented!())
}

pub(crate) fn esp_rom_spiflash_unlock() -> i32 {
    maybe_with_critical_section(|| unimplemented!())
}

pub(crate) fn esp_rom_spiflash_erase_sector(_sector_number: u32) -> i32 {
    maybe_with_critical_section(|| unimplemented!())
}

pub(crate) fn esp_rom_spiflash_write(_dest_addr: u32, _data: *const u32, _len: u32) -> i32 {
    maybe_with_critical_section(|| unimplemented!())
}
