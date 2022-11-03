use arduino_hal::{
    hal::port::{PB5},
    hal::usart::{Usart0},

    port::{mode,Pin}
};

type PinLed    = Pin<mode::Output, PB5>;
type UartDuino = Usart0<arduino_hal::DefaultClock>;

pub struct Board {
    // Pins
    pub led:    PinLed,

    // Peripherals
    pub serial: UartDuino
}

impl Board {
    pub fn new(peripherals: arduino_hal::Peripherals) -> Self {
        let pins   = arduino_hal::pins!(peripherals);

        let led    = pins.d13.into_output();
        let serial = arduino_hal::default_serial!(peripherals, pins, 115200);

        Self {
            led: led,
            serial: serial
        }
    }

    pub fn take() -> Option<Self> {
        Some(Self::new(
            arduino_hal::Peripherals::take()?
        ))
    }
}

//---------------

pub struct BoardSupport {
    pub serial: UartDuino,
    pub led:    PinLed,
}

impl BoardSupport {
    pub fn new(board: Board) -> Self {
        Self {
            serial: board.serial,
            led: board.led
        }
    }

    pub fn take() -> Option<Self> {
        Some(Self::new(
            Board::take()?
        ))
    }
}