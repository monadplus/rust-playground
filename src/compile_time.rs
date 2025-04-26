// Computing at compile time
const fn fibonacci_compile_time(n: u32) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        n => {
            let mut a = 0;
            let mut b = 1;
            let mut i = 2;
            while i <= n {
                let temp = a + b;
                a = b;
                b = temp;
                i += 1;
            }
            b
        }
    }
}

const FIB_100: u128 = fibonacci_compile_time(100);
