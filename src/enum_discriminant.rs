#[derive(Debug, Copy, Clone, derive_more::Display)]
enum Foo {
    Zero = 0,
    TwoHundredFiftyFive = 255,
    Overflow = 256,
}

#[test]
fn enum_discriminant_test() {
    assert_eq!(Foo::Zero as u8, 0);
    assert_eq!(Foo::TwoHundredFiftyFive as u8, 255);
    // overflows without panicking
    assert_eq!(Foo::Overflow as u8, 0);
}
