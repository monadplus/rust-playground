#[test]
fn lazy_static_test() {
    use once_cell::sync::OnceCell;
    use std::collections::HashMap;

    macro_rules! lazy_static {
        (@ ($($vis:tt)*) $name:ident : $ty:ty = $init:expr ; $($tt:tt)*) => {
            $($vis)* struct $name;

            impl ::std::ops::Deref for $name {
                type Target = $ty;

                fn deref(&self) -> &Self::Target {
                    static $name: OnceCell<$ty> = OnceCell::new();
                    $name.get_or_init(|| $init)
                }
            }

            lazy_static!($($tt)*);
        };
        (pub static ref $name:ident : $ty:ty = $init:expr ; $($tt:tt)*) => {
            lazy_static!{@ (pub) $name : $ty = $init ; $($tt)*}
        };
        (static ref $name:ident : $ty:ty = $init:expr ; $($tt:tt)* ) => {
            lazy_static!{@ () $name : $ty = $init ; $($tt)*}
        };
        () => {};
    }

    lazy_static! {
        pub static ref HASHMAP: HashMap<u32, &'static str> = {
            let mut m = HashMap::new();
            m.insert(0, "foo");
            m
        };
        static ref COUNT: usize = HASHMAP.len();
    }

    assert_eq!(HASHMAP.get(&0), Some(&"foo"));
}
