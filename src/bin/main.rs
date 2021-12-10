#![no_main]
#![no_std]

#[rtic::app(device = nrf52840_hal::pac, dispatchers = [TIMER0, TIMER1], peripherals = true)]
mod app {
    use air_monitor::{self as _, Buzzer, Display, RgbLed, SCD30};
    use dwt_systick_monotonic::{DwtSystick, ExtU32};
    use nrf52840_hal::{
        clocks::HFCLK_FREQ,
        gpio::{p0::Parts as P0Parts, p1::Parts as P1Parts, Level},
        pac::{SPIM3, TIMER0, TIMER1, TWIM0},
        spim::{self, Spim},
        timer::OneShot,
        twim, Timer, Twim,
    };

    #[monotonic(binds = SysTick, default = true)]
    type DwtMono = DwtSystick<HFCLK_FREQ>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led_state: RgbLed,
        buzzer: Buzzer,
        sensor: SCD30<TWIM0>,
        timer: Timer<TIMER0, OneShot>,
        display: Display<Spim<SPIM3>, Timer<TIMER1, OneShot>>,
    }

    #[init]
    fn init(mut context: init::Context) -> (Shared, Local, init::Monotonics) {
        let mono = DwtSystick::new(
            &mut context.core.DCB,
            context.core.DWT,
            context.core.SYST,
            HFCLK_FREQ,
        );

        let board = context.device;
        let pins = P0Parts::new(board.P0);
        let timer = Timer::new(board.TIMER0);

        // LED
        let led_state = RgbLed::new(
            pins.p0_03.degrade(),
            pins.p0_04.degrade(),
            pins.p0_28.degrade(),
        );

        // Buzzer
        let buzzer = Buzzer::new(pins.p0_29.degrade());

        // SCD30
        let pins = twim::Pins {
            scl: pins.p0_30.into_floating_input().degrade(),
            sda: pins.p0_31.into_floating_input().degrade(),
        };
        let i2c = Twim::new(board.TWIM0, pins, twim::Frequency::K100);
        let mut sensor = SCD30::new(i2c);

        let firmware_version = sensor.get_firmware_version().unwrap();
        defmt::info!("SCD30 Firmware Version: {}", firmware_version);

        sensor.start_continuous_measurement(1020).unwrap();

        // Display
        let pins_1 = P1Parts::new(board.P1);
        let din = pins_1.p1_01.into_push_pull_output(Level::Low).degrade();
        let clk = pins_1.p1_02.into_push_pull_output(Level::Low).degrade();

        let spi_pins = spim::Pins {
            sck: clk,
            miso: None,
            mosi: Some(din),
        };

        let spi = Spim::new(
            board.SPIM3,
            spi_pins,
            spim::Frequency::K500,
            spim::MODE_0,
            0,
        );

        let delay = Timer::new(board.TIMER1);
        let display = Display::new(
            spi,
            pins_1.p1_03.degrade(),
            pins_1.p1_06.degrade(),
            pins_1.p1_04.degrade(),
            pins_1.p1_05.degrade(),
            delay,
        );

        read::spawn().unwrap();

        (
            Shared {},
            Local {
                led_state,
                buzzer,
                sensor,
                timer,
                display,
            },
            init::Monotonics(mono),
        )
    }

    #[task(priority = 1, local = [led_state, sensor, display])]
    fn read(context: read::Context) {
        let led_state = context.local.led_state;

        if let Some(data) = context.local.sensor.read_measurement().unwrap() {
            defmt::info!("{}", data);

            context.local.display.draw(data);

            if data.co2 < 800.0 {
                led_state.green();
            } else if data.co2 < 1000.0 {
                led_state.yellow();
            } else {
                led_state.red();
            }

            if data.co2 > 1400.0 {
                buzz::spawn().unwrap();
            }
        }

        read::spawn_after(30.secs()).unwrap();
    }

    #[task(priority = 2, local = [buzzer, timer])]
    fn buzz(context: buzz::Context) {
        context.local.buzzer.buzz(context.local.timer)
    }
}
