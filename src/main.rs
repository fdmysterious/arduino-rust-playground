#![no_std]
#![no_main]

use panic_halt as _;
mod app;


///////////////////////////////////////////////////////////////////

#[arduino_hal::entry]
fn main() -> ! {
    let mut app = app::App::take().unwrap();
    app.run_forever();
}
