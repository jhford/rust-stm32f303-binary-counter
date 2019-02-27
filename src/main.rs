#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

extern crate stm32f3;
use stm32f3::stm32f303;

#[derive(Debug)]
enum LEDState {
    Off,
    On,
}

struct Board {
    peripherals: stm32f303::Peripherals
}

impl Board {
    fn new() -> Self {
        Board {
            peripherals: stm32f303::Peripherals::take().unwrap(),
        }
    }

    fn init(&mut self) {

        let rcc = &self.peripherals.RCC;
        let gpioe = &self.peripherals.GPIOE;
 
        hprintln!("Enabling rcc.ahbenr");
        rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());

        hprintln!("Configuring modes as outputs");
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

        hprintln!("Finished initializing");
    }

    fn change(&mut self, num: u8, state: LEDState) {
        hprintln!("Changing LED {} to {:?}", num, state);
        self.peripherals.GPIOE.odr.write(|w| {
            match num {
                0 => w.odr8().set_bit(),
                1 => w.odr9().set_bit(),
                2 => w.odr10().set_bit(),
                3 => w.odr11().set_bit(),
                4 => w.odr12().set_bit(),
                5 => w.odr13().set_bit(),
                6 => w.odr14().set_bit(),
                7 => w.odr15().set_bit(),
                _ => w.odr10().set_bit(),
            }
        });
    }

    fn ledOff(&mut self) {
        self.peripherals.GPIOE.odr.write(|w| {
            w.odr8().set_bit()
        });
    }
}

fn init () -> (stm32f3::stm32f303::RCC, stm32f3::stm32f303::GPIOE) {
    let peripherals = stm32f303::Peripherals::take().unwrap();

    let rcc = peripherals.RCC;
    let gpioe = peripherals.GPIOE;

    return (rcc, gpioe);
}

#[entry]
fn main() -> ! {
    let mut board = Board::new();

    board.init();

    board.change(0, LEDState::On);
    board.change(1, LEDState::On);
    board.change(2, LEDState::On);
    board.change(3, LEDState::On);
    board.change(4, LEDState::On);
    board.change(5, LEDState::On);
    board.change(6, LEDState::On);
    board.change(7, LEDState::On);

    loop {
        // your code goes here
    }
}
