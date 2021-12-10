use nrf52840_hal::{
    gpio::{Level, Output, Pin, PushPull},
    pac::TIMER0,
    prelude::*,
    timer::OneShot,
    Timer,
};

pub struct Buzzer {
    pin: Pin<Output<PushPull>>,
}

impl Buzzer {
    pub fn new<Mode>(pin: Pin<Mode>) -> Self {
        Self {
            pin: pin.into_push_pull_output(Level::Low),
        }
    }

    pub fn buzz(&mut self, timer: &mut Timer<TIMER0, OneShot>) {
        let mut count: u16 = 0;

        while count < 500 {
            self.high();
            timer.delay_ms(1_u16);

            self.low();
            timer.delay_ms(1_u16);

            count += 2;
        }
    }

    fn high(&mut self) {
        self.pin.set_high().unwrap();
    }

    fn low(&mut self) {
        self.pin.set_low().unwrap();
    }
}
