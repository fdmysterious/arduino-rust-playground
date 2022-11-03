#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::prelude::*;

mod bsp;


///////////////////////////////////////////////////////////////////

#[arduino_hal::entry]
fn main() -> ! {
    let mut bsp = bsp::BoardSupport::take().unwrap();


    loop{
        bsp.led.toggle();
        arduino_hal::delay_ms(100);

        ufmt::uwriteln!(&mut bsp.serial, "Hello world\r").void_unwrap();
    }
}
