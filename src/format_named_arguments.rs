#[test]
fn format_named_arguments_test() {
    let a: u8 = 1;
    let b: u8 = 2;
    println!("a: {a}\nb: {b}\na+b: {ab}", ab = a + b);
}
