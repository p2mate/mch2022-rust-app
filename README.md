Rust on the ESP32 of the MCH2022 badge
--------------------------------------

+ Install Rust toolchain from https://github.com/esp-rs/rust-build:
  ```
  git clone https://github.com/esp-rs/rust-build.git
  cd rust-build
  ./install-rust-toolchain.sh --extra-crates "ldproxy cargo-espflash wokwi-server web-flash" \
    --export-file /home/<user>/export-esp.sh \
    --esp-idf-version "release/v4.4" \
    --minified-esp-idf "YES" \
    --build-target "esp32" \
  source /home/<user>/export-esp-rust.sh  
  ```
+ Install cargo-generate:
  ```
  cargo install cargo-generate
  ```
+ Install the mch2022 webusb tools from https://github.com/badgeteam/mch2022-tools:
  ```
  pip install pyusb
  ```
+ Create a new project as follows:
```
cargo generate --git https://github.com/esp-rs/esp-idf-template cargo
cd <project-name>
```
+ You can generate an app image using:
```
cargo espflash ESP32 save-image rust_esp.img
```
+ This image can then be uploaded to the badge as follows:
```
webusb_push.py --run rust rust_esp.img
```
+ If you would like to build and upload the code, you can use:
    - Terminal approach:
      - Using `flash.sh` script:

        ```
        scripts/flash.sh [debug | release]
        ```
        > If no argument is passed, `release` will be used as default

    - UI approach:
        - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
        select `Build & Flash`.
        - From UI: Press `Build & Flash` on the left side of the Status Bar.
    - Any alternative flashing method from host machine.

+ `println!` output appears on the first serial port exposed by the MCH022 badge
+ If your program panics, the badge will reboot to the main menu, use the serial
port to see any panic related messages.
+ All crates which work on ESP32 should be available

Limitations
-----------

+ These instructions use the esp-idf as provided by Espressif so you won't have
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

Dev Containers
------------
This repository offers ready-to-use devlopment environments via Dev Containers for:
-  [Gitpod](https://gitpod.io/)[![Open ESP32 in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/github.com/SergioGasquez/mch2022-rust-app)
-  [VS Code Dev Containers](https://code.visualstudio.com/docs/remote/containers#_quick-start-open-an-existing-folder-in-a-container)
-  [GitHub Codespaces](https://docs.github.com/en/codespaces/developing-in-codespaces/creating-a-codespace)
> **Note**
>
> In [order to use GitHub Codespaces](https://github.com/features/codespaces#faq)
> the project needs to be published in a GitHub repository and the user needs
> to be part of the Codespaces beta or have the project under an organization.

If using VS Code or GitHub Codespaces, you can pull the image instead of building it
from the Dockerfile by selecting the `image` property instead of `build` in
`.devcontainer/devcontainer.json`. Further customization of the Dev Container can
be achived, see [.devcontainer.json reference](https://code.visualstudio.com/docs/remote/devcontainerjson-reference).

When using Dev Containers, some tooling to facilitate building, flashing and
simulating in Wokwi is also added.
### Build
- Terminal approach:

    ```
    scripts/build.sh  [debug | release]
    ```
    > If no argument is passed, `release` will be used as default


-  UI approach:

    The default build task is already set to build the project, and it can be used
    in VS Code and Gitpod:
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Build Task` command.
    - `Terminal`-> `Run Build Task` in the menu.
    - With `Ctrl-Shift-B` or `Cmd-Shift-B`.
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build`.
    - From UI: Press `Build` on the left side of the Status Bar.

### Flash

At the momment there is no way to flash the Badge from the containers

### Wokwi Simulation
When using a custom Wokwi project, please change the `WOKWI_PROJECT_ID` in
`run-wokwi.sh`. If no project id is specified, a DevKit for esp32 will be
used.
> **Warning**
>
>  ESP32-S3 is not available in Wokwi

- Terminal approach:

    ```
    scripts/run-wokwi.sh [debug | release]
    ```
    > If no argument is passed, `release` will be used as default

- UI approach:

    The default test task is already set to build the project, and it can be used
    in VS Code and Gitpod:
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Test Task` command
    - With `Ctrl-Shift-,` or `Cmd-Shift-,`
        > **Note**
        >
        > This Shortcut is not available in Gitpod by default.
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build & Run Wokwi`.
    - From UI: Press `Build & Run Wokwi` on the left side of the Status Bar.

> **Warning**
>
>  The simulation will pause if the browser tab is in the background.This may
> affect the execution, specially when debuging.

#### Debuging with Wokwi

Wokwi offers debugging with GDB.

- Terminal approach:
    ```
    $HOME/.espressif/tools/xtensa-esp32-elf/esp-2021r2-patch3-8.4.0/xtensa-esp32-elf/bin/xtensa-esp32-elf-gdb target/xtensa-esp32-espidf/debug/mch -ex "target remote localhost:9333"
    ```

    > [Wokwi Blog: List of common GDB commands for debugging.](https://blog.wokwi.com/gdb-avr-arduino-cheatsheet/?utm_source=urish&utm_medium=blog)
- UI approach:
    1. Run the Wokwi Simulation in `debug` profile
    2. Go to `Run and Debug` section of the IDE (`Ctrl-Shift-D or Cmd-Shift-D`)
    3. Start Debugging by pressing the Play Button or pressing `F5`
    4. Choose the proper user:
        - `esp` when using VS Code or GitHub Codespaces
        - `gitpod` when using Gitpod
