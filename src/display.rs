use arrayvec::ArrayString;
use core::fmt::{self, Write as _};
use embedded_graphics::{
    egtext,
    fonts::{Font12x16, Font24x32, Text},
    geometry::Point,
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyle,
    text_style,
};
use embedded_hal::blocking::{delay::DelayMs, spi::Write};
use epd_waveshare::{epd4in2::*, graphics::Display as _, prelude::*};
use nrf52840_hal::gpio::{Floating, Input, Level, Output, Pin, PushPull};

use crate::scd30;

const LABEL_X: i32 = 20;
const VALUE_X: i32 = 220;

const Y_ORIGIN: i32 = 30;

const AIR_QUALITY_LABEL: &str = "Air Quality:";

const CO2_LABEL: &str = "Carbon Dioxide:";
const CO2_UNIT: &str = " ppm";

const TEMPERATURE_LABEL: &str = "Temperature:";
const TEMP_UNIT: &str = "°F";

const HUMIDITY_LABEL: &str = "Humidity:";
const HUMIDITY_UNIT: &str = "%";

type Epd<Spi> = EPD4in2<
    Spi,
    Pin<Output<PushPull>>,
    Pin<Input<Floating>>,
    Pin<Output<PushPull>>,
    Pin<Output<PushPull>>,
>;

pub struct Display<Spi> {
    spi: Spi,
    epd: Epd<Spi>,
    display: Display4in2,
}

impl<Spi> Display<Spi>
where
    Spi: Write<u8>,
    Spi::Error: fmt::Debug,
{
    pub fn new<CsMode, BusyMode, DcMode, RstMode, Delay>(
        mut spi: Spi,
        cs: Pin<CsMode>,
        busy: Pin<BusyMode>,
        dc: Pin<DcMode>,
        rst: Pin<RstMode>,
        delay: &mut Delay,
    ) -> Self
    where
        Delay: DelayMs<u8>,
    {
        let cs = cs.into_push_pull_output(Level::Low);
        let dc = dc.into_push_pull_output(Level::Low);
        let rst = rst.into_push_pull_output(Level::Low);
        let busy = busy.into_floating_input();

        let epd = EPD4in2::new(&mut spi, cs, busy, dc, rst, delay).unwrap();

        Self {
            spi,
            epd,
            display: Display4in2::default(),
        }
    }

    pub fn draw(&mut self, data: scd30::Data) {
        self.display.clear(BinaryColor::Off).unwrap();

        let title = Text::new(AIR_QUALITY_LABEL, Point::new(LABEL_X, Y_ORIGIN))
            .into_styled(TextStyle::new(Font24x32, BinaryColor::On));
        let bottom = title.bottom_right().y;
        title.draw(&mut self.display).unwrap();

        self.draw_labels_and_values(
            bottom + 32,
            Font12x16,
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
            .update_frame(&mut self.spi, self.display.buffer())
            .unwrap();
        self.epd
            .display_frame(&mut self.spi)
            .expect("display frame new graphics");
    }

    fn draw_labels_and_values<F>(
        &mut self,
        y: i32,
        font: F,
        labels_and_values: &[(&str, f32, &str)],
    ) -> i32
    where
        F: Copy + Font,
    {
        let mut y = y;

        for (label, value, unit) in labels_and_values {
            y = self.draw_label_and_value(label, *value, unit, y, font);
        }

        y
    }

    fn draw_label_and_value<F>(
        &mut self,
        label: &str,
        value: f32,
        unit: &str,
        y: i32,
        font: F,
    ) -> i32
    where
        F: Copy + Font,
    {
        let text = Text::new(label, Point::new(LABEL_X, y))
            .into_styled(TextStyle::new(font, BinaryColor::On));

        let top = text.top_left().y;
        let bottom = text.bottom_right().y;
        let height = bottom - top;

        text.draw(&mut self.display).unwrap();

        let mut buf = ArrayString::<[_; 12]>::new();
        write!(&mut buf, "{:.0}{}", value, unit).expect("Failed to write to buffer");

        egtext!(
            text = &buf,
            top_left = (VALUE_X, y),
            style = text_style!(font = Font12x16, text_color = BinaryColor::On)
        )
        .draw(&mut self.display)
        .unwrap();

        bottom + height
    }
}
