#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embedded_hal::pwm::SetDutyCycle;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::ledc::timer::config::Duty;
use esp_hal::ledc::timer::TimerIFace;
use esp_hal::ledc::HighSpeed;
use esp_hal::ledc::channel::{Channel, ChannelIFace};
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

    let pwm = ledc::Ledc::new(peripherals.LEDC);

    let mut mosfet_timer = pwm.timer::<HighSpeed>(ledc::timer::Number::Timer0);
    let mut mosfet_channel = pwm.channel::<HighSpeed>(ledc::channel::Number::Channel1, peripherals.GPIO5);

    let _ = mosfet_timer.configure(ledc::timer::config::Config {
        duty: Duty::Duty14Bit,
        clock_source: esp_hal::ledc::timer::HSClockSource::APBClk,
        frequency: Rate::from_khz(1),
    });

    // btw its until 65535
    let _ = mosfet_channel.configure(ledc::channel::config::Config {
        timer: &mosfet_timer,
        duty_pct: 0, // Start with 0% duty cycle (off)
        drive_mode: esp_hal::gpio::DriveMode::PushPull
    });

    let delayer = Delay::new();

    let mut cycle: u16 = 50;
    let mut ledreturn: bool = false;

    loop {
        if cycle > 65500 {
            ledreturn = true;
        }
        else if cycle < 50 {
            ledreturn = false;
        }
        else if !ledreturn {
            cycle += 26;
        }
        else if ledreturn {
            cycle -= 26;
        }
        let _ = mosfet_channel.set_duty_cycle(cycle);
        delayer.delay_millis(1);
    }
}


















