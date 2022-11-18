use std::borrow::Cow;
use std::fmt::Display;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;

static COUNTER: AtomicU8 = AtomicU8::new(0);

struct Cloned<T>(T);

impl<T> Display for Cloned<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Clone for Cloned<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        COUNTER.fetch_add(1, Ordering::SeqCst);
        Self(self.0.clone())
    }
}

#[test]
fn cow_test() {
    let xs: Vec<Cloned<u8>> = [b'a', b'b', b'c'].into_iter().map(|x| Cloned(x)).collect();
    let mut cow: Cow<[Cloned<u8>]> = Cow::from(&xs[..]);
    for (i, v) in cow.into_iter().enumerate() {
        println!("Cow[{i}]: {}", v);
    }
    assert_eq!(COUNTER.load(Ordering::SeqCst), 0);

    let vs = cow.to_mut();
    assert_eq!(COUNTER.load(Ordering::SeqCst), 3);
    vs.push(Cloned(b'd'));
}
