#![no_main]
use libfuzzer_sys::fuzz_target;
use sum::sum_wrapping;

fuzz_target!(|data: &[u8]| {
    sum_wrapping(data);
});
