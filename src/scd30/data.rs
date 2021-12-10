#[derive(Clone, Copy)]
pub struct Data {
    pub co2: f32,
    pub temperature: f32,
    pub humidity: f32,
}

impl defmt::Format for Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "{} ppm, {}°C, {}%",
            self.co2,
            self.temperature,
            self.humidity
        );
    }
}
