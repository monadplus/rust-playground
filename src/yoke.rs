#[test]
fn yoke_test() {
    use std::{borrow::Cow, rc::Rc};
    use yoke::Yoke;

    // https://docs.rs/yoke/latest/yoke/struct.Yoke.html
    //
    // Allows you to Cow<'a, str> -> Yoke<Cow<'static, str>, ...>
    // to move the object around without caring about lifetimes.
    //
    //            Yoke<Y, C>
    //
    // Y = "yokeable": contains references. E.g. Cow, ZeroVec.
    // C = "cart":     referenced by Y.     E.g. Box,Rc,Arc,Option
    //                 Is only used to guarantee that Y's references remain valid as long as yoke
    //                 remains in memory
    //
    // Constructor: attach_to_chart(): https://docs.rs/yoke/latest/yoke/struct.Yoke.html#method.attach_to_cart
    //
    // Short live reference to Y: .get(): https://docs.rs/yoke/latest/yoke/struct.Yoke.html#method.get
    // -  Yoke<Cow<'static, str>, _> -> &'a Cow<'a, str>

    fn load_from_cache() -> Rc<[u8]> {
        Rc::new([104, 101, 108, 108, 111])
    }

    fn load_object() -> Yoke<Cow<'static, str>, Rc<[u8]>> {
        let rc: Rc<[u8]> = load_from_cache();
        Yoke::<Cow<'static, str>, Rc<[u8]>>::attach_to_cart(rc, |data: &[u8]| {
            let cow: Cow<'_, str> = Cow::Borrowed(std::str::from_utf8(data).unwrap());
            cow
        })
    }

    let yoke = load_object();
    assert_eq!(&**yoke.get(), "hello");
    assert!(matches!(yoke.get(), &Cow::Borrowed(_)));
}
