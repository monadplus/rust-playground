//! Source: https://recursion.wtf/posts/rust_schemes/#fn:1

use std::collections::VecDeque;

// Cache locallity is super important for performance.
// Traversing pointers will result in a cache miss.
// An it's likely that it's not in contiguous mememory, so it will fill the cache.
#[derive(Debug, Clone)]
pub enum ExprBoxed {
    Add {
        a: Box<ExprBoxed>,
        b: Box<ExprBoxed>,
    },
    Sub {
        a: Box<ExprBoxed>,
        b: Box<ExprBoxed>,
    },
    Mul {
        a: Box<ExprBoxed>,
        b: Box<ExprBoxed>,
    },
    LiteralInt {
        literal: i64,
    },
}

impl ExprBoxed {
    pub fn eval(&self) -> i64 {
        match &self {
            ExprBoxed::Add { a, b } => a.eval() + b.eval(),
            ExprBoxed::Sub { a, b } => a.eval() - b.eval(),
            ExprBoxed::Mul { a, b } => a.eval() * b.eval(),
            ExprBoxed::LiteralInt { literal } => *literal,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ExprLayer<A> {
    Add { a: A, b: A },
    Sub { a: A, b: A },
    Mul { a: A, b: A },
    LiteralInt { literal: i64 },
}

impl<A> ExprLayer<A> {
    #[inline(always)]
    fn map<B, F: FnMut(A) -> B>(self, mut f: F) -> ExprLayer<B> {
        match self {
            ExprLayer::Add { a, b } => ExprLayer::Add { a: f(a), b: f(b) },
            ExprLayer::Sub { a, b } => ExprLayer::Sub { a: f(a), b: f(b) },
            ExprLayer::Mul { a, b } => ExprLayer::Mul { a: f(a), b: f(b) },
            ExprLayer::LiteralInt { literal } => ExprLayer::LiteralInt { literal },
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct ExprIdx(usize);
impl ExprIdx {
    fn head() -> Self {
        ExprIdx(0)
    }
}

pub struct ExprTopo {
    // nonempty, in topological-sorted order. guaranteed via construction.
    elems: Vec<ExprLayer<ExprIdx>>,
}

impl ExprTopo {
    pub fn eval(self) -> i64 {
        self.collapse_layers(|expr| match expr {
            ExprLayer::Add { a, b } => a + b,
            ExprLayer::Sub { a, b } => a - b,
            ExprLayer::Mul { a, b } => a * b,
            ExprLayer::LiteralInt { literal } => literal,
        })
    }

    fn collapse_layers<A, F: FnMut(ExprLayer<A>) -> A>(self, mut collapse_layer: F) -> A {
        use std::mem::MaybeUninit;

        let mut results = std::iter::repeat_with(|| MaybeUninit::<A>::uninit())
            .take(self.elems.len())
            .collect::<Vec<_>>();

        for (idx, layer) in self.elems.into_iter().enumerate().rev() {
            let result = {
                let layer = layer.map(|x| unsafe {
                    let maybe_uninit =
                        std::mem::replace(results.get_unchecked_mut(x.0), MaybeUninit::uninit());
                    maybe_uninit.assume_init()
                });
                collapse_layer(layer)
            };
            results[idx].write(result);
        }

        unsafe {
            let maybe_uninit = std::mem::replace(
                results.get_unchecked_mut(ExprIdx::head().0),
                MaybeUninit::uninit(),
            );
            maybe_uninit.assume_init()
        }
    }

    pub fn from_boxed(seed: &ExprBoxed) -> Self {
        let mut frontier: VecDeque<&ExprBoxed> = VecDeque::from([seed]);
        let mut elems = vec![];

        // expand layers to build a vec of elems while preserving topo order
        while let Some(seed) = { frontier.pop_front() } {
            let layer = match seed {
                ExprBoxed::Add { a, b } => ExprLayer::Add { a, b },
                ExprBoxed::Sub { a, b } => ExprLayer::Sub { a, b },
                ExprBoxed::Mul { a, b } => ExprLayer::Mul { a, b },
                ExprBoxed::LiteralInt { literal } => ExprLayer::LiteralInt { literal: *literal },
            };
            let layer = layer.map(|seed| {
                frontier.push_back(seed);
                // idx of pointed-to element determined from frontier + elems size
                ExprIdx(elems.len() + frontier.len())
            });

            elems.push(layer);
        }

        Self { elems }
    }
}

#[cfg(test)]
proptest::proptest! {
    #[test]
    fn expr_eval(boxed_expr in arb_expr()) {
        let eval_boxed = boxed_expr.eval();
        let eval_via_collapse = ExprTopo::from_boxed(&boxed_expr).eval();

        assert_eq!(eval_boxed, eval_via_collapse);
    }
}

#[cfg(test)]
pub fn arb_expr() -> impl proptest::strategy::Strategy<Value = ExprBoxed> {
    use proptest::strategy::Strategy;

    let leaf =
        proptest::prelude::any::<i8>().prop_map(|x| ExprBoxed::LiteralInt { literal: x as i64 });
    leaf.prop_recursive(
        8,   // 8 levels deep
        256, // Shoot for maximum size of 256 nodes
        10,  // We put up to 10 items per collection
        |inner| {
            proptest::prop_oneof![
                (inner.clone(), inner.clone()).prop_map(|(a, b)| ExprBoxed::Add {
                    a: Box::new(a),
                    b: Box::new(b)
                }),
                (inner.clone(), inner.clone()).prop_map(|(a, b)| ExprBoxed::Sub {
                    a: Box::new(a),
                    b: Box::new(b)
                }),
                (inner.clone(), inner).prop_map(|(a, b)| ExprBoxed::Mul {
                    a: Box::new(a),
                    b: Box::new(b)
                }),
            ]
        },
    )
}
