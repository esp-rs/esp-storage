fn main() -> Result<(), String> {
    if cfg!(feature = "esp32") {
        match std::env::var("OPT_LEVEL") {
            Ok(level) => {
                if level != "2" && level != "3" {
                    Err(format!("Building esp-storage for ESP32 needs optimization level 2 or 3 - yours is {}. See https://github.com/esp-rs/esp-storage", level))
                } else {
                    Ok(())
                }
            }
            Err(_err) => Ok(()),
        }
    } else {
        Ok(())
    }
}
