use std::error::Error;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tower::Layer;
use tower::Service;

pub struct Timeout<T> {
    inner: T,
    timeout: Duration,
}

impl<T> Timeout<T> {
    pub fn new(inner: T, timeout: Duration) -> Timeout<T> {
        Timeout { inner, timeout }
    }
}

#[derive(Debug)]
pub struct Expired;

impl fmt::Display for Expired {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "expired")
    }
}

impl Error for Expired {}

impl<T, Request> Service<Request> for Timeout<T>
where
    T: Service<Request>,
    T::Future: 'static,
    T::Error: Into<Box<dyn Error + Send + Sync>> + 'static,
    T::Response: 'static,
{
    type Response = T::Response;

    type Error = Box<dyn Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let timeout = tokio::time::sleep(self.timeout);

        let fut = self.inner.call(req);

        let f = async move {
            tokio::select! {
                res = fut => {
                    res.map_err(|err| err.into())
                },
                _ = timeout => {
                    Err(Box::new(Expired) as Box<dyn Error + Send + Sync>)
                },
            }
        };

        Box::pin(f)
    }
}

pub struct TimeoutLayer(Duration);

impl TimeoutLayer {
    pub fn new(delay: Duration) -> Self {
        TimeoutLayer(delay)
    }
}

impl<S> Layer<S> for TimeoutLayer {
    type Service = Timeout<S>;

    fn layer(&self, service: S) -> Timeout<S> {
        Timeout::new(service, self.0)
    }
}
