type Writer<T> = (T, String);

fn to_upper(s: &String) -> Writer<String> {
    (s.to_uppercase(), "to_upper".to_string())
}

fn words(s: &String) -> Writer<Vec<String>> {
    (s.split(' ').map(|chunk| chunk.to_string()).collect(), "words".to_string())
}

struct Procedural<A> {
    base: A,
    log: Vec<String>,
}

impl<A> Procedural<A> {
    fn then<B>(self, f: impl Fn(&A) -> Writer<B>) -> Procedural<B> {
        let mut log = self.log;
        let (base, log_append) = f(&self.base);
        log.push(log_append);
        Procedural::<B> {
            base,
            log,
        }
    }
}

fn proc<A>(base: A) -> Procedural<A> {
    Procedural::<A> {
        base,
        log: vec![],
    }
}

#[test]
fn perform1() {
    let base = "Akapulko to jest to".to_string();
    let (res1, log1) = to_upper(&base);
    let (res2, log2) = words(&res1);
    println!("{:?} ({})", res2, log1 + &log2);
}

#[test]
fn perform2() {
    let res = proc("Akapulko to jest to".to_string()).then(to_upper).then(words);
    println!("{:?} ({:?})", &res.base, &res.log);
}