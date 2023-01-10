#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;

use longan_nano::hal::{pac, prelude::*};
use longan_nano::hal::delay::McycleDelay;

use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.configure().freeze();
    
    let gpioa = dp.GPIOA.split(&mut rcu);
    let mut pa7 = gpioa.pa7.into_push_pull_output();
    
    let mut delay = McycleDelay::new(&rcu.clocks);

    loop {
        pa7.set_low().unwrap();
        delay.delay_ms(500);
        pa7.set_high().unwrap();
        delay.delay_ms(500);
    }
}
