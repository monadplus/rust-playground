//! Source https://gist.github.com/jFransham/369a86eff00e5f280ed25121454acec1#achieving-warp-speed-with-rust

// We're redefining these here since they're private in the stdlib
#[derive(Debug, Clone, PartialEq, Eq)]
struct ParseIntError {
    kind: IntErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum IntErrorKind {
    Empty,
    InvalidDigit,
    Overflow,
    Underflow,
}

#[inline]
fn from_str_radix(input: &str, radix: usize) -> Result<usize, ParseIntError> {
    fn to_digit_ascii(ascii: u8, radix: usize) -> Result<usize, ParseIntError> {
        let decimal_digit = ascii.wrapping_sub(b'0');

        if radix > 10 && decimal_digit > 9 {
            let out = (ascii | 32).wrapping_sub(b'a') as usize;

            if out > radix - 10 {
                Err(ParseIntError {
                    kind: IntErrorKind::InvalidDigit,
                })
            } else {
                Ok(out + 10)
            }
        } else {
            let decimal_digit = decimal_digit as usize;
            if decimal_digit > radix {
                Err(ParseIntError {
                    kind: IntErrorKind::InvalidDigit,
                })
            } else {
                Ok(decimal_digit)
            }
        }
    }

    if radix > 36 {
        panic!("from_str_radix: radix is too high (maximum 36)");
    }

    let bytes = input.as_bytes();

    if bytes.len() == 0 {
        return Err(ParseIntError {
            kind: IntErrorKind::Empty,
        });
    }

    let bytes = match bytes[0] {
        b'+' => &bytes[1..],
        b'-' => {
            return Err(ParseIntError {
                kind: IntErrorKind::Underflow,
            })
        }
        _ => bytes,
    };

    let mut mul = radix;
    let mut index = bytes.len() - 1;

    let mut output = to_digit_ascii(bytes[index], radix)?;

    for &byte in bytes[..index].iter().rev() {
        let digit = to_digit_ascii(byte, radix)?;

        let next_output = output.wrapping_add(digit * mul);

        if output > next_output {
            return Err(ParseIntError {
                kind: IntErrorKind::Overflow,
            });
        }

        mul *= radix;
        output = next_output;
    }

    Ok(output)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;
//
//     #[bench]
//     fn bench_from_str(b: &mut Bencher) {
//         b.iter(|| {
//             let input = black_box("1235112512");
//             assert_eq!(from_str_radix(input, 10), Ok(1235112512));
//             let input = black_box("FFaf125A");
//             assert_eq!(from_str_radix(input, 16), Ok(0xFFaf125A));
//         });
//     }
//
//     #[bench]
//     fn bench_from_str_native(b: &mut Bencher) {
//         b.iter(|| {
//             let input = black_box("1235112512");
//             assert_eq!(usize::from_str_radix(input, 10), Ok(1235112512));
//             let input = black_box("FFaf125A");
//             assert_eq!(usize::from_str_radix(input, 16), Ok(0xFFaf125A));
//         });
//     }
//
//     #[bench]
//     fn bench_from_str_nonconstradix(b: &mut Bencher) {
//         b.iter(|| {
//             let input = black_box("1235112512");
//             let radix = black_box(10);
//             assert_eq!(from_str_radix(input, radix), Ok(1235112512));
//             let input = black_box("FFaf125A");
//             let radix = black_box(16);
//             assert_eq!(from_str_radix(input, radix), Ok(0xFFaf125A));
//         });
//     }
//
//     #[bench]
//     fn bench_from_str_native_nonconstradix(b: &mut Bencher) {
//         b.iter(|| {
//             let input = black_box("1235112512");
//             let radix = black_box(10);
//             assert_eq!(usize::from_str_radix(input, radix), Ok(1235112512));
//             let input = black_box("FFaf125A");
//             let radix = black_box(16);
//             assert_eq!(usize::from_str_radix(input, radix), Ok(0xFFaf125A));
//         });
//     }
//
//     #[bench]
//     fn bench_from_str_1char(b: &mut Bencher) {
//         b.iter(|| {
//             let input = black_box("1");
//             assert_eq!(from_str_radix(input, 10), Ok(1));
//             let input = black_box("F");
//             assert_eq!(from_str_radix(input, 16), Ok(0xF));
//         });
//     }
//
//     #[bench]
//     fn bench_from_str_native_1char(b: &mut Bencher) {
//         b.iter(|| {
//             let input = black_box("1");
//             assert_eq!(usize::from_str_radix(input, 10), Ok(1));
//             let input = black_box("F");
//             assert_eq!(usize::from_str_radix(input, 16), Ok(0xF));
//         });
//     }
// }
