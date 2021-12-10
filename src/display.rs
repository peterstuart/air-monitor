use crate::scd30;
use arrayvec::ArrayString;
use core::fmt::{self, Write as _};
use embedded_graphics::{
    geometry::Point,
    mono_font::{ascii::FONT_10X20, MonoFont, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use embedded_hal::blocking::{delay::DelayMs, spi::Write};
use epd_waveshare::{epd4in2::*, graphics::Display as _, prelude::*};
use nrf52840_hal::gpio::{Floating, Input, Level, Output, Pin, PushPull};

const LABEL_X: i32 = 20;
const VALUE_X: i32 = 220;

const Y_ORIGIN: i32 = 30;

const AIR_QUALITY_LABEL: &str = "Air Quality:";

const CO2_LABEL: &str = "Carbon Dioxide:";
const CO2_UNIT: &str = " ppm";

const TEMPERATURE_LABEL: &str = "Temperature:";
const TEMP_UNIT: &str = " F";

const HUMIDITY_LABEL: &str = "Humidity:";
const HUMIDITY_UNIT: &str = "%";

type Epd<Spi, Delay> = Epd4in2<
    Spi,
    Pin<Output<PushPull>>,
    Pin<Input<Floating>>,
    Pin<Output<PushPull>>,
    Pin<Output<PushPull>>,
    Delay,
>;

pub struct Display<Spi, Delay> {
    spi: Spi,
    delay: Delay,
    epd: Epd<Spi, Delay>,
    display: Display4in2,
}

impl<Spi, Delay> Display<Spi, Delay>
where
    Spi: Write<u8>,
    Spi::Error: fmt::Debug,
    Delay: DelayMs<u8>,
{
    pub fn new<CsMode, BusyMode, DcMode, RstMode>(
        mut spi: Spi,
        cs: Pin<CsMode>,
        busy: Pin<BusyMode>,
        dc: Pin<DcMode>,
        rst: Pin<RstMode>,
        mut delay: Delay,
    ) -> Self {
        let cs = cs.into_push_pull_output(Level::Low);
        let dc = dc.into_push_pull_output(Level::Low);
        let rst = rst.into_push_pull_output(Level::Low);
        let busy = busy.into_floating_input();

        let epd = Epd4in2::new(&mut spi, cs, busy, dc, rst, &mut delay).unwrap();

        Self {
            spi,
            delay,
            epd,
            display: Display4in2::default(),
        }
    }

    pub fn draw(&mut self, data: scd30::Data) {
        self.display.clear(BinaryColor::Off).unwrap();

        let title = Text::new(
            AIR_QUALITY_LABEL,
            Point::new(LABEL_X, Y_ORIGIN),
            MonoTextStyle::new(&FONT_10X20, BinaryColor::On),
        );
        let bottom = title.bounding_box().bottom_right().unwrap().y;
        title.draw(&mut self.display).unwrap();

        self.draw_labels_and_values(
            bottom + 32,
            &FONT_10X20,
            &[
                (CO2_LABEL, data.co2, CO2_UNIT),
                (
                    TEMPERATURE_LABEL,
                    data.temperature * 9.0 / 5.0 + 32.0,
                    TEMP_UNIT,
                ),
                (HUMIDITY_LABEL, data.humidity, HUMIDITY_UNIT),
            ],
        );

        self.epd
            .update_frame(&mut self.spi, self.display.buffer(), &mut self.delay)
            .unwrap();
        self.epd
            .display_frame(&mut self.spi, &mut self.delay)
            .expect("display frame new graphics");
    }

    fn draw_labels_and_values(
        &mut self,
        y: i32,
        font: &MonoFont,
        labels_and_values: &[(&str, f32, &str)],
    ) -> i32 {
        let mut y = y;

        for (label, value, unit) in labels_and_values {
            y = self.draw_label_and_value(label, *value, unit, y, font);
        }

        y
    }

    fn draw_label_and_value(
        &mut self,
        label: &str,
        value: f32,
        unit: &str,
        y: i32,
        font: &MonoFont,
    ) -> i32 {
        let label_text = Text::new(
            label,
            Point::new(LABEL_X, y),
            MonoTextStyle::new(font, BinaryColor::On),
        );

        let top = label_text.bounding_box().top_left.y;
        let bottom = label_text.bounding_box().bottom_right().unwrap().y;
        let height = bottom - top;

        label_text.draw(&mut self.display).unwrap();

        let mut buf = ArrayString::<12>::new();
        write!(&mut buf, "{:.0}{}", value, unit).expect("Failed to write to buffer");

        Text::new(
            &buf,
            Point::new(VALUE_X, y),
            MonoTextStyle::new(font, BinaryColor::On),
        )
        .draw(&mut self.display)
        .unwrap();

        bottom + height
    }
}
