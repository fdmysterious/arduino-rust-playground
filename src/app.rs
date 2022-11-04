use arduino_hal::{
    hal::port::{PB5},
    hal::usart::{Usart0},
    port::{mode,Pin},

    hal as atmega_hal
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

const SERIAL_BAUDRATE: u32 = 115200; // bps

impl App {
    // Instanciate app and configure peripherals
    pub fn new(peripherals: arduino_hal::Peripherals) -> Self {
        let pins    = atmega_hal::pins!(peripherals);

        let led     = pins.pb5.into_output();
        let serial  =  arduino_hal::Usart::new(
            peripherals.USART0,
            pins.pd0,
            pins.pd1.into_output(),
            arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(SERIAL_BAUDRATE),
        );

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