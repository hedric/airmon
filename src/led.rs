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

pub fn test() {
    info!("LED test");
}
