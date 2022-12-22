#[test]
fn const_generic_test() {
    fn sum_of<const N: usize>() {
        let mut arr: [u128; N] = [0; N];
        for (elem, val) in arr.iter_mut().zip(1..=N) {
            *elem = val as u128;
        }
        println!("sum: {}", arr.iter().sum::<u128>());
    }

    const NINETY_NINE: usize = 99;
    sum_of::<{ NINETY_NINE + 1 }>();
}

#[test]
fn const_generic_expr_test() {
    struct A<const N: usize> {
        elems: Vec<u8>,
    }

    // impl<const N: usize> A<N> {
    //     fn push(mut self, v: u8) -> A<{ N - 1 }> {
    //         self.elems.push(v);
    //         A { elems: self.elems }
    //     }
    // }

    impl A<0> {
        fn end(self) -> Vec<u8> {
            self.elems
        }
    }

    impl A<1> {
        fn push(mut self, v: u8) -> A<0> {
            self.elems.push(v);
            A { elems: self.elems }
        }
    }

    impl A<2> {
        fn push(mut self, v: u8) -> A<1> {
            self.elems.push(v);
            A { elems: self.elems }
        }
    }

    let a: A<2> = A { elems: Vec::new() };
    a.push(0).push(1).end();
}

#[test]
fn const_generic_associated_type() {
    struct Variable<const N: usize>;
    struct Constant<const N: usize>;

    trait Tensor {
        const N: usize;

        fn get_dim(&self) -> usize {
            Self::N
        }
    }
    trait ConvertTo<const N: usize> {
        type To;

        fn convert(&self) -> Self::To;
    }

    impl<const N: usize> Tensor for Variable<N> {
        const N: usize = N;
    }
    impl<const N: usize> Tensor for Constant<N> {
        const N: usize = N;
    }

    impl<const N: usize, const M: usize> ConvertTo<M> for Variable<N> {
        type To = Constant<M>;

        fn convert(&self) -> Self::To {
            Constant::<M>
        }
    }

    impl<const N: usize, const M: usize> ConvertTo<M> for Constant<N> {
        type To = Variable<M>;

        fn convert(&self) -> Self::To {
            Variable::<M>
        }
    }

    fn convert_plus_one<X>(x: X) -> <X as ConvertTo<{ <X as Tensor>::N + 1 }>>::To
    where
        X: Tensor + ConvertTo<{ <X as Tensor>::N + 1 }>,
    {
        x.convert()
    }

    let x = Constant::<3>;
    let y = convert_plus_one(x);
    assert_eq!(y.get_dim(), 4);
}
