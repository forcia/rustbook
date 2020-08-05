extern "C" {
    fn c_fib(n: u32) -> u32;
}

fn main() {
    unsafe {
        println!("{}", c_fib(45));
    }
}
