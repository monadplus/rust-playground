use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    pub fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        self.item_ready.notify_one();
    }

    pub fn receive(&self) -> T {
        let mut b = self.queue.lock().unwrap();
        loop {
            if let Some(message) = b.pop_front() {
                return message;
            }
            // Unlocks the mutex while waiting.
            b = self.item_ready.wait(b).unwrap();
        }
    }
}

#[test]
fn channel1_test() {
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    thread::scope(|s| {
        let channel: Arc<Channel<u8>> = Arc::new(Channel::new());
        let arc1 = Arc::clone(&channel);
        let arc2 = Arc::clone(&channel);

        s.spawn(move || {
            arc1.send(0);
            println!("Send 0");
            thread::sleep(Duration::from_secs(1));
            println!("Send 1");
            arc1.send(1);
            println!("Send 2");
            arc1.send(2);
            thread::sleep(Duration::from_secs(1));
            println!("Send 3");
            arc1.send(3);
            // Won't work, will block
            // println!("Receiving..");
            // arc1.receive();
        });

        s.spawn(move || loop {
            if false {
                break;
            }
            let t = arc2.receive();
            println!("Received {}", t);
            if t > 2 {
                println!("Sending..");
                arc2.send(3);
                return;
            }
        });
    });
}
