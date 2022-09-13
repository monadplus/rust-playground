use std::{marker::PhantomPinned, pin::Pin};

use futures::pin_mut;

#[derive(Debug)]
struct T1 {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}
impl T1 {
    fn new(a: &str) -> Pin<Box<Self>> {
        let t = T1 {
            a: a.to_string(),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut t = Box::pin(t);
        let self_ptr: *const String = &t.a;
        unsafe {
            t.as_mut().get_unchecked_mut().b = self_ptr;
        }
        t
    }

    fn a(self: Pin<&Self>) -> &str {
        &(self.get_ref().a)
    }

    fn b(self: Pin<&Self>) -> &str {
        unsafe { &*(self.b) }
    }
}

#[test]
fn test_t1() {
    let mut t = T1::new("hello");
    assert_eq!(t.as_ref().a(), "hello");
    assert_eq!(t.as_ref().b(), t.as_ref().a());
}

#[derive(Debug)]
struct T2 {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}
impl T2 {
    fn new(a: &str) -> Self {
        T2 {
            a: a.to_string(),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        unsafe { self.get_unchecked_mut().b = self_ptr }
    }

    fn a(self: Pin<&Self>) -> &str {
        &(self.get_ref().a)
    }

    fn b(self: Pin<&Self>) -> &str {
        unsafe { &*(self.b) }
    }
}

#[test]
fn test_t2() {
    let t = T2::new("hello");
    pin_mut!(t);
    t.as_mut().init();
    assert_eq!(t.as_ref().a(), "hello");
    assert_eq!(t.as_ref().b(), t.as_ref().a());
}
