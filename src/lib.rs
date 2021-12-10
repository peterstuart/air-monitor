#![no_std]

mod button;
mod buzzer;
mod display;
mod rgb_led;
mod scd30;
mod unit;

pub use button::Button;
pub use buzzer::Buzzer;
pub use display::Display;
pub use rgb_led::RgbLed;
pub use scd30::SCD30;
pub use unit::Unit;

use defmt_rtt as _; // global logger
use nrf52840_hal as _; // memory layout
use panic_probe as _;

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
