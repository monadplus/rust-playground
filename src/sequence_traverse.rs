use std::iter::FromIterator;

pub fn sequence<A, S, T>(source: S) -> T
where
    S: IntoIterator<Item = A>,
    T: FromIterator<A>,
{
    source.into_iter().collect()
}

pub fn traverse<A, B, F, S, T>(source: S, f: F) -> T
where
    S: IntoIterator<Item = A>,
    F: Fn(A) -> B,
    T: FromIterator<B>,
{
    source.into_iter().map(f).collect()
}

#[test]
fn sequence_test() {
    let xs: Vec<Result<u32, String>> = (1..10).into_iter().map(Ok).collect();
    let result: Result<Vec<u32>, String> = sequence(xs);
    let expected: Result<Vec<u32>, String> = Ok((1..10).into_iter().collect());
    assert_eq!(result, expected);

    let mut xs: Vec<Result<u32, String>> = (1..10).into_iter().map(Ok).collect();
    xs.push(Err("Failed".to_owned()));
    let result: Result<Vec<u32>, String> = sequence(xs);
    let expected: Result<Vec<u32>, String> = Err("Failed".to_owned());
    assert_eq!(result, expected);

    let xs: Vec<Option<u32>> = (1..10).into_iter().map(Some).collect();
    let result: Option<Vec<u32>> = sequence(xs);
    let expected: Option<Vec<u32>> = Some((1..10).into_iter().collect());
    assert_eq!(result, expected);

    let mut xs: Vec<Option<u32>> = (1..10).into_iter().map(Some).collect();
    xs.push(None);
    let result: Option<Vec<u32>> = sequence(xs);
    let expected: Option<Vec<u32>> = None;
    assert_eq!(result, expected);
}

#[test]
fn traverse_vec_test() {
    // Traversable ~ Vec
    // Applicative ~ Result
    let xs: Vec<u32> = (1..10).into_iter().collect();
    let result: Result<Vec<u32>, String> = traverse(xs, |x| Ok::<u32, String>(x));
    let expected: Result<Vec<u32>, String> = Ok((1..10).into_iter().collect());
    assert_eq!(result, expected);

    let xs: Vec<u32> = (1..10).into_iter().collect();
    let result: Result<Vec<u32>, String> = traverse(xs, |x| {
        if x < 9 {
            Ok::<u32, String>(x)
        } else {
            Err::<u32, String>("failed".to_string())
        }
    });
    let expected: Result<Vec<u32>, String> = Err("failed".to_string());
    assert_eq!(result, expected);

    // Traversable ~ Vec
    // Applicative ~ Option
    let xs: Vec<u32> = (1..10).into_iter().collect();
    let result: Option<Vec<u32>> = traverse(xs, Some);
    let expected: Option<Vec<u32>> = Some((1..10).into_iter().collect());
    assert_eq!(result, expected);

    let xs: Vec<u32> = (1..10).into_iter().collect();
    let result: Option<Vec<u32>> = traverse(xs, |x| if x < 9 { Some(x) } else { None });
    let expected: Option<Vec<u32>> = None;
    assert_eq!(result, expected);
}
