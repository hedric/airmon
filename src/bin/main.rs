#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::{main};
use esp_hal::rmt::{Rmt};
use esp_hal::time::{Duration, Instant, Rate};
use esp_hal::Blocking;
use {esp_backtrace as _, esp_println as _};

use airmon::led;
// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const RMT_CLOCK_FREQ_MHZ: u32 = 80;

#[main]
fn main() -> ! {

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let mut peripherals = esp_hal::init(config);

    let freq = Rate::from_mhz(RMT_CLOCK_FREQ_MHZ);
    let mut rmt: Rmt<'_, Blocking> = Rmt::new(peripherals.RMT, freq).unwrap();

    let mut led = led::Led::new(peripherals.GPIO8.reborrow(), rmt.channel0.reborrow());

    led.set_color(0,0,0);

    loop {

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(1000) {}
        info!("RED!");
        led.set_color(2,0,0);

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(1000) {}
        info!("GREEN!");
        led.set_color(0,2,0);

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(1000) {}
        info!("BLUE!");
        led.set_color(0,0,2);

    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.1/examples/src/bin
}
