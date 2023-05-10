// Source: https://github.com/alexpusch/rust-magic-function-params

pub trait FromContext<C> {
    fn from_context(context: &C) -> Self;
}

pub trait Handler<T, C> {
    fn call(self, context: C);
}

pub fn trigger<T, H, C>(context: C, handler: H)
where
    H: Handler<T, C>,
{
    handler.call(context);
}

impl<F, T, C> Handler<(T,), C> for F
where
    F: Fn(T),
    T: FromContext<C>,
{
    fn call(self, context: C) {
        (self)(T::from_context(&context));
    }
}

impl<F, T1, T2, C> Handler<(T1, T2), C> for F
where
    F: Fn(T1, T2),
    T1: FromContext<C>,
    T2: FromContext<C>,
{
    fn call(self, context: C) {
        (self)(T1::from_context(&context), T2::from_context(&context));
    }
}

macro_rules! impl_from_context {
    () => {};
    ($($tys:ident),+ $(,)?) => {
        impl<C, $($tys: FromContext<C>,)*> FromContext<C> for ($($tys,)*) {
            fn from_context(context: &C) -> Self {
                ($($tys::from_context(context),)*)
            }
        }
        impl_from_context!(@next $($tys,)*);
    };
    (@next $ty:ident, $($other:ident,)*) => { impl_from_context!($($other),*); };
}

impl_from_context!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);

pub mod example {
    use super::*;

    #[derive(Clone)]
    pub struct Context {
        param: String,
        id: u32,
    }

    impl Context {
        pub fn new(param: String, id: u32) -> Self {
            Context { param, id }
        }
    }

    impl FromContext<Context> for Param {
        fn from_context(context: &Context) -> Self {
            Param(context.param.clone())
        }
    }

    impl FromContext<Context> for Id {
        fn from_context(context: &Context) -> Self {
            Id(context.id)
        }
    }

    pub struct Param(pub String);

    pub struct Id(pub u32);
}

#[test]
fn multi_parameter() {
    use example::*;

    fn print_id(id: Id) {
        println!("id is {}", id.0);
    }

    fn print_param(Param(param): Param) {
        println!("param is {param}");
    }

    fn print_tuple(tuple: (Param, Id)) {
        println!(
            "param is {param}, id is {id}",
            param = tuple.0 .0,
            id = tuple.1 .0
        );
    }

    fn print_all(Param(param): Param, Id(id): Id) {
        println!("param is {param}, id is {id}");
    }

    fn print_all_switched(Id(id): Id, Param(param): Param) {
        println!("param is {param}, id is {id}");
    }

    let context = Context::new("magic".into(), 33);
    trigger(context.clone(), print_id);
    trigger(context.clone(), print_param);
    trigger(context.clone(), print_tuple);
    trigger(context.clone(), print_all);
    trigger(context, print_all_switched);
}
