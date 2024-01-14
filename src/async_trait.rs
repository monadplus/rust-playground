use async_trait::async_trait;
use derive_more::Deref;

async fn foo() {}

trait Hello {
    fn hello(&self) {
        println!("Hello!")
    }
}

#[derive(Deref)]
struct Wrapper<A: ?Sized>(A);

#[async_trait(?Send)]
trait Trait<A: Hello> {
    fn get(&self) -> &Wrapper<A>;

    async fn run(&self) {
        let b = self.get();
        foo().await;
        b.hello();
    }
}
