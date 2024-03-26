#[test]
fn test_gat() {
    /*
    trait Iterable {
        type Item;
        type Iter: Iterator<Item = Self::Item>;
        fn iter<'c>(&'c self) -> Self::Iter;
    }

    impl<T> Iterable for [T] {
        type Item = &'hmm T;
        //           ^^^^ what lifetime to use here?

        type Iter = Iter<'hmm, T>;
        //               ^^^^ what lifetime to use here?

        fn iter<'c>(&'c self) -> Self::Iter {
            //       ^^ THIS is the lifetime we want, but it's not in scope!
            Iter { data: self }
        }
    }
    */

    trait Iterable {
        // Type of item yielded up; will be a reference into `Self`.
        type Item<'collection>
        where
            Self: 'collection;

        // Type of iterator we return. Will return `Self::Item` elements.
        type Iterator<'collection>: Iterator<Item = Self::Item<'collection>>
        where
            Self: 'collection;

        fn iter<'c>(&'c self) -> Self::Iterator<'c>;
        //           ^^                         ^^
        //
        // Returns a `Self::Iter` derived from `self`.
    }

    struct Iter<'c, T> {
        data: &'c [T],
    }

    impl<'c, T> Iterator for Iter<'c, T> {
        type Item = &'c T;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some((prefix_elem, suffix)) = self.data.split_first() {
                self.data = suffix;
                Some(prefix_elem)
            } else {
                None
            }
        }
    }

    impl<T> Iterable for [T] {
        type Item<'c> = &'c T
    where
        T: 'c;

        type Iterator<'c> = Iter<'c, T>
    where
        T: 'c;

        fn iter<'c>(&'c self) -> Self::Iterator<'c> {
            Iter { data: self }
        }
    }

    impl<T> Iterable for Vec<T> {
        type Item<'c> = &'c T where T: 'c;

        type Iterator<'c> = Iter<'c, T> where T: 'c;

        fn iter<'c>(&'c self) -> Self::Iterator<'c> {
            Iter { data: self }
        }
    }

    fn count_twice<I: Iterable + ?Sized>(collection: &I) -> usize {
        let mut count = 0;

        for _ in collection.iter() {
            count += 1;
        }

        for _ in collection.iter() {
            count += 1;
        }

        count
    }

    let x: &[usize] = &[1, 2, 3];
    let c = count_twice(x);
    assert_eq!(c, 6);

    let c = count_twice(&vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(c, 12);

    fn into_vec<T>(iterable: &(impl for<'a> Iterable<Item<'a> = &'a T> + ?Sized)) -> Vec<T>
    where
        T: Clone,
    {
        let mut out = vec![];
        for elem in iterable.iter() {
            out.push(elem.clone());
        }
        out
    }

    let xs: [u8; 4] = [0; 4];
    let _vs = into_vec(&xs[..]);

    fn first<'i, T>(iterable: &'i impl Iterable<Item<'i> = &'i T>) -> Option<&'i T> {
        iterable.iter().next()
    }

    fn sendable_items<I>(_iterable: &I)
    where
        I: Iterable,
        for<'a> I::Item<'a>: Send,
    {
    }
}
