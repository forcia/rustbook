#![feature(test)]

extern crate test;

use mandelbrot::generate_mandelbrot_set;
use test::Bencher;

#[bench]
fn bench_generate_mandelbrot_set(b: &mut Bencher) {
    b.iter(|| {
        let canvas_w = test::black_box(1200);
        let canvas_h = test::black_box(1200);
        let x_min = test::black_box(-1.5);
        let x_max = test::black_box(0.5);
        let y_min = test::black_box(-1.0);
        let y_max = test::black_box(1.0);
        let max_iter = test::black_box(64);
        generate_mandelbrot_set(canvas_w, canvas_h, x_min, x_max, y_min, y_max, max_iter);
    })
}
