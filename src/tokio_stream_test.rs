#[cfg(test)]
mod tests {
    use std::{io, net::SocketAddr};

    use async_stream::{stream, try_stream};
    use tokio::{
        net::{TcpListener, TcpStream},
        pin,
    };
    use tokio_stream::{Stream, StreamExt};

    fn zero_to_three() -> impl Stream<Item = u32> {
        stream! {
            for i in 0..3 {
                yield i;
            }
        }
    }

    fn double<S: Stream<Item = u32>>(input: S) -> impl Stream<Item = u32> {
        stream! {
            for await value in input {
                yield value * 2;
            }
        }
    }

    #[allow(dead_code)]
    fn bind_and_accept(addr: SocketAddr) -> impl Stream<Item = io::Result<TcpStream>> {
        try_stream! {
            let listener = TcpListener::bind(addr).await?;

            loop {
                let (stream, addr) = listener.accept().await?;
                println!("received on {:?}", addr);
                yield stream;
            }
        }
    }

    #[tokio::test]
    async fn test_async_stream() {
        pin! {
            let s = double(zero_to_three());
        }

        while let Some(value) = s.next().await {
            println!("got {}", value);
        }
    }
}
