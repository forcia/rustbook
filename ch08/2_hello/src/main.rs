#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    let _ = hprintln!("Hello, world!");

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
