use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicU32, Ordering},
};

pub struct RwLock<T> {
    /// The number of readers, or u32::MAX if write-locked.
    state: AtomicU32,
    /// Incremented to wake up writers.
    writer_wake_counter: AtomicU32, // New!
    value: UnsafeCell<T>,
}

// Multiples readers will have access to the data at once
unsafe impl<T> Sync for RwLock<T> where T: Send + Sync {}

impl<T> RwLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),               // Unlocked.
            writer_wake_counter: AtomicU32::new(0), // New!
            value: UnsafeCell::new(value),
        }
    }

    pub fn read(&self) -> ReadGuard<T> {
        let mut s = self.state.load(Ordering::Relaxed);
        loop {
            if s < u32::MAX {
                assert!(s < u32::MAX - 1, "too many readers");
                match self.state.compare_exchange_weak(
                    s,
                    s + 1,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => return ReadGuard { rwlock: self },
                    Err(e) => s = e,
                }
            }
            if s == u32::MAX {
                atomic_wait::wait(&self.state, u32::MAX);
                s = self.state.load(Ordering::Relaxed);
            }
        }
    }

    pub fn write(&self) -> WriteGuard<T> {
        while self
            .state
            .compare_exchange(0, u32::MAX, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            let w = self.writer_wake_counter.load(Ordering::Acquire);
            if self.state.load(Ordering::Relaxed) != 0 {
                // Wait if the RwLock is still locked, but only if
                // there have been no wake signals since we checked.
                atomic_wait::wait(&self.writer_wake_counter, w);
            }
        }
        WriteGuard { rwlock: self }
    }
}

pub struct ReadGuard<'a, T> {
    rwlock: &'a RwLock<T>,
}

impl<T> std::ops::Deref for ReadGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.rwlock.value.get() }
    }
}

impl<T> Drop for ReadGuard<'_, T> {
    fn drop(&mut self) {
        if self.rwlock.state.fetch_sub(1, Ordering::Release) == 1 {
            self.rwlock
                .writer_wake_counter
                .fetch_add(1, Ordering::Release); // New!
            atomic_wait::wake_one(&self.rwlock.writer_wake_counter); // Changed!
        }
    }
}

// Behave like `&mut T`
pub struct WriteGuard<'a, T> {
    rwlock: &'a RwLock<T>,
}

impl<T> std::ops::Deref for WriteGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.rwlock.value.get() }
    }
}

impl<T> std::ops::DerefMut for WriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.rwlock.value.get() }
    }
}

impl<T> Drop for WriteGuard<'_, T> {
    fn drop(&mut self) {
        self.rwlock.state.store(0, Ordering::Release);
        self.rwlock
            .writer_wake_counter
            .fetch_add(1, Ordering::Release); // New!
        atomic_wait::wake_one(&self.rwlock.writer_wake_counter); // New!
        atomic_wait::wake_all(&self.rwlock.state);
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Instant};

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn rwlock_works() {
        let m = RwLock::new(0);
        std::hint::black_box(&m);
        let start = Instant::now();
        thread::scope(|s| {
            for _ in 0..4 {
                s.spawn(|| {
                    for _ in 0..5_000_000 {
                        *m.write() += 1;
                    }
                });
            }
        });
        let duration = start.elapsed();
        println!("locked {} times in {:?}", *m.read(), duration);
        assert_eq!(*m.read(), 5_000_000 * 4);
    }
}