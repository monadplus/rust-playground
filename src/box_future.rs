#[tokio::test]
async fn store_box_futures() {
    use futures::future::{join_all, BoxFuture, Future};

    struct Store<'a, T> {
        futs: Vec<BoxFuture<'a, T>>,
    }

    impl<'a, T> Store<'a, T> {
        fn add<F, Fut>(&mut self, fut: Fut)
        where
            Fut: Future<Output = T> + Send + Sync + 'a,
        {
            self.futs.push(Box::pin(fut));
        }

        async fn run(self) -> Vec<T> {
            join_all(self.futs).await
        }
    }

    let mut s = Store { futs: Vec::new() };
    s.add::<u8, _>(async { 0_u8 });
    s.add::<u8, _>(async { 1_u8 });
    assert_eq!(s.run().await, vec!(0_u8, 1_u8));
}

#[tokio::test]
async fn store_fn_box_futures() {
    use futures::future::{join_all, BoxFuture, Future};

    struct Store {
        fns: Vec<Box<dyn Fn() -> BoxFuture<'static, ()>>>,
    }

    impl Store {
        fn add<F, Fut>(&mut self, func_ptr: F)
        where
            F: Fn() -> Fut + 'static,
            Fut: Future<Output = ()> + Send + Sync + 'static,
        {
            self.fns.push(Box::new(move || Box::pin(func_ptr())));
        }

        async fn run(&self) {
            let mut futures = Vec::new();
            for f in &self.fns {
                futures.push(f());
            }
            join_all(futures).await;
        }
    }

    async fn f1() -> () {
        println!("Hello");
    }
    async fn f2() -> () {
        println!("Goodbye");
    }

    let mut s = Store { fns: Vec::new() };
    s.add(f1);
    s.add(f2);
    s.run().await;
}
