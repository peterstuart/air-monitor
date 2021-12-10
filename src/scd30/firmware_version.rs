#[derive(Clone, Copy)]
pub struct FirmwareVersion {
    pub major: u8,
    pub minor: u8,
}

impl FirmwareVersion {
    pub fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }
}

impl defmt::Format for FirmwareVersion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "{}.{}", self.major, self.minor)
    }
}
