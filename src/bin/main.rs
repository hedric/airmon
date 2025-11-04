#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::time::{Duration, Instant};
use {esp_backtrace as _, esp_println as _};

use airmon::led;
// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let mut gpio8 = Output::new(peripherals.GPIO8, Level::High, OutputConfig::default());
    let mut delay = Delay::new();

    let mut rgb_led = led::Led::new(&mut gpio8, &mut delay);

    loop {
        info!("Hello airmon!");
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
        rgb_led.set_color(255, 0, 0);
        while delay_start.elapsed() < Duration::from_millis(1000) {}
        rgb_led.set_color(0, 255, 0);
        while delay_start.elapsed() < Duration::from_millis(1500) {}
        rgb_led.set_color(0, 0, 255);
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.1/examples/src/bin
}
