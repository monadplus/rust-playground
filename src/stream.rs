#[allow(dead_code)]
const URLS: [&str; 3] = [
    "https://api.ipify.org/",
    "https://www.boredapi.com/api/activity",
    "https://random.dog/woof.json",
];

#[tokio::test]
async fn stream_fetch_urls() {
    use futures::StreamExt;
    use reqwest::{Client, Error};

    let http = Client::new();

    futures::stream::iter(URLS)
        .map(|url| {
            let client = &http;
            async move {
                let resp = client.get(url).send().await?;
                let status = resp.status();
                let text = resp.text().await?;
                Ok::<_, Error>((url, status, text))
            }
        })
        .buffer_unordered(4)
        .for_each(|res| async {
            if let Ok((url, status, text)) = res {
                println!("{url} ({status}) {text}");
            }
        })
        .await;
}

#[tokio::test]
async fn iter_fetch_urls() {
    use futures::{self, future::try_join_all};
    use reqwest::{Client, Error};

    let http = &Client::new();
    let requests = URLS.iter().map(|&url| async move {
        let resp = http.get(url).send().await?;
        let status = resp.status();
        let text = resp.text().await?;
        println!("{url} ({status}) {text}");
        Ok::<_, Error>(())
    });

    try_join_all(requests).await.unwrap();
}
