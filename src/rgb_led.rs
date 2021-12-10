use nrf52840_hal::{
    gpio::{Level, Output, Pin, PushPull},
    prelude::*,
};

pub struct RgbLed {
    r: Pin<Output<PushPull>>,
    b: Pin<Output<PushPull>>,
    g: Pin<Output<PushPull>>,
}

impl RgbLed {
    pub fn new<RedMode, BlueMode, GreenMode>(
        led_red: Pin<RedMode>,
        led_blue: Pin<BlueMode>,
        led_green: Pin<GreenMode>,
    ) -> Self {
        Self {
            r: led_red.into_push_pull_output(Level::High),
            b: led_blue.into_push_pull_output(Level::High),
            g: led_green.into_push_pull_output(Level::High),
        }
    }

    pub fn off(&mut self) {
        self.r.set_high().unwrap();
        self.g.set_high().unwrap();
        self.b.set_high().unwrap();
    }

    pub fn red(&mut self) {
        self.r.set_low().unwrap();
        self.g.set_high().unwrap();
        self.b.set_high().unwrap();
    }

    pub fn green(&mut self) {
        self.r.set_high().unwrap();
        self.g.set_low().unwrap();
        self.b.set_high().unwrap();
    }

    pub fn blue(&mut self) {
        self.r.set_high().unwrap();
        self.g.set_high().unwrap();
        self.b.set_low().unwrap();
    }

    pub fn yellow(&mut self) {
        self.r.set_low().unwrap();
        self.g.set_low().unwrap();
        self.b.set_high().unwrap();
    }
}
