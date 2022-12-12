// This test fails on 2018
// https://play.rust-lang.org/?version=stable&mode=release&edition=2018&gist=c17d299cacd626c572def0c4262aed69
#[test]
fn maybe_uninit_test() {
    use std::mem;

    fn always_returns_true(x: u8) -> bool {
        x < 120 || x == 120 || x > 120
    }
    let x: u8 = unsafe { mem::MaybeUninit::uninit().assume_init() };
    assert!(always_returns_true(x));
}
