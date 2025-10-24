# Airmon #
This project is intended for my own amusement and Rust learning. The main goal of the project is to have a bunch of air quality sensors connected to a ESP32 devboard. The ESP32 will then report the sensor data to a server on a local network via wifi. The data shall then be represented on a web gui of some sort.

This repository will hold both the firmware running on the ESP32 and the web gui software running on the server. The idea is to write all of it in Rust.

## Docs ##
* [Rust on ESP Book](https://docs.espressif.com/projects/rust/book/)
* [ESP32C6 Devkit](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitc-1/index.html)
* [ESP32C6 Datasheet](https://documentation.espressif.com/esp32-c6_datasheet_en.pdf)

## Hardware ##
* Board:  ESP32-C6-DevKitC-1-N8
* HP CPU:    160 MHz RISC-V
* LP CPU:    20  MHz RISC-V
* HP RAM:    512 kB
* LP RAM:    16 kB
* ROM:       32 kB
* SPI Flash: 8 MB
* and more...

## Tools ##

### Toolchain ###
Installation steps (provided that Rust is installed), where the target triple is: `riscv32imac-unknown-none-elf`
```bash
rustup update
rustup toolchain install nightly --component rust-src
rustup target add riscv32imac-unknown-none-elf
```

### probe-rs ###
An embedded programming and debugging tooltool. Among other feature it supports RTT.
```bash
curl -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

### espflash ###
A serial flash cli utility tool for esp32 SoC's. Provided by Espressif.
```bash
cargo install espflash --locked
```
