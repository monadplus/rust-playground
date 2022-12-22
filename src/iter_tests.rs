#[test]
fn into_iter_test() {
    struct Container<T>(Vec<T>);

    impl<T> IntoIterator for Container<T> {
        type Item = <Vec<T> as IntoIterator>::Item;

        type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }

    let container = Container((1u8..100).collect());
    for v in container {
        println!("v: {v}")
    }
}

#[test]
fn iter_to_array_test() {
    let xs: std::ops::RangeInclusive<u128> = 1..=10;
    println!("product: {}", xs.product::<u128>());

    // Allocates heap for temporal Vec
    let xs: [u128; 10] = (1..=10)
        .collect::<Vec<_>>()
        .try_into()
        .expect("wrong iter size");
    println!("product: {}", xs.iter().product::<u128>());

    // Heap allocated array
    let xs: Box<[u128; 10]> = (1..=10)
        .collect::<Box<[u128]>>()
        .try_into()
        .expect("wrong iter size");
    println!("product: {}", xs.iter().product::<u128>());

    // Stack only
    let mut xs: [u128; 10] = [0; 10];
    for (elem, val) in xs.iter_mut().zip(1..=10) {
        *elem = val as u128;
    }
    println!("product: {}", xs.iter().product::<u128>());
}
