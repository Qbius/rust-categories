use crate::futils::{f, g, h, compose, id};

fn fmap<A, B>(f: impl Fn(A) -> B) -> impl Fn(Option<A>) -> Option<B> {
    move |obj| match obj {
        Some(a) => Some(f(a)),
        None => None,
    }
}

#[test]
fn lifting() {
    assert_eq!(fmap(compose(g, f))(Some(3)), compose(fmap(g), fmap(f))(Some(3)));
    assert_eq!(compose(fmap(compose(g, h)), id)(Some(2)), compose(compose(fmap(g), id), fmap(h))(Some(2)));
}  

