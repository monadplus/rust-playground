#[derive(Debug, Clone)]
struct Counter {
    counter: u16,
}

impl Counter {
    fn add_one(&mut self) {
        self.counter += 1;
    }
}

#[tokio::test]
async fn arc_swap_manually() {
    use futures::{stream::FuturesUnordered, StreamExt};
    use once_cell::sync::Lazy;
    use std::{sync::Arc, time::Duration};
    use tokio::{sync::RwLock, time::sleep};

    static COUNTER: Lazy<RwLock<Arc<Counter>>> =
        Lazy::new(|| RwLock::new(Arc::new(Counter { counter: 0 })));

    tokio::spawn(async move {
        sleep(Duration::from_millis(10)).await;
        let mut wlock = COUNTER.write().await;
        let mut new_counter: Counter = (**wlock).clone();
        new_counter.add_one();
        *wlock = Arc::new(new_counter);
    });
    (1..100)
        .into_iter()
        .map(|i| {
            tokio::spawn(async move {
                sleep(Duration::from_millis(i)).await;
                let rlock = COUNTER.read().await;
                println!("Counter {} has value {:?}", i, rlock.counter)
            })
        })
        .collect::<FuturesUnordered<_>>()
        .for_each(|_| async {})
        .await;
}

#[tokio::test]
async fn arc_swap() {
    use arc_swap::ArcSwap;
    use futures::{stream::FuturesUnordered, StreamExt};
    use once_cell::sync::Lazy;
    use std::{sync::Arc, time::Duration};
    use tokio::time::sleep;

    static COUNTER: Lazy<ArcSwap<Counter>> =
        Lazy::new(|| ArcSwap::from_pointee(Counter { counter: 0 }));

    tokio::spawn(async move {
        sleep(Duration::from_millis(10)).await;
        let mut new_counter: Counter = (**COUNTER.load()).clone();
        new_counter.add_one();
        COUNTER.swap(Arc::new(new_counter));
    });
    (1..100)
        .into_iter()
        .map(|i| {
            tokio::spawn(async move {
                sleep(Duration::from_millis(i)).await;
                let counter = COUNTER.load();
                println!("Counter {} has value {:?}", i, counter.counter)
            })
        })
        .collect::<FuturesUnordered<_>>()
        .for_each(|_| async {})
        .await;
}
