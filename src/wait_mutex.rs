use std::future::Future;
use std::pin::Pin;
use std::task::Poll;

use futures::future::BoxFuture;
use futures::FutureExt;
use tokio::sync::OwnedMutexGuard;

use std::sync::Arc;

/*
Implement std::future::Future for Test:

struct Test {
    handle: tokio::task::JoinHandle<()>,
    is_ready: Arc<tokio::sync::Mutex<bool>>,
}
*/

struct Test {
    pub handle: tokio::task::JoinHandle<()>,
    pub is_ready: Arc<tokio::sync::Mutex<bool>>,
    lock: Option<BoxFuture<'static, OwnedMutexGuard<bool>>>,
    pub waker: Arc<std::sync::Mutex<Option<std::task::Waker>>>,
}

/*
The Test example is contrived.
You'd rather use something like

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}
*/

impl Test {
    #[allow(dead_code)]
    pub fn new(
        handle: tokio::task::JoinHandle<()>,
        is_ready: Arc<tokio::sync::Mutex<bool>>,
    ) -> Self {
        Self {
            handle,
            is_ready,
            lock: None,
            waker: Arc::new(std::sync::Mutex::new(None)),
        }
    }
}

impl Future for Test {
    type Output = Result<(), tokio::task::JoinError>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        println!("poll called");
        let this = self.get_mut();
        let previous_lock = this.lock.take();
        let mut current_lock = match previous_lock {
            None => {
                println!("new lock");
                Box::pin(this.is_ready.clone().lock_owned())
            }
            Some(lock) => {
                println!("re-using lock");
                lock
            }
        };

        let Poll::Ready(guard) = current_lock.poll_unpin(cx) else {
            println!("lock not ready");
            this.lock = Some(current_lock);
            return Poll::Pending;
        };

        if !*guard {
            println!("is_ready not ready");
            let mut lock = this.waker.lock().unwrap();
            *lock = Some(cx.waker().clone());
            return Poll::Pending;
        }

        println!("waiting handle");
        this.handle.poll_unpin(cx)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    async fn __wait_works(
        task_duration: Duration,
        is_ready_duration: Duration,
        wait_is_ready: bool,
    ) {
        async fn foo(task_duration: Duration) {
            tokio::time::sleep(task_duration).await
        }

        let handle = tokio::spawn(foo(task_duration));
        let mutex = Arc::new(tokio::sync::Mutex::new(false));
        let test = Test::new(handle, mutex.clone());
        let waker_clone = test.waker.clone();

        let handle = tokio::spawn(async move {
            tokio::time::sleep(is_ready_duration).await;
            println!("is_ready = true");
            let mut lock = mutex.lock().await;
            *lock = true;
            if let Some(waker) = waker_clone.lock().unwrap().take() {
                waker.wake();
            }
        });
        if wait_is_ready {
            handle.await.unwrap();
        }

        let start = tokio::time::Instant::now();
        let res = test.await;
        println!("elapsed: {}ms", start.elapsed().as_millis());

        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn wait_works() {
        let a = vec![Duration::from_millis(100), Duration::from_millis(500)];
        let b = vec![false, true];
        let product: Vec<(Duration, Duration, bool)> =
            itertools::iproduct!(a.clone(), a, b).collect();
        for (a, b, c) in product {
            println!(
                "Scenario: task duration = {a:?}, is_ready duration = {b:?}, wait is ready = {c}"
            );
            __wait_works(a, b, c).await;
        }
    }
}
