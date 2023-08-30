use core::arch::asm;

use crate::maybe_with_critical_section;

const ESP_ROM_SPIFLASH_READ: u32 = 0x40062ed8;
const ESP_ROM_SPIFLASH_ERASE_SECTOR: u32 = 0x40062ccc;
const SPI_READ_STATUS_HIGH: u32 = 0x40062448;
const SPI_WRITE_STATUS: u32 = 0x400622f0;

const CACHE_READ_DISABLE_ROM: u32 = 0x40009ab8;
const CACHE_FLUSH_ROM: u32 = 0x40009a14;
const CACHE_READ_ENABLE_ROM: u32 = 0x40009a84;

const SPI_BASE_REG: u32 = 0x3ff42000; /* SPI peripheral 1, used for SPI flash */
const SPI0_BASE_REG: u32 = 0x3ff43000; /* SPI peripheral 0, inner state machine */
const SPI_EXT2_REG: u32 = SPI_BASE_REG + 0xF8;
const SPI0_EXT2_REG: u32 = SPI0_BASE_REG + 0xF8;
const SPI_RD_STATUS_REG: u32 = SPI_BASE_REG + 0x10;
const SPI_ST: u32 = 0x7;
#[allow(clippy::identity_op)]
const SPI_CMD_REG: u32 = SPI_BASE_REG + 0x00;
const SPI_USER_REG: u32 = SPI_BASE_REG + 0x1c;
const SPI_USER1_REG: u32 = SPI_BASE_REG + 0x20;
const SPI_USR_DUMMY: u32 = 1 << 29;
const ESP_ROM_SPIFLASH_W_SIO_ADDR_BITSLEN: u32 = 23;
const SPI_USR_ADDR_BITLEN_M: u32 = 0x3f << 26;
const SPI_USR_ADDR_BITLEN_S: u32 = 26;
const PERIPHS_SPI_FLASH_ADDR: u32 = SPI_BASE_REG + 4;
const PERIPHS_SPI_FLASH_C0: u32 = SPI_BASE_REG + 0x80;
const SPI_FLASH_WREN: u32 = 1 << 30;
const SPI_FLASH_RDSR: u32 = 1 << 27;
const STATUS_WIP_BIT: u32 = 1 << 0;
const STATUS_QIE_BIT: u32 = 1 << 9; /* Quad Enable */
const SPI_CTRL_REG: u32 = SPI_BASE_REG + 0x08;
const SPI_WRSR_2B: u32 = 1 << 22;
const SPI_FLASH_WRDI: u32 = 1 << 29;

const FLASH_CHIP_ADDR: u32 = 0x3ffae270;

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn cache_read_disable_rom(cpu_num: u32) {
    unsafe {
        let cache_read_disable_rom: unsafe extern "C" fn(u32) =
            core::mem::transmute(CACHE_READ_DISABLE_ROM);
        cache_read_disable_rom(cpu_num)
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn cache_flush_rom(cpu_num: u32) {
    unsafe {
        let cache_flush_rom: unsafe extern "C" fn(u32) = core::mem::transmute(CACHE_FLUSH_ROM);
        cache_flush_rom(cpu_num)
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn cache_read_enable_rom(cpu_num: u32) {
    unsafe {
        let cache_read_enable_rom: unsafe extern "C" fn(u32) =
            core::mem::transmute(CACHE_READ_ENABLE_ROM);
        cache_read_enable_rom(cpu_num)
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn spi_read_status_high(
    flash_chip: *const EspRomSpiflashChipT,
    status: &mut u32,
) -> i32 {
    unsafe {
        let spi_read_status_high: unsafe extern "C" fn(
            *const EspRomSpiflashChipT,
            *mut u32,
        ) -> i32 = core::mem::transmute(SPI_READ_STATUS_HIGH);
        spi_read_status_high(flash_chip, status as *mut u32)
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn spi_write_status(flash_chip: *const EspRomSpiflashChipT, status_value: u32) -> i32 {
    unsafe {
        let spi_write_status: unsafe extern "C" fn(*const EspRomSpiflashChipT, u32) -> i32 =
            core::mem::transmute(SPI_WRITE_STATUS);
        spi_write_status(flash_chip, status_value)
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
fn begin() {
    cache_read_disable_rom(0);
    cache_read_disable_rom(1);
}

#[inline(always)]
#[link_section = ".rwtext"]
fn end() {
    cache_flush_rom(0);
    cache_flush_rom(1);
    cache_read_enable_rom(0);
    cache_read_enable_rom(1);
}

#[derive(Debug)]
#[repr(C)]
pub struct EspRomSpiflashChipT {
    device_id: u32,
    chip_size: u32, // chip size in bytes
    block_size: u32,
    sector_size: u32,
    page_size: u32,
    status_mask: u32,
}

#[inline(never)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_read(src_addr: u32, data: *const u32, len: u32) -> i32 {
    maybe_with_critical_section(|| {
        spiflash_wait_for_ready();
        unsafe {
            let esp_rom_spiflash_read: unsafe extern "C" fn(u32, *const u32, u32) -> i32 =
                core::mem::transmute(ESP_ROM_SPIFLASH_READ);
            esp_rom_spiflash_read(src_addr, data, len)
        }
    })
}

#[inline(never)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_erase_sector(sector_number: u32) -> i32 {
    maybe_with_critical_section(|| {
        let res = unsafe {
            let esp_rom_spiflash_erase_sector: unsafe extern "C" fn(u32) -> i32 =
                core::mem::transmute(ESP_ROM_SPIFLASH_ERASE_SECTOR);
            esp_rom_spiflash_erase_sector(sector_number)
        };

        if res != 0 {
            end();
        }
        res
    })
}

#[inline(never)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_write(dest_addr: u32, data: *const u32, len: u32) -> i32 {
    maybe_with_critical_section(|| {
        spiflash_wait_for_ready();
        begin();

        write_register(SPI_USER_REG, read_register(SPI_USER_REG) & !SPI_USR_DUMMY);
        let addrbits = ESP_ROM_SPIFLASH_W_SIO_ADDR_BITSLEN;
        let mut regval = read_register(SPI_USER1_REG);
        regval &= !SPI_USR_ADDR_BITLEN_M;
        regval |= addrbits << SPI_USR_ADDR_BITLEN_S;
        write_register(SPI_USER1_REG, regval);

        for block in (0..len).step_by(32) {
            spiflash_wait_for_ready();
            spi_write_enable();

            let block_len = if len - block < 32 { len - block } else { 32 };
            write_register(
                PERIPHS_SPI_FLASH_ADDR,
                ((dest_addr + block) & 0xffffff) | block_len << 24,
            );

            let data_ptr = unsafe { data.offset((block / 4) as isize) };
            for i in 0..block_len / 4 {
                write_register(PERIPHS_SPI_FLASH_C0 + (4 * i), unsafe {
                    data_ptr.offset(i as isize).read_volatile()
                });
            }

            write_register(SPI_RD_STATUS_REG, 0);
            write_register(SPI_CMD_REG, 1 << 25); // FLASH PP
            while read_register(SPI_CMD_REG) != 0 { /* wait */ }

            wait_for_ready();
        }

        end();
        0
    })
}

#[inline(always)]
#[link_section = ".rwtext"]
pub fn read_register(address: u32) -> u32 {
    unsafe { (address as *const u32).read_volatile() }
}

#[inline(always)]
#[link_section = ".rwtext"]
pub fn write_register(address: u32, value: u32) {
    unsafe {
        (address as *mut u32).write_volatile(value);
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
fn wait_for_ready() {
    while (read_register(SPI_EXT2_REG) & SPI_ST) != 0 {}
    while (read_register(SPI0_EXT2_REG) & SPI_ST) != 0 {} // ESP32_OR_LATER
}

#[inline(always)]
#[link_section = ".rwtext"]
fn spiflash_wait_for_ready() {
    loop {
        wait_for_ready();

        write_register(SPI_RD_STATUS_REG, 0);
        write_register(SPI_CMD_REG, SPI_FLASH_RDSR);
        while read_register(SPI_CMD_REG) != 0 {}
        if read_register(SPI_RD_STATUS_REG) & STATUS_WIP_BIT == 0 {
            return;
        }
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
fn spi_write_enable() {
    spiflash_wait_for_ready();

    write_register(SPI_RD_STATUS_REG, 0);
    // Write flash enable.  Write enable command will be sent when the bit is set. The bit will be cleared once the operation done.
    write_register(SPI_CMD_REG, SPI_FLASH_WREN);
    while read_register(SPI_CMD_REG) != 0 {}
}

#[inline(never)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_unlock() -> i32 {
    maybe_with_critical_section(|| {
        begin();
        let flashchip = FLASH_CHIP_ADDR as *const EspRomSpiflashChipT;
        let mut status: u32 = 0;

        spiflash_wait_for_ready(); /* ROM SPI_read_status_high() doesn't wait for this */
        if spi_read_status_high(flashchip, &mut status) != 0 {
            return -1;
        }

        let new_status = status & STATUS_QIE_BIT;

        // two bytes data will be written to status register when it is set
        // bit 12 = WAIT_FLASH_IDLE_EN RW wait flash idle when program flash or erase flash. 1: enable 0: disable.
        write_register(
            SPI_CTRL_REG,
            read_register(SPI_CTRL_REG) | SPI_WRSR_2B | (1 << 12),
        );

        spi_write_enable();
        if spi_write_status(flashchip, new_status) != 0 {
            end();
            return -1;
        }

        // WEL bit should be cleared after operations regardless of writing succeed or not.
        //  spiflash_wait_for_ready();
        write_register(SPI_CMD_REG, SPI_FLASH_WRDI);
        while read_register(SPI_CMD_REG) != 0 {}
        //  spiflash_wait_for_ready();

        end();
        0
    })
}

#[allow(clippy::identity_op)]
pub fn park_other_core() -> bool {
    const SW_CPU_STALL: u32 = 0x3ff480ac;
    const OPTIONS0: u32 = 0x3ff48000;

    let sw_cpu_stall = SW_CPU_STALL as *mut u32;
    let options0 = OPTIONS0 as *mut u32;

    let current = get_current_core();
    let other_was_running;

    match current {
        0 => unsafe {
            other_was_running = (options0.read_volatile() & 0b11) == 0;
            sw_cpu_stall
                .write_volatile(sw_cpu_stall.read_volatile() & !(0b111111 << 20) | (0x21 << 20));
            options0.write_volatile(options0.read_volatile() & !(0b11) | 0b10);
        },
        _ => unsafe {
            other_was_running = (options0.read_volatile() & 0b1100) == 0;
            sw_cpu_stall
                .write_volatile(sw_cpu_stall.read_volatile() & !(0b111111 << 26) | (0x21 << 26));
            options0.write_volatile(options0.read_volatile() & !(0b1100) | 0b1000);
        },
    }

    other_was_running
}

#[allow(clippy::identity_op)]
pub fn unpark_other_core(enable: bool) {
    if enable {
        const SW_CPU_STALL: u32 = 0x3ff480ac;
        const OPTIONS0: u32 = 0x3ff48000;

        let sw_cpu_stall = SW_CPU_STALL as *mut u32;
        let options0 = OPTIONS0 as *mut u32;

        let current = get_current_core();

        match current {
            0 => unsafe {
                sw_cpu_stall
                    .write_volatile(sw_cpu_stall.read_volatile() & !(0b111111 << 20) | (0x0 << 20));
                options0.write_volatile(options0.read_volatile() & !(0b11) | 0b00);
            },
            _ => unsafe {
                sw_cpu_stall
                    .write_volatile(sw_cpu_stall.read_volatile() & !(0b111111 << 26) | (0x0 << 26));
                options0.write_volatile(options0.read_volatile() & !(0b1100) | 0b0000);
            },
        }
    }
}

#[inline]
fn get_current_core() -> u8 {
    let mut x: u32;
    unsafe { asm!("rsr.prid {0}", out(reg) x, options(nostack)) };

    match ((x >> 13) & 1) != 0 {
        false => 0,
        true => 1,
    }
}
