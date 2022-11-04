use arduino_hal::{
    hal::port::{PB5},
    hal::usart::{Usart0},
    port::{mode,Pin}
};

type PinLed    = Pin<mode::Output, PB5>;
type UartDuino = Usart0<arduino_hal::DefaultClock>;

struct AppInterfaces {
    pub led:    PinLed,
    pub serial: UartDuino,
}

struct AppStatus {
    counter: u32
}

pub struct App {
    hw: AppInterfaces,
    status: AppStatus,
}

impl App {
    // Instanciate app and configure peripherals
    pub fn new(peripherals: arduino_hal::Peripherals) -> Self {
        let pins    = arduino_hal::pins!(peripherals);
        let led     = pins.d13.into_output();

        let serial  = arduino_hal::default_serial!(peripherals, pins, 115200);

        Self {
            hw: AppInterfaces {
                led,
                serial,
            },

            status: AppStatus {
                counter: 0,
            },
        }
    }

    pub fn take() -> Option<Self> {
        Some(Self::new(
            arduino_hal::Peripherals::take()?
        ))
    }

    ///////////////////////////////////////////////////

    pub fn run_forever(&mut self) -> !{
        loop {
            self.hw.led.set_high();
            arduino_hal::delay_ms(30);
            self.hw.led.set_low();

            self.status.counter += 1;

            ufmt::uwriteln!(&mut self.hw.serial, "Hello world {}\r", self.status.counter).unwrap();
            arduino_hal::delay_ms(1000-30);
        }
    }
}