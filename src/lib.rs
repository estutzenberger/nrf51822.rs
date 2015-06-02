//! NRF51822 microcontroller
//!
//! This crate provides functionality common to STM32 microcontrollers
//!
//! - Interrupt vector
//!   - All the interrupts are weakly linked to the `abort` function provided by the `cortex`
//!     crate.
//! - Register definition of peripherals
//! - A `start` function. This function resets all the peripherals and then calls the external
//!   `main` function. If the `main` function exits, then the `abort` function is called.

#![feature(core)]
#![feature(no_std)]
#![no_std]

extern crate core;
extern crate cortex;
extern crate volatile;

#[macro_use]
extern crate reg;

pub mod power;
pub mod interrupt;
pub mod clock;
pub mod peripheral;
pub mod uicr;
pub mod ficr;
pub mod gpio;
pub mod rtc;
pub mod adc;
pub mod wdt;

fn reset_peripherals() {
    let power = peripheral::power();

    power.ramon.update(|ramon| {
    	use power::ramon::prelude::*;

    	ramon | ONRAM1 | ONRAM0
    });

    power.ramonb.update(|ramonb| {
    	use power::ramonb::prelude::*;

    	ramonb | ONRAM2 | ONRAM3
    });
}

#[no_mangle]
pub unsafe extern fn start() {
    extern {
        fn main();
    }

    reset_peripherals();

    main();

    cortex::abort();
}
