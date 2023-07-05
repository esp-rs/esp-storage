#![no_std]
#![no_main]

use embedded_storage::{ReadStorage, Storage};
#[cfg(feature = "esp32")]
use esp32_hal as hal;

#[cfg(feature = "esp32s2")]
use esp32s2_hal as hal;

#[cfg(feature = "esp32s3")]
use esp32s3_hal as hal;

#[cfg(feature = "esp32c3")]
use esp32c3_hal as hal;

#[cfg(feature = "esp32c2")]
use esp32c2_hal as hal;

#[cfg(feature = "esp32c6")]
use esp32c6_hal as hal;

#[cfg(feature = "esp32h2")]
use esp32h2_hal as hal;

use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc};

use esp_storage::FlashStorage;

use esp_backtrace as _;
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();

    #[cfg(any(feature = "esp32", feature = "esp32s2", feature = "esp32s3"))]
    {
        #[cfg(feature = "esp32")]
        let system = peripherals.DPORT.split();
        #[cfg(not(feature = "esp32"))]
        let system = peripherals.SYSTEM.split();

        let mut clock_control = system.peripheral_clock_control;

        let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

        let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks, &mut clock_control);
        let mut wdt = timer_group0.wdt;
        let mut rtc = Rtc::new(peripherals.RTC_CNTL);

        // Disable MWDT and RWDT (Watchdog) flash boot protection
        wdt.disable();
        rtc.rwdt.disable();
    }

    #[cfg(any(feature = "esp32c3", feature = "esp32c2"))]
    {
        let system = peripherals.SYSTEM.split();
        let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
        let mut clock_control = system.peripheral_clock_control;

        let mut rtc = Rtc::new(peripherals.RTC_CNTL);
        let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks, &mut clock_control);
        let mut wdt0 = timer_group0.wdt;

        #[cfg(not(feature = "esp32c2"))]
        let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks, &mut clock_control);
        #[cfg(not(feature = "esp32c2"))]
        let mut wdt1 = timer_group1.wdt;

        rtc.swd.disable();
        rtc.rwdt.disable();
        wdt0.disable();
        #[cfg(not(feature = "esp32c2"))]
        wdt1.disable();
    }

    #[cfg(any(feature = "esp32c6", feature = "esp32h2"))]
    {
        let system = peripherals.PCR.split();
        let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
        let mut clock_control = system.peripheral_clock_control;

        let mut rtc = Rtc::new(peripherals.LP_CLKRST);
        let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks, &mut clock_control);
        let mut wdt0 = timer_group0.wdt;

        #[cfg(not(feature = "esp32c2"))]
        let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks, &mut clock_control);
        #[cfg(not(feature = "esp32c2"))]
        let mut wdt1 = timer_group1.wdt;

        rtc.swd.disable();
        rtc.rwdt.disable();
        wdt0.disable();
        #[cfg(not(feature = "esp32c2"))]
        wdt1.disable();
    }

    let mut bytes = [0u8; 32];

    let mut flash = FlashStorage::new();

    let flash_addr = 0x9000;
    println!("Flash size = {}", flash.capacity());

    flash.read(flash_addr, &mut bytes).unwrap();
    println!("Read from {:x}:  {:02x?}", flash_addr, &bytes[..32]);

    bytes[0x00] = bytes[0x00].wrapping_add(1);
    bytes[0x01] = bytes[0x01].wrapping_add(2);
    bytes[0x02] = bytes[0x02].wrapping_add(3);
    bytes[0x03] = bytes[0x03].wrapping_add(4);
    bytes[0x04] = bytes[0x04].wrapping_add(1);
    bytes[0x05] = bytes[0x05].wrapping_add(2);
    bytes[0x06] = bytes[0x06].wrapping_add(3);
    bytes[0x07] = bytes[0x07].wrapping_add(4);

    flash.write(flash_addr, &bytes).unwrap();
    println!("Written to {:x}: {:02x?}", flash_addr, &bytes[..32]);

    let mut reread_bytes = [0u8; 32];
    flash.read(flash_addr, &mut reread_bytes).unwrap();
    println!("Read from {:x}:  {:02x?}", flash_addr, &reread_bytes[..32]);

    loop {}
}
