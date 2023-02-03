// Source: https://orlp.net/blog/ordering-numbers/#generalizing

// DONT
// fn is_less_eq(x: i64, y: f64) -> bool {
//     x as f64 <= y
// }

// let x: i64 = 1 << 58;
// let y: f64 = x as f64; // 2^58, exactly representable.
// println!("{x} <= {y}: {}", x as f64 <= y);
// 288230376151711744 <= 288230376151711740: true

fn is_greater_eq(x: i64, y: f64) -> bool {
    if y.is_nan() {
        return false;
    }
    if y >= 9223372036854775808.0 {
        // 2^63
        false // y is always bigger.
    } else if y >= -9223372036854775808.0 {
        // -2^63
        x >= y.ceil() as i64 // y is in [-2^63, 2^63)
    } else {
        true // y is always smaller.
    }
}

fn is_less(x: i64, y: f64) -> bool {
    if y.is_nan() {
        return false;
    }
    if y >= 9223372036854775808.0 {
        // 2^63
        true
    } else if y >= -9223372036854775808.0 {
        // -2^63
        let yf = y.floor(); // y is in [-2^63, 2^63)
        let yfi = yf as i64;
        x < yfi || x == yfi && yf < y
    } else {
        false
    }
}
