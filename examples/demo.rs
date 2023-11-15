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
