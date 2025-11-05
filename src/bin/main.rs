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
use esp_hal::rmt::{TxChannelConfig};
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::rmt::{Rmt, ChannelCreator, Channel, TxChannelCreator, PulseCode, TxChannelCreator};
use esp_hal::time::{Duration, Instant, Rate};
use {esp_backtrace as _, esp_println as _};

use airmon::led;
// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const RMT_CLOCK_FREQ_MHZ: u32 = 80;

const WS2812_T0H: u32 = RMT_CLOCK_FREQ_MHZ * 400 / 1000;
const WS2812_T0L: u32 = RMT_CLOCK_FREQ_MHZ * 850 / 1000;

const WS2812_T1H: u32 = RMT_CLOCK_FREQ_MHZ * 800 / 1000;
const WS2812_T1L: u32 = RMT_CLOCK_FREQ_MHZ * 450 / 1000;

const WS2812_RST: u32 = RMT_CLOCK_FREQ_MHZ * 50000 / 1000;


#[main]
fn main() -> ! {

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    //let mut gpio8 = Output::new(peripherals.GPIO8, Level::High, OutputConfig::default());
    let mut delay = Delay::new();

    //let mut rgb_led = led::Led::new(&mut gpio8, &mut delay);

    let freq = Rate::from_mhz(RMT_CLOCK_FREQ_MHZ);
    let rmt = Rmt::new(peripherals.RMT, freq).unwrap();

    let tx_config = TxChannelConfig::default().with_clk_divider(1);

    let logic_0 = PulseCode::new(Level::High, WS2812_T0H as u16, Level::Low, WS2812_T0L as u16);
    let logic_1 = PulseCode::new(Level::High, WS2812_T1H as u16, Level::Low, WS2812_T1L as u16);
    let logic_rst = PulseCode::new(Level::High, 0, Level::Low, WS2812_RST as u16);

    let mut channel = rmt.channel0.configure_tx(peripherals.GPIO8, tx_config);

    let red = 10;
    let green = 10;
    let blue = 10;

    let color = ((green as u32) << 16) | ((red as u32) << 8) | (blue as u32);

    let mut data = [PulseCode::default(); 25];

    for i in (0..24).rev() {
        if (color >> i) & 1 == 1 {
            //bit_bang_1(self.delay, self.pin);
            data[i] = logic_1;

        } else {
            //bit_bang_0(self.delay, self.pin);
            data[i] = logic_0;
        }
    }

    data[24] = logic_rst;

    //rgb_led.set_color(10, 0, 0);

    let transaction = channel.transmit(&data)?;
    channel = transaction.wait()?;


    loop {
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(1000) {}
        info!("test..");
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.1/examples/src/bin
}
