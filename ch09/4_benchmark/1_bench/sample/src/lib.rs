#![feature(test)]

pub fn hash(x: u64) -> u64 {
    let mut y = x;
    for _ in 0..512 {
        y = y << 5;
        y = y ^ x;
    }
    y
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_hash(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(2);
            hash(n)
        });
    }
}
