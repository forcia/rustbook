fn add_array(n: u64, x: u64) -> u64 {
    let mut a = vec![0u64; n as usize];
    for _ in 0..x {
        for i in 0..n as usize {
            a[i] += 1;
        }
    }
    a.iter().sum()
}

use std::env;
fn main() {
    let args: Vec<_> = env::args().collect();
    let n = args[1].parse::<u64>().unwrap();
    let x = args[2].parse::<u64>().unwrap();
    println!("{}", add_array(n, x));
}
