use better_default_derive::Default;

#[derive(Debug, PartialEq, Eq, Default)]
enum Either<L, R> {
    Left(L),
    #[default]
    Right(R),
}

#[test]
fn better_default_derive_test() {
    let either: Either<String, u8> = Either::default();
    assert_eq!(either, Either::Right(u8::default()));
}
