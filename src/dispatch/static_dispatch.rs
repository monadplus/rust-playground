//! Source: https://medium.com/digitalfrontiers/rust-dynamic-dispatching-deep-dive-236a5896e49b
use std::time::SystemTime;

struct PositiveBackend;

impl PositiveBackend {
    #[inline(never)]
    fn compute(&self, number: u64) -> u64 {
        number + 1
    }
}

// # https://github.com/pacak/cargo-show-asm
// $ cargo asm --bin=playground --release "playground::main
//
// https://godbolt.org/z/9qbe4Yn7n
fn main() {
    let backend = Box::new(PositiveBackend);
    let mut res = 0 as u64;
    let start_time = SystemTime::now();
    let total = 20_000_000 as u64;

    // our main loop
    for i in 0..total {
        res += backend.compute(i) + res;
    }

    println!("Result: {}", res);
    println!("Elapsed_ms: {}", start_time.elapsed().unwrap().as_millis());
}
