pub(crate) const ESP_ROM_SPIFLASH_READ: u32 = 0x40062ed8;
pub(crate) const ESP_ROM_SPIFLASH_UNLOCK: u32 = 0x0; // ??? patched version ??? see components/spi_flash/esp32/spi_flash_rom_patch.c
pub(crate) const ESP_ROM_SPIFLASH_ERASE_SECTOR: u32 = 0x40062ccc;
pub(crate) const ESP_ROM_SPIFLASH_WRITE: u32 = 0x40062d50;
