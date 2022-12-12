#![deny(unused_must_use)]

// More on https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-must_use-attribute

#[test]
fn must_use_attribute_test() {
    struct Foo {
        a: i32,
    }

    impl Foo {
        #[must_use]
        fn get_a(self) -> i32 {
            self.a
        }
    }

    let foo = Foo { a: 0i32 };
    let a = foo.get_a();
    assert_eq!(a, 0i32);
}

