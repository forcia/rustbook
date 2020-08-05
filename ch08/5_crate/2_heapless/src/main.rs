#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use heapless::consts::U2;
use heapless::Vec;

#[entry]
fn main() -> ! {
    let mut x = Vec::<_, U2>::new();
    let _ = x.push(123);
    let _ = x.push(456);
    let _ = x.push(789);

    let _ = hprintln!("{:?}", x);

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
