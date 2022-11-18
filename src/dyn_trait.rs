trait Foo {}

impl<T> Foo for T {}

#[tokio::test]
async fn borrow_dyn_test() {
    struct C<'a> {
        c: Vec<&'a dyn Foo>,
    }

    C {
        c: vec![&0u8, &"hello"],
    };
}

#[tokio::test]
async fn box_dyn_test() {
    struct C {
        c: Vec<Box<dyn Foo>>,
    }

    C {
        c: vec![Box::new(0u8), Box::new("hello")],
    };
}
