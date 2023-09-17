use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, Ordering},
};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    in_use: AtomicBool,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            in_use: AtomicBool::new(false),
            ready: AtomicBool::new(false),
        }
    }

    /// Panics when trying to send more than one message.
    pub fn send(&self, message: T) {
        if self.in_use.swap(true, Ordering::Relaxed) {
            panic!("can't send more than one message!");
        }
        unsafe { (*self.message.get()).write(message) };
        self.ready.store(true, Ordering::Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Relaxed)
    }

    /// Panics if no message is available yet,
    /// or if the message was already consumed.
    ///
    /// Tip: Use `is_ready` to check first.
    pub fn receive(&self) -> T {
        if !self.ready.swap(false, Ordering::Acquire) {
            panic!("no message available!");
        }
        // Safety: We've just checked (and reset) the ready flag.
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        // Can only be called by the current owner of the channel.
        // So, there's no need to use atomic
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

#[test]
fn channel3_test() {
    let channel = Channel::new();

    let t = std::thread::current();
    std::thread::scope(|s| {
        s.spawn(|| {
            channel.send("hello world!");
            channel.send("hello world!");
            t.unpark();
        });
        while !channel.is_ready() {
            std::thread::park();
        }
        assert_eq!(channel.receive(), "hello world!");
    });
}
