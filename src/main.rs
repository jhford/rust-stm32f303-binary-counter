#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

extern crate stm32f3;
use stm32f3::stm32f303;


#[entry]
fn main() -> ! {
    let mut peripherals = stm32f303::Peripherals::take().unwrap();

    let rcc = &peripherals.RCC;
    let gpioe = &peripherals.GPIOE;

    rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());
    hprintln!("Enabled GPIOE");

    gpioe.odr.read();

    hprintln!("Read from register");

    // configure the pins as outputs
    gpioe.moder.modify(|_, w| {
        w.moder8().output();
        w.moder9().output();
        w.moder10().output();
        w.moder11().output();
        w.moder12().output();
        w.moder13().output();
        w.moder14().output();
        w.moder15().output()
    });

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        w.odr8().set_bit();
        w.odr9().set_bit();
        w.odr10().set_bit();
        w.odr11().set_bit();
        w.odr12().set_bit();
        w.odr13().set_bit();
        w.odr14().set_bit();
        w.odr15().set_bit()
    });

    loop {
        // your code goes here
    }
}
