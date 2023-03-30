use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

struct Unmovable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}

impl Unmovable {
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovable {
            data,
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);
        let slice = NonNull::from(&boxed.data);
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        boxed
    }
}

#[test]
fn pin_box_test() {
    let unmoved = Unmovable::new("hello".to_string());

    let mut still_unmoved = unmoved;
    assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));

    let mut new_unmoved = Unmovable::new("world".to_string());
    std::mem::swap(&mut still_unmoved, &mut new_unmoved);
    assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));
    assert_eq!(new_unmoved.slice, NonNull::from(&new_unmoved.data));
}
