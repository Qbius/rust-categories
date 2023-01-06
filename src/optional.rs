fn opt_compose<A, B, C>(f2: impl Fn(B) -> Option<C>, f1: impl Fn(A) -> Option<B>) -> impl Fn(A) -> Option<C> {
    move |obj: A| match f1(obj) {
        Some(result) => f2(result),
        None => None
    }
}

fn opt_identity<A>(obj: A) -> Option<A> {
    Some(obj)
}

fn safe_root(a: f64) -> Option<f64> {
    match a >= 0.0 {
        true => Some(a.sqrt()),
        false => None,
    }
}

fn safe_reciprocal(a: f64) -> Option<f64> {
    match a != 0.0 {
        true => Some(1.0 / a),
        false => None,
    }
}

fn threedec(a: f64) -> Option<f64> {
    match format!("{a:.3}").parse::<f64>() {
        Ok(result) => Some(result),
        Err(_) => None,
    }
}

#[test]
fn slow_inverse_square_root() {
    let base = 6.349;
    assert_eq!(opt_compose(threedec, opt_compose(safe_root, safe_reciprocal))(base), opt_compose(threedec, opt_compose(safe_reciprocal, safe_root))(base));
    // they're not fully equal since one is Some(0.3968691458357084) and the other Some(0.39686914583570837) which is funny

    assert_eq!(safe_root(base), opt_compose(opt_identity, safe_root)(base));
    assert_eq!(safe_root(base), opt_compose(safe_root, opt_identity)(base));
}