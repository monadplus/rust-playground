enum Either<L, R> {
    Left(L),
    Right(R),
}

enum Void {}

fn never_left(e: Either<Void, String>) -> String {
    match e {
        Either::Right(str) => str,
        // The compiler is not clever enough to see that this enum has no branches and so it cannot be constructed.
        Either::Left(_) => unreachable!(),
    }
}
