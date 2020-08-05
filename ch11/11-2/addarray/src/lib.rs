#[no_mangle]
pub extern "C" fn add_array(n: u64, x: u64) -> u64 {
    let mut a = vec![0u64; n as usize];
    for _ in 0..x {
        for i in 0..n as usize {
            a[i] += 1;
        }
    }
    a.iter().sum()
}
