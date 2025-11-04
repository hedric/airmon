//      Bit bang driver for the WS2812B
//      Bits encoded as follows:
//
//      "Logic 0":
//         +-------+              +--
//         |       |              |
//         |       |              |
//         |       |              |
//         |       |--------------|
//         +       +              +
//         | 0.4us |   0.85us     |
//
//      "Logic 1":
//         +-------------+       +--
//         |             |       |
//         |             |       |
//         |             |       |
//         |             |       |
//         +             +-------+
//         |    0.8us    | 0.4us |

use defmt::info;
use esp_hal::{
    delay::Delay,
    gpio::Output,
};


pub struct Led<'a> {
    pin: &'a mut Output<'a>,
    delay: &'a mut Delay,
    color: u32,
}

pub fn bit_bang_1(delay: Delay, pin: &mut Output) {
    pin.set_high();
    delay.delay_nanos(400 as u32);
    pin.set_low();
    delay.delay_nanos(850 as u32);
    pin.set_high();
}

pub fn bit_bang_0(delay: Delay, pin: &mut Output) {
    pin.set_high();
    delay.delay_nanos(800 as u32);
    pin.set_low();
    delay.delay_nanos(400 as u32);
    pin.set_high();
}

impl<'a> Led<'a> {
    pub fn new(pin: &'a mut Output<'a>, delay: &'a mut Delay) -> Self {
        Self {
            pin,
            delay,
            color: 0,
        }
    }

    pub fn set_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color |= (red as u32) << 16;
        self.color |= (green as u32) << 8;
        self.color |= blue as u32;

        info!("RGB LED: start sending bits to RGB LED");
        for i in 0..24 {
            if (self.color >> i) & 1 == 1 {
                bit_bang_1(*self.delay, self.pin);
            } else {
                bit_bang_0(*self.delay, self.pin);
            }
        }
        info!("RGB LED: done sending bits to RGB LED");
        info!("sent this color code: {}", self.color);
    }
}

