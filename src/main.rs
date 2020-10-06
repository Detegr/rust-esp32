#![no_std]
#![no_main]

use esp32_hal as hal;
use esp32_hal::target;
use hal::prelude::*;
use panic_halt as _;
use xtensa_lx::timer::delay;
use xtensa_lx_rt as _;
use xtensa_lx_rt::entry;

mod init;
use init::disable_watchdog_timers;

const CORE_HZ: u32 = 80_000_000;

#[entry]
fn main() -> ! {
    let mut peripherals = target::Peripherals::take().expect("Failed to obtain peripherals");

    disable_watchdog_timers(&mut peripherals);

    let pins = peripherals.GPIO.split();
    let mut led = pins.gpio2.into_push_pull_output();

    loop {
        // Blink away
        led.set_high().unwrap();
        delay(CORE_HZ);
        led.set_low().unwrap();
        delay(CORE_HZ);
    }
}
