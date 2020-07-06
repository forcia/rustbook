#![feature(test)]

extern crate test;

use sample::hash;
use test::Bencher;

#[bench]
fn bench_hash(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(2);
        hash(n)
    });
}
