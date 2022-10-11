//! SPI e-paper example
//!
//! This example prints some text on https://www.waveshare.com/product/2.9inch-e-paper-module.htm
//!
//! The following wiring is assumed:
//! - CS => GPIO10
//! - DC => GPIO21
//! - SCK => GPIO12
//! - RST => GPIO18
//! - BUSY => GPIO5
//! - MOSI => GPIO11

#![no_std]
#![no_main]

use esp32s3_hal::{
    clock::ClockControl,
    gpio::IO,
    pac::Peripherals,
    prelude::*,
    spi::{Spi, SpiMode},
    timer::TimerGroup,
    Delay, Rtc,
};

use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    prelude::*,
    text::{Baseline, Text, TextStyleBuilder},
};

use epd_waveshare::{
    color::*,
    epd2in9_v2::{Display2in9, Epd2in9},
    graphics::DisplayRotation,
    prelude::*,
};

use esp_backtrace as _;

#[xtensa_lx_rt::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt = timer_group0.wdt;
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);

    // Disable MWDT and RWDT (Watchdog) flash boot protection
    wdt.disable();
    rtc.rwdt.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let cs = io.pins.gpio10;
    let dc = io.pins.gpio11;
    let sck = io.pins.gpio12;
    let rst = io.pins.gpio18;

    let busy = io.pins.gpio5.into_floating_input();
    let mosi = io.pins.gpio13;

    let mut spi = Spi::new_no_cs_no_miso(
        peripherals.SPI2,
        sck,
        mosi,
        100u32.kHz(),
        SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &clocks,
    );

    let mut delay = Delay::new(&clocks);
    let mut epd = Epd2in9::new(
        &mut spi,
        cs.into_push_pull_output(),
        busy,
        dc.into_push_pull_output(),
        rst.into_push_pull_output(),
        &mut delay,
    )
    .unwrap();

    let mut display = Display2in9::default();
    display.set_rotation(DisplayRotation::Rotate90);

    draw_text(&mut display, " Hello Rust from ESP32S3! ", 15, 50);
    epd.update_and_display_frame(&mut spi, display.buffer(), &mut delay).expect("Frame cannot be cleared and updated!");

    loop {}
}

fn draw_text(display: &mut Display2in9, text: &str, x: i32, y: i32) {
    let style = MonoTextStyleBuilder::new()
        .font(&embedded_graphics::mono_font::ascii::FONT_10X20)
        .text_color(Black)
        .background_color(White)
        .build();

    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

    let _ = Text::with_text_style(text, Point::new(x, y), style, text_style).draw(display);
}
