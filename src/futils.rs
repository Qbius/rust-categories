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

pub fn h(n: i32) -> i32 {
    n + 7
}

pub fn g(n: i32) -> i32 {
    n - 2 
}

pub fn f(n: i32) -> i32 {
    n * 3
}

#[test]
fn composing() {
    assert_eq!(compose(g, f)(1), g(f(1)));
    assert_eq!(compose(h, compose(g, f))(1), compose(compose(h, g), f)(1));
    assert_eq!(compose(f, compose(f, f))(1), compose(compose(f, f), f)(1));

    let a = |n| n * 5;
    let b = |n| n - 3;
    assert_eq!(compose(a, b)(2), -5);
    assert_eq!(compose(a, a)(2), 50);
}

#[test]
fn iding() {
    assert_eq!(1, id(1));
    assert_eq!(1, id(id(id(id(id(1))))));
}