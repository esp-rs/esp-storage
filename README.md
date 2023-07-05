# esp-storage

This implements [`embedded-storage`](https://github.com/rust-embedded-community/embedded-storage) traits to access unencrypted ESP32 flash.

## Current support

ESP32, ESP32-C2, ESP32-C3, ESP32-C6, ESP32-H2, ESP32-S2 and ESP32-S3 are supported in `esp-storage`

## Examples
- ESP32:
  1. Run the example:
     - `cargo +esp run --release --example demo --features esp32 --target xtensa-esp32-none-elf`
     - `cargo +esp run --release --example low_level --features "esp32,low-level" --target xtensa-esp32-none-elf`
- ESP32-C2:
  1. Uncomment the ESP32-C2 section, under `target.riscv32imc-unknown-none-elf.dev-dependencies` of the `Cargo-toml` file.
  2. Run the example:
     - `cargo "+nightly" run --example demo --features esp32c2 --target riscv32imc-unknown-none-elf`
     - `cargo "+nightly" run --example low_level --features "esp32c2,low-level" --target riscv32imc-unknown-none-elf`
- ESP32-C3:
  1. Uncomment the ESP32-C3 section, under `target.riscv32imc-unknown-none-elf.dev-dependencies` of the `Cargo-toml` file.
  2. Run the example:
     - `cargo "+nightly" run --example demo --features esp32c3 --target riscv32imc-unknown-none-elf`
     - `cargo "+nightly" run --example low_level --features "esp32c3,low-level" --target riscv32imc-unknown-none-elf`
- ESP32-C6:
  1. Uncomment the ESP32-C6 section, under `target.riscv32imac-unknown-none-elf.dev-dependencies` of the `Cargo-toml` file.
  2. Run the example:
     - `cargo "+nightly" run --example demo --features esp32c6 --target riscv32imac-unknown-none-elf`
     - `cargo "+nightly" run --example low_level --features "esp32c6,low-level" --target riscv32imac-unknown-none-elf`
- ESP32-H2:
  1. Uncomment the ESP32-H2 section, under `target.riscv32imac-unknown-none-elf.dev-dependencies` of the `Cargo-toml` file.
  2. Run the example:
     - `cargo "+nightly" run --example demo --features esp32h2 --target riscv32imac-unknown-none-elf`
     - `cargo "+nightly" run --example low_level --features "esp32h2,low-level" --target riscv32imac-unknown-none-elf`
- ESP32-S2:
  1. Run the example:
     - `cargo "+esp" run --example demo --features esp32s2 --target xtensa-esp32s2-none-elf`
     - `cargo +esp run --release --example low_level --features "esp32s2,low-level" --target xtensa-esp32s2-none-elf`
- ESP32-S3:
  1. Run the example:
     - `cargo "+esp" run --example demo --features esp32s3 --target xtensa-esp32s3-none-elf`
     - `cargo +esp run --release --example low_level --features "esp32s3,low-level" --target xtensa-esp32s3-none-elf`

## Important

For ESP32 it is necessary to build with [optimization level](https://doc.rust-lang.org/cargo/reference/profiles.html#opt-level) 2 or 3.

To make it work also for `debug` builds add this to your `Cargo.toml`

```toml
[profile.dev.package.esp-storage]
opt-level = 3
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
