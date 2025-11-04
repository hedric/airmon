//set gpio high
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
    //peripherals::{GPIO, GPIO8},
    //time::Duration,
};

//use esp_hal::delay::Delay;
//use esp_hal::gpio::Output;

pub struct Led<'a> {
    pin: &'a mut Output<'a>,
    delay: &'a mut Delay,
    color: u32,
}

pub fn bit_bang_high(delay: Delay, pin: &mut Output) {
    delay.delay_nanos(800 as u32);
    pin.set_high();
    //set gpio low
    info!("high");
}

pub fn bit_bang_low(delay: Delay, pin: &mut Output) {
    delay.delay_nanos(100 as u32);
    pin.set_low();
    info!("high");
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

        info!("RGB LED: changing color");
        for i in 0..32 {
            if (self.color >> i) & 1 == 1 {
                bit_bang_high(*self.delay, self.pin);
            } else {
                bit_bang_low(*self.delay, self.pin);
            }
        }
        info!("RGB LED: color change complete");
    }
}

pub fn test() {
    info!("LED test");
}
