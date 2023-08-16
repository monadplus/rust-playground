#[test]
fn acquire_release_test() {
    use std::sync::atomic::AtomicPtr;
    use std::sync::atomic::Ordering;
    use std::thread;
    use std::{sync::atomic::*, time::Duration};

    struct Data;

    static T: AtomicBool = AtomicBool::new(false);

    fn get_data() -> &'static Data {
        while !T.load(Ordering::Acquire) {}

        static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

        let mut p = PTR.load(Ordering::Acquire);

        if p.is_null() {
            // println!("Thread {:?}: init Data", thread::current().id());
            p = Box::into_raw(Box::new(Data));
            if let Err(e) = PTR.compare_exchange(
                std::ptr::null_mut(),
                p,
                Ordering::Release,
                Ordering::Acquire,
            ) {
                // Safety: p comes from Box::into_raw right above,
                // and wasn't shared with any other thread.
                drop(unsafe { Box::from_raw(p) });
                p = e;
                println!("Thread {:?}: dropping box", thread::current().id())
            }
        }

        // println!("Thread {:?}: end", thread::current().id());

        // Safety: p is not null and points to a properly initialized value.
        unsafe { &*p }
    }

    for _ in 1..=64 {
        thread::spawn(get_data);
    }

    T.store(true, Ordering::Release);

    thread::sleep(Duration::from_secs(1));
}
