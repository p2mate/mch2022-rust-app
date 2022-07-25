Rust on the ESP32 of the MCH2022 badge
--------------------------------------

+ Install Rust toolchain from https://github.com/esp-rs/rust-build
+ Install cargo-generate (cargo install cargo-generate)
+ Install the mch2022 webusb tools from https://github.com/badgeteam/mch2022-tools
+ Create a new project as follows:
```
cargo generate --git https://github.com/esp-rs/esp-idf-template cargo
cd $PROJECT_NAME
rustup override set esp
```
+ You can generate an app image using:
```
cargo espflash ESP32 save-image rust_esp.img
```
+ This image can then be uploaded to the badge as follows:
```
webusb_push.py --run rust rust_esp.img
```
+ println! output appears on the first serial port exposed by the MCH022 badge
+ If your program panics, the badge will reboot to the main menu, use the serial
port to see any panic related messages.
+ All crates which work on ESP32 should be available

Limitations
-----------

+ These instructions use the esp-idf as provided by Espressiv so you won't have
access to the components added by the badge team. It's probably possible to use
their version, but I have not tried this.

Example code
------------

+ The code is a quick hack but should give you an idea on what you can do.

Known issues
------------

+ There's no VSYNC easily available so the icon doesn't move as smooth as it should
+ Drawing is rather slow probably because of the SPI interface and the overhead of
using the embedded_graphics crate. Maybe writing a dedicated driver for controlling
the display would result in better performance.

