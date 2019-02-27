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

    fn turnOnLed(&mut self, num: u8) -> Result<(), ()> {
        let mut result = Ok(());
        //hprintln!("Turning on LED {}", num);
        self.peripherals.GPIOE.odr.modify(|_, w| {
            match num {
                0 => w.odr8().set_bit(),
                1 => w.odr9().set_bit(),
                2 => w.odr10().set_bit(),
                3 => w.odr11().set_bit(),
                4 => w.odr12().set_bit(),
                5 => w.odr13().set_bit(),
                6 => w.odr14().set_bit(),
                7 => w.odr15().set_bit(),
                _ => {
                    result = Err(());
                    w
                }
            }
        });

        return result;
    }


    fn turnOffLed(&mut self, num: u8) -> Result<(), ()> {
        let mut result = Ok(());
        //hprintln!("Turning off LED {}", num);
        self.peripherals.GPIOE.odr.modify(|_, w| {
            match num {
                0 => w.odr8().clear_bit(),
                1 => w.odr9().clear_bit(),
                2 => w.odr10().clear_bit(),
                3 => w.odr11().clear_bit(),
                4 => w.odr12().clear_bit(),
                5 => w.odr13().clear_bit(),
                6 => w.odr14().clear_bit(),
                7 => w.odr15().clear_bit(),
                _ => {
                    result = Err(());
                    w
                }
            }
        });

        return result;
    }

    fn changeLed(&mut self, num: u8, state: LEDState) -> Result<(), ()> {
        match state {
            LEDState::On => self.turnOnLed(num),
            LEDState::Off => self.turnOffLed(num),
        }
    }

    fn displayNumber(&mut self, num: u8) -> Result<(), ()> {
        hprintln!("Showing {}", num);
        for led in 0..8 {
            match (1 << led) & num == 0 {
                true => self.turnOffLed(led),
                false => self.turnOnLed(led),
            };
        }
        
        Ok(())
    }

}

#[entry]
fn main() -> ! {
    let mut board = Board::new();

    board.init();

    let mut i: u8;

    i = 120;

    loop {
        board.displayNumber(i);    
        i = i + 1;
    }
}
