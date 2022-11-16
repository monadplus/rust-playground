use futures::{stream::FuturesUnordered, StreamExt};

async fn do_something(i: u8) -> String {
    format! {"Future {i:#b}"}
}

#[tokio::main]
async fn main() {
    let v = std::sync::Arc::new(tokio::sync::Mutex::new(vec![]));
    (0..100)
        .map(|i| tokio::spawn(do_something(i)))
        .collect::<FuturesUnordered<_>>()
        .for_each(|r| {
            let v = std::sync::Arc::clone(&v);
            async move {
                let mut lock = v.lock().await;
                let str = r.unwrap();
                lock.push(str);
            }
        })
        .await;
    println!("{v:?}");
}
