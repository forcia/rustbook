#![no_main]
use libfuzzer_sys::fuzz_target;
use sum::sum;

fuzz_target!(|data: &[u8]| {
    sum(data);
});
