use std::sync::atomic::compiler_fence;

#[derive(Debug)]
struct MyA<A> {
    a: A,
}

impl<A: Default> Default for MyA<A> {
    fn default() -> Self {
        Self {
            a: Default::default(),
        }
    }
}

#[derive(Debug)]
struct MyB;

fn to_b<A>(_a: MyA<A>) -> MyB {
    MyB
}

#[derive(Debug)]
struct MyC<C> {
    c: C,
}

fn to_c<A>(a: MyA<A>) -> MyC<A> {
    MyC { c: a.a }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_experiment_1() {
        let mut a: MyA<u8> = MyA { a: 1 };
        let ptr_a: *mut MyA<u8> = &mut a;
        let b = to_b(a);
        unsafe {
            ptr_a.write_volatile(MyA::<u8>::default());
            compiler_fence(std::sync::atomic::Ordering::SeqCst);
        }
        println!("{b:?}");

        let mut a: MyA<u8> = MyA { a: 1 };
        let ptr_a: *mut MyA<u8> = &mut a;
        println!("{a:?}");
        let c = to_c(a);
        unsafe {
            ptr_a.write_volatile(MyA::<u8>::default());
            compiler_fence(std::sync::atomic::Ordering::SeqCst);
        }
        unsafe {
            let a = ptr_a.read_volatile();
            println!("{a:?}");
        }
        // Why c is not changed?
        println!("{c:?}");
    }
}
