mod futils;
use futils::compose;

fn h(n: i32) -> i32 {
    n + 7
}

fn g(n: i32) -> i32 {
    n - 2 
}

fn f(n: i32) -> i32 {
    n * 3
}

#[test]
fn composing() {
    assert_eq!(compose(g, f)(1), g(f(1)));
    assert_eq!(compose(h, compose(g, f))(1), compose(compose(h, g), f)(1));
    assert_eq!(compose(f, compose(f, f))(1), compose(compose(f, f), f)(1));
}

fn main() {
    //assert!("{}", 2 == id(2));
}
