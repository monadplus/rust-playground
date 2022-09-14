use std::ptr::NonNull;

#[derive(Clone, Debug)]
struct Nod {
    name: String,
}

#[derive(Clone)]
struct Ptr {
    ptr: NonNull<Nod>,
}

impl Drop for Ptr {
    fn drop(&mut self) {
        let b = unsafe { Box::from_raw(self.ptr.as_ptr()) };
        println!("Dropping Ptr");
        drop(b);
    }
}

impl Ptr {
    fn new(name: &str) -> Self {
        let x = Box::new(Nod {
            name: name.to_string(),
        });
        let x_ptr = Box::into_raw(x);
        let ptr = unsafe { NonNull::new_unchecked(x_ptr) };
        Ptr { ptr }
    }
}

#[test]
fn test_ptr() {
    let x = Ptr::new("Hello");
    unsafe {
        println!("x: {:?}", *(x.ptr.as_ptr()));
    }
}
