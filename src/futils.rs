pub fn compose<A, B, C, F1, F2>(f2: F2, f1: F1) -> impl Fn(A) -> C
where
    F1: Fn(A) -> B,
    F2: Fn(B) -> C,
{
    move |x| f2(f1(x))
}

pub fn id<T>(o: T) -> T {
    o
}