# Airmon #
This project is intended for my own amusement and Rust learning. The main goal of the project is to have a bunch of air quality sensors connected to a ESP32 devboard. The ESP32 will then report the sensor data to a server on a local network via wifi. The data shall then be represented on a web gui of some sort.

This repository will hold both the firmware running on the ESP32 and the web gui software running on the server. The idea is to write all of it in Rust.

## Docs ##
[Rust on ESP Book](https://docs.espressif.com/projects/rust/book/)

## Toolchain ##
Toolchain installation (provided that Rust is installed)
```bash
rustup update
rustup toolchain install nightly --component rust-src
rustup target add riscv32imac-unknown-none-elf
```

## probe-rs ##
Embedded programming tool. Supports RTT.

Install probe-rs
```bash
curl -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```
## espflash ##
Serial flash cli utility tool for esp32 SoC's. Provided by Espressif.

Install espflash
```bash
cargo install espflash --locked
```
