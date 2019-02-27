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
pub enum LEDState {
    Off,
    On,
}

#[derive(Debug)]
pub enum ButtonState {
    Open,
    Closed,
}

pub struct Board {
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
        let gpioa = &self.peripherals.GPIOA;
 
        hprintln!("Enabling rcc.ahbenr").unwrap();
        rcc.ahbenr.modify(|_, w| {
            w.iopeen().set_bit();
            w.iopaen().set_bit();
            w
        });

        hprintln!("Configuring modes as outputs").unwrap();
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

        gpioa.moder.modify(|_, w| {
            w.moder0().input();
            w
        });

        gpioa.pupdr.modify(|_, w| {
            w.pupdr0().floating();
            w
        });

        hprintln!("Finished initializing").unwrap();
    }

    fn read_user_button(&self) -> ButtonState {
        //hprintln!("{:?}", self.peripherals.GPIOA.idr.read().bits());
        match self.peripherals.GPIOA.idr.read().idr0().is_high() {
            true => ButtonState::Closed,
            false => ButtonState::Open,
        }
    }

    fn turn_on_led(&mut self, num: u8) -> Result<(), ()> {
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


    fn turn_off_led(&mut self, num: u8) -> Result<(), ()> {
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

    pub fn change_led(&mut self, num: u8, state: LEDState) -> Result<(), ()> {
        match state {
            LEDState::On => self.turn_on_led(num),
            LEDState::Off => self.turn_off_led(num),
        }
    }

    fn display_number(&mut self, num: u8) -> Result<(), ()> {
        hprintln!("Showing {}", num).unwrap();
        for led in 0..8 {
            match (1 << led) & num == 0 {
                true => self.turn_off_led(led),
                false => self.turn_on_led(led),
            }?
        }
        
        Ok(())
    }

}



#[entry]
fn main() -> ! {
    let mut board = Board::new();

    board.init();

    let mut i: u8 = 0;

    loop {
        match board.read_user_button() {
            ButtonState::Open => i = i.wrapping_add(1),
            ButtonState::Closed => i = i.wrapping_sub(1),
        };

        board.display_number(i).unwrap();
    }
}
