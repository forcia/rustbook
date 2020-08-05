#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate panic_halt;

use alloc::alloc::{GlobalAlloc, Layout};
use alloc::vec::Vec;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        0x2000_0100 as *mut u8
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static A: MyAllocator = MyAllocator;

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    let mut x = Vec::new();
    x.push(123);
    x.push(456);

    let _ = hprintln!("{:?}", x);

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
