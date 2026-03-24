#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::ledc::channel::ChannelHW;
use esp_hal::ledc::timer::config::Duty;
use esp_hal::ledc::timer::TimerIFace;
use esp_hal::ledc::LowSpeed;
use esp_hal::ledc::channel::ChannelIFace;
use esp_hal::{ledc, main};
use esp_hal::time::Rate;
use esp_println::println;
use panic_rtt_target as _;

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.2.0
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);

    let mosfet = Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default());
    let pwm = ledc::Ledc::new(peripherals.LEDC);


    let mut mosfet_timer = pwm.timer::<LowSpeed>(ledc::timer::Number::Timer0);
    let mut mosfet_channel = pwm.channel::<LowSpeed>(ledc::channel::Number::Channel1, mosfet);

    let _ = mosfet_timer.configure(ledc::timer::config::Config {
        duty: Duty::Duty14Bit,
        clock_source: esp_hal::ledc::timer::LSClockSource::APBClk,
        frequency: Rate::from_khz(1),
    });

    let _ = mosfet_channel.configure(ledc::channel::config::Config {
        timer: &mosfet_timer,
        duty_pct: 0, // Start with 0% duty cycle (off)
        drive_mode: esp_hal::gpio::DriveMode::PushPull
    });

    let delayer = Delay::new();

    loop {
        println!("dim");
        mosfet_channel.set_duty_hw(2355);
        delayer.delay_millis(500);
        println!("Bright");
        mosfet_channel.set_duty_hw(7000);
        delayer.delay_millis(500);
        
    }

}
