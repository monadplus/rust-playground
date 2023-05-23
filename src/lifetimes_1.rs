use async_trait::async_trait;
use futures::{future::BoxFuture, FutureExt};

struct A;

impl A {
    fn nothing(&mut self) {}
}

// Job Processor
#[async_trait]
trait Other<'a> {
    async fn other(&self);
}

#[async_trait]
trait Base<'a> {
    fn base<'life0, 'short>(
        &'life0 self,
    ) -> Option<Box<dyn Fn(&'short mut A) -> BoxFuture<'short, ()> + Send + Sync + 'short>>
    where
        'life0: 'short;
}

#[async_trait]
trait Super<'a> {
    async fn top(&self, a: &'async_trait A);
}

#[async_trait]
impl<'a, T: Super<'a> + Send + Sync> Base<'a> for T {
    fn base<'life0, 'short>(
        &'life0 self,
    ) -> Option<Box<dyn Fn(&'short mut A) -> BoxFuture<'short, ()> + Send + Sync + 'short>>
    where
        'life0: 'short,
    {
        Some(Box::new(|a: &mut A| self.top(a).boxed()))
    }
}

#[async_trait]
impl<'a, T: Base<'a> + Send + Sync> Other<'a> for T {
    async fn other(&self) {
        let mut a = A;
        let x = if let Some(base) = self.base() {
            let r = base(&mut a).await;
            r
        };
        a.nothing();
    }
}
