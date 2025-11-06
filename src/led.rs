//         |       |              |
//
//      Bit bang driver for the WS2812B
//      Bits encoded as follows:
//
//                 "Logic 0":
//         +-------+              +--
//         |       |              |
//         |       |              |
//         |       |--------------|
//         +       +              +
//         | 0.4us |    0.85us    |
//
//                 "Logic 1":
//         +-------------+         +--
//         |             |         |
//         |             |         |
//         |             |         |
//         +             +---------+
//         |    0.8us    | 0.45 us |

const RMT_CLOCK_FREQ_MHZ: u32 = 80;

const WS2812_T0H: u32 = RMT_CLOCK_FREQ_MHZ * 400 / 1000;
const WS2812_T0L: u32 = RMT_CLOCK_FREQ_MHZ * 850 / 1000;
const WS2812_T1H: u32 = RMT_CLOCK_FREQ_MHZ * 800 / 1000;
const WS2812_T1L: u32 = RMT_CLOCK_FREQ_MHZ * 450 / 1000;
const WS2812_RST: u32 = RMT_CLOCK_FREQ_MHZ * 50000 / 1000;

use defmt::info;
use esp_hal::{
    Blocking,
    gpio::{Level},
    rmt::{ChannelCreator, PulseCode, TxChannelConfig, TxChannelCreator},
};

pub struct Led<'a> {
    gpio: esp_hal::peripherals::GPIO8<'a>,
    rmt_channel: esp_hal::rmt::ChannelCreator<'a,Blocking, 0>,
}


impl<'a> Led<'a> {
    pub fn new(gpio: esp_hal::peripherals::GPIO8<'a>, rmt_channel: ChannelCreator<'a,Blocking,0>) -> Self {
        Self {
            gpio,
            rmt_channel,
        }
    }

    pub fn set_color(&mut self, red: u8, green: u8, blue: u8) {

        let color = ((blue as u32) << 16) | ((red as u32) << 8) | (green as u32);

        let transfer = self.send_data(&color);

        match transfer {
            Ok(_) => info!("Transmission successful"),
            Err(e) => info!("Transmission failed {:?}", e),
        }

        info!("Setting color: {}", color);

    }

    fn send_data(
        &mut self,
        color: &u32
    ) -> Result<(), esp_hal::rmt::Error> {

        let tx_config = TxChannelConfig::default()
            .with_clk_divider(1)
            .with_idle_output_level(Level::Low);

        let channel = self.rmt_channel.reborrow().configure_tx(self.gpio.reborrow(), tx_config).unwrap();

        let logic_0 = PulseCode::new(Level::High, WS2812_T0H as u16, Level::Low, WS2812_T0L as u16);
        let logic_1 = PulseCode::new(Level::High, WS2812_T1H as u16, Level::Low, WS2812_T1L as u16);
        let logic_rst = PulseCode::new(Level::Low, 0, Level::Low, WS2812_RST as u16);

        let mut data = [PulseCode::default(); 25];

        for i in (0..24).rev() {
            if (color >> i) & 1 == 1 {
                data[i] = logic_1;

            } else {
                data[i] = logic_0;
            }
        }

        data[24] = logic_rst;

        let transaction = channel.transmit(&data)?;

        match transaction.wait() {
            Ok(_) => (),
            Err((e, _)) => return Err(e),
        }

        info!("color transmitted: {}", color);

        Ok(())
    }
}

