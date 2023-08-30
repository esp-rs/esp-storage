#![cfg_attr(not(all(test, feature = "emulation")), no_std)]
#![cfg_attr(target_arch = "xtensa", feature(asm_experimental_arch))]

#[cfg(not(feature = "emulation"))]
#[cfg_attr(feature = "esp32c2", path = "esp32c2.rs")]
#[cfg_attr(feature = "esp32c3", path = "esp32c3.rs")]
#[cfg_attr(feature = "esp32c6", path = "esp32c6.rs")]
#[cfg_attr(feature = "esp32h2", path = "esp32h2.rs")]
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
        feature = "esp32s3",
        feature = "esp32h2"
    )),
    path = "stub.rs"
)]
mod chip_specific;

#[cfg(feature = "emulation")]
#[path = "stub.rs"]
mod chip_specific;

#[cfg(any(feature = "storage", feature = "nor-flash"))]
mod common;

#[cfg(any(feature = "storage", feature = "nor-flash"))]
pub use common::{FlashStorage, FlashStorageError};

#[cfg(any(feature = "storage", feature = "nor-flash"))]
use common::FlashSectorBuffer;

#[cfg(feature = "storage")]
mod storage;

#[cfg(feature = "nor-flash")]
mod nor_flash;

#[cfg(feature = "low-level")]
pub mod ll;

#[cfg(not(feature = "emulation"))]
#[inline(always)]
#[link_section = ".rwtext"]
fn maybe_with_critical_section<R>(f: impl FnOnce() -> R) -> R {
    #[cfg(feature = "multicore-aware")]
    let was_running = chip_specific::park_other_core();

    #[cfg(feature = "critical-section")]
    let res = critical_section::with(|_| f());

    #[cfg(not(feature = "critical-section"))]
    let res = f();

    #[cfg(feature = "multicore-aware")]
    chip_specific::unpark_other_core(was_running);

    res
}

#[cfg(feature = "emulation")]
fn maybe_with_critical_section<R>(f: impl FnOnce() -> R) -> R {
    f()
}
