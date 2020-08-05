#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::debug;
use stm32f4xx_hal::{delay::Delay, prelude::*, stm32};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (stm32::Peripherals::take(), stm32::CorePeripherals::take()) {
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        let gpiod = dp.GPIOD.split();
        let mut led = gpiod.pd15.into_push_pull_output();

        let mut delay = Delay::new(cp.SYST, clocks);

        for _ in 0..5 {
            led.set_high().unwrap();
            delay.delay_ms(100_u32);

            led.set_low().unwrap();
            delay.delay_ms(100_u32);
        }
    }

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
