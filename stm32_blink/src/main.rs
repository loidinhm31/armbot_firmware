#![no_std]
#![no_main]


use cortex_m_rt::entry;
use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};
use fugit::{Duration, ExtU32};
use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
};

#[allow(unused_imports)]
use panic_halt; // When a panic occurs, stop the microcontroller

// This marks the entrypoint of our application. The cortex_m_rt creates some
// startup code before this, but we don't need to worry about this
#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    // Acquire GPIOA and GPIOD peripherals
    let gpioa = dp.GPIOA.split();
    let gpiod = dp.GPIOD.split();

    // Configure PA0 as input (for reading button/switch status)
    let pa0 = gpioa.pa0.into_pull_down_input();

    // Configure PD12, PD13, PD14, PD15 as output (for controlling LEDs)
    let mut led12 = gpiod.pd12.into_push_pull_output();
    let mut led13 = gpiod.pd13.into_push_pull_output();
    let mut led14 = gpiod.pd14.into_push_pull_output();
    let mut led15 = gpiod.pd15.into_push_pull_output();

    // Set initial LED states (turn on LED13 and LED15)
    led13.set_low();
    led15.set_low();

    // Create and initialize a delay variable to manage delay loop
    let mut del_var: Duration<u32, 1, 1000> = 2001.millis();


    // Create a Millisecond Counter Handle
    let mut counter = dp.TIM1.counter_ms(&clocks);

    // Application Loop
    loop {
        // Start counter with del_var duration
        counter.start(del_var).unwrap();

        // Enter loop and check for button press until counter reaches del_var
        while counter.now().duration_since_epoch() < del_var - 1.millis() {
            // Decrease the delay value by 500 ms
            del_var = del_var - 500.millis();
            // If updated delay value drops below 500 ms then reset it back to starting value to 2 secs
            if del_var.to_millis() < 500_u32 {
                del_var = 2001.millis();
            }
        }

        led13.toggle();
        led15.toggle();

        // Read the status of PA0 (input pin)
        if pa0.is_high() {
            // If PA0 is high, turn on LED12 and LED14
            led12.set_high();
            led14.set_high();
        } else {
            // If PA0 is low, turn off LED12 and LED14
            led12.set_low();
            led14.set_low();
        }
    }
}