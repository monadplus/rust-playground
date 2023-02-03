use std::collections::HashMap;
use tokio::sync::RwLock;

#[ignore]
#[tokio::test]
async fn match_causes_a_deadlock() {
    let lock: RwLock<HashMap<u8, u8>> = RwLock::default();
    let key = 0;
    match lock.read().await.get(&key) {
        None => {
            lock.write().await.insert(key, 1);
        }
        Some(_) => {}
    };
    println!("Not locked");
}
