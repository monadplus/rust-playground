# Memory ordering

- In a thread, instructions happen in order.
- Between thread, out-of-order

Example:

```rust
static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn a() {
    X.store(10, Relaxed);
    Y.store(20, Relaxed);
}

fn b() {
    let y = Y.load(Relaxed);
    let x = X.load(Relaxed);
    println!("{x} {y}");
}
```

- `0 20` is a valid output.

## Spawn and joining

- Spawn and joining createsa happen-before relationship.
  This assert will never fai.
  In `f`, `X.store(2, Relaxed);` may have or not happened, but `X.store(1, Relaxed);` must have happened.

```rust
static X: AtomicI32 = AtomicI32::new(0);

fn main() {
    X.store(1, Relaxed);
    let t = thread::spawn(f);
    X.store(2, Relaxed);
    t.join().unwrap();
    X.store(3, Relaxed);
}

fn f() {
    let x = X.load(Relaxed);
    assert!(x == 1 || x == 2);
}
```

## Relaxed

Relaxed only guarantees **total modification order of each individual atomic varialbe**.

```rust
static X: AtomicI32 = AtomicI32::new(0);

fn a() {
    X.fetch_add(5, Relaxed);
    X.fetch_add(10, Relaxed);
}

fn b() {
    let a = X.load(Relaxed);
    let b = X.load(Relaxed);
    let c = X.load(Relaxed);
    let d = X.load(Relaxed);
    println!("{a} {b} {c} {d}");
}
```

- Valid transition: `0 -> 5 -> 15`
- `0 0 0 0`, `0 0 5 15`, `0 15 15 15` are valid outputs
  - The output `0 0 10 15` or `0 5 0 15` cannot happen
- The order of modification of `X` on the same thread is 'ordered'.
- And All threads see `X` modifications in the same order

For example, if we divide `a` into `a1` and `a2`:

```rust
fn a1() {
    X.fetch_add(5, Relaxed);
}

fn a2() {
    X.fetch_add(10, Relaxed);
}
```

- Valid transition: `0 -> 5 -> 15` or `0 -> 10 -> 15`
  - Now the output `0 0 10 15` is valid.

## Release and Acquire Ordering

- `Release` for store
- `Acquire` for load
- store and before happens before the load and everything after it

In this example, with `Relaxed`, the main thread could print `0` because
the main thread could observe `READY.store(true, Release)` before `DATA.store(123, Relaxed)`.

```rust
static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        unsafe { DATA = 123 };
        READY.store(true, Release); // Everything from before this store ..
    });
    while !READY.load(Acquire) { // .. is visible after this loads `true`.
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    // Safety: Nothing is mutating DATA, because READY is set.
    println!("{}", unsafe { DATA });
}
```

```rust
static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    if LOCKED.compare_exchange(false, true, Acquire, Relaxed).is_ok() {
        // Safety: We hold the exclusive lock, so nothing else is accessing DATA.
        unsafe { DATA.push('!') };
        LOCKED.store(false, Release);
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });
}
```

```rust
use std::sync::atomic::AtomicPtr;

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        // Release to make sure PTR is set with properly initialized p data.
        // If Relaxed, thread could see PTR set with uninit p
        if let Err(e) = PTR.compare_exchange(
            std::ptr::null_mut(), p, Release, Acquire
        ) {
            // We must guarantee that p = .. happens before the failure case
            // This is why compare_exhange is Release and this branch is Acquire
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }

    unsafe { &*p }
}
```

## Sequentially Consistent Ordering

Globally consistent order of operations. Hardly ever used.

```rust
static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

fn main() {
    // In Sequentially consistent ordering only one of them can win the race.
    let a = thread::spawn(|| {
        A.store(true, SeqCst);
        if !B.load(SeqCst) {
            unsafe { S.push('!') };
        }
    });

    let b = thread::spawn(|| {
        B.store(true, SeqCst);
        if !A.load(SeqCst) {
            unsafe { S.push('!') };
        }
    });

    a.join().unwrap();
    b.join().unwrap();
}
```
