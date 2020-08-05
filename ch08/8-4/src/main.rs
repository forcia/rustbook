#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

//use embedded_hal::serial::Write;
//use stm32f4xx_hal::serial::{config::Config, config::StopBits, Serial};
//use stm32f4xx_hal::{prelude::*, stm32};
//
//#[entry]
//fn main() -> ! {
//    if let Some(dp) = stm32::Peripherals::take() {
//        let rcc = dp.RCC.constrain();
//        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();
//        let gpioc = dp.GPIOC.split();
//
//        let config = Config::default()
//            .baudrate(115200.bps())
//            .wordlength_8()
//            .parity_none()
//            .stopbits(StopBits::STOP1);
//
//        let pins = (
//            gpioc.pc10.into_alternate_af8(),
//            gpioc.pc11.into_alternate_af8(),
//        );
//        let mut uart4 = Serial::uart4(dp.UART4, pins, config, clocks).unwrap();
//
//        let _ = uart4.write(b'H');
//        let _ = uart4.write(b'e');
//        let _ = uart4.write(b'l');
//        let _ = uart4.write(b'l');
//        let _ = uart4.write(b'o');
//        let _ = uart4.flush();
//    }
//
//    loop {}
//}

use stm32f4xx_hal::{i2c::I2c, prelude::*, stm32};

#[entry]
fn main() -> ! {
    if let Some(dp) = stm32::Peripherals::take() {
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        let gpiob = dp.GPIOB.split();

        let pins = (
            gpiob.pb8.into_alternate_af4().set_open_drain(),
            gpiob.pb9.into_alternate_af4().set_open_drain(),
        );
        let mut i2c = I2c::i2c1(dp.I2C1, pins, 400.khz(), clocks);
        let _ = i2c.write(0, &[0]);
    }

    loop {}
}
