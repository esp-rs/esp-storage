# esp-storage

This implements `embedded-storage` traits to access unencrypted ESP32 flash.

## Implementation status

ESP32, ESP32-S2, ESP32-S3 and ESP32-C3 work

## Run examples

`cargo "+esp" run --example demo --features esp32 --target xtensa-esp32-none-elf --release`

`cargo "+esp" run --example demo --features esp32s2 --target xtensa-esp32s2-none-elf`

`cargo "+esp" run --example demo --features esp32s3 --target xtensa-esp32s3-none-elf`

`cargo "+nightly" run --example demo --features esp32c3 --target riscv32imc-unknown-none-elf`

To run the example for ESP32-C2 you need to modify `Cargo-toml`, section `target.riscv32imc-unknown-none-elf.dev-dependencies` like this:

```toml
esp32c2-hal = { git = "https://github.com/esp-rs/esp-hal/", package = "esp32c2-hal" }
riscv-rt = { version = "0.9.0" }
riscv = { version = "0.9.0" }
esp-println = { version = "0.3.0", features = [ "esp32c2" ] }
esp-backtrace = { git = "https://github.com/esp-rs/esp-backtrace.git", features = [ "esp32c2", "panic-handler", "exception-handler" ] }
```

## Important

For ESP32 it is necessary to build with optimization level 2 or 3.

To make it work also for debug builds add this to your `Cargo.toml`

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
