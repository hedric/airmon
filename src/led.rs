//
//      Bit bang driver for the WS2812B
//      Bits encoded as follows:
//
//                 "Logic 0":
//         +-------+              +--
//         |       |              |
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

const WS2812_T0H_NS: u32 = 400;
const WS2812_T0L_NS: u32 = 850;

const WS2812_T1H_NS: u32 = 800;
const WS2812_T1L_NS: u32 = 450;

use defmt::info;
use esp_hal::{
    delay::Delay,
    gpio::Output,
//    time::Instant,
};

pub struct Led<'a> {
    pin: &'a mut Output<'a>,
    delay: &'a mut Delay,
}

pub fn bit_bang_0(delay: &mut Delay, pin: &mut Output) {
    pin.set_high();
    delay.delay_nanos(WS2812_T0H_NS);
    pin.set_low();
    delay.delay_nanos(WS2812_T0L_NS);
}

pub fn bit_bang_1(delay: &mut Delay, pin: &mut Output) {
    pin.set_high();
    delay.delay_nanos(WS2812_T1H_NS);
    pin.set_low();
    delay.delay_nanos(WS2812_T1L_NS);
}

impl<'a> Led<'a> {
    pub fn new(pin: &'a mut Output<'a>, delay: &'a mut Delay) -> Self {
        Self {
            pin,
            delay,
        }
    }

    pub fn set_color(&mut self, red: u8, green: u8, blue: u8) {

        let color = ((green as u32) << 16) | ((red as u32) << 8) | (blue as u32);

        info!("Setting color: {}", color);

        // Most significant bits are sent first
        for i in (0..24).rev() {
            if (color >> i) & 1 == 1 {
                bit_bang_1(self.delay, self.pin);
            } else {
                bit_bang_0(self.delay, self.pin);
            }
        }

        // Send reset pulse >50 us to latch data
        self.pin.set_low();
        self.delay.delay_micros(80);
    }
}

