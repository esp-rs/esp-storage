use core::arch::asm;

use crate::maybe_with_critical_section;

const ESP_ROM_SPIFLASH_READ: u32 = 0x40000a20;
const ESP_ROM_SPIFLASH_UNLOCK: u32 = 0x40000a2c;
const ESP_ROM_SPIFLASH_ERASE_SECTOR: u32 = 0x400009fc;
const ESP_ROM_SPIFLASH_WRITE: u32 = 0x40000a14;

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_read(src_addr: u32, data: *const u32, len: u32) -> i32 {
    maybe_with_critical_section(|| unsafe {
        let esp_rom_spiflash_read: unsafe extern "C" fn(u32, *const u32, u32) -> i32 =
            core::mem::transmute(ESP_ROM_SPIFLASH_READ);
        esp_rom_spiflash_read(src_addr, data, len)
    })
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_unlock() -> i32 {
    maybe_with_critical_section(|| unsafe {
        let esp_rom_spiflash_unlock: unsafe extern "C" fn() -> i32 =
            core::mem::transmute(ESP_ROM_SPIFLASH_UNLOCK);
        esp_rom_spiflash_unlock()
    })
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_erase_sector(sector_number: u32) -> i32 {
    maybe_with_critical_section(|| unsafe {
        let esp_rom_spiflash_erase_sector: unsafe extern "C" fn(u32) -> i32 =
            core::mem::transmute(ESP_ROM_SPIFLASH_ERASE_SECTOR);
        esp_rom_spiflash_erase_sector(sector_number)
    })
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_write(dest_addr: u32, data: *const u32, len: u32) -> i32 {
    maybe_with_critical_section(|| unsafe {
        let esp_rom_spiflash_write: unsafe extern "C" fn(u32, *const u32, u32) -> i32 =
            core::mem::transmute(ESP_ROM_SPIFLASH_WRITE);
        esp_rom_spiflash_write(dest_addr, data, len)
    })
}

pub fn park_other_core() -> bool {
    const SW_CPU_STALL: u32 = 0x600080bc;
    const OPTIONS0: u32 = 0x60008000;

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

pub fn unpark_other_core(enable: bool) {
    if enable {
        const SW_CPU_STALL: u32 = 0x600080bc;
        const OPTIONS0: u32 = 0x60008000;

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
