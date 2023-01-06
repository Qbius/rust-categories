
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub type Maybe<A> = Either<(), A>;
fn either_to_option<A>(e: Maybe<A>) -> Option<A> {
    match e {
        Maybe::<A>::Left(()) => None,
        Maybe::<A>::Right(obj) => Some(obj)
    }
}

fn option_to_either<A>(o: Option<A>) -> Maybe<A> {
    match o {
        Some(obj) => Maybe::<A>::Right(obj),
        None => Maybe::<A>::Left(()),   
    }
}

fn swap<A, B>((a, b): (A, B)) -> (B, A) {
    (b, a)
}

struct Pair<A, B>(A, B);

struct Element {
    name: String,
    symbol: String,
    atomic_number: u8,
}

impl Element {
    fn from_tuple((name, symbol, atomic_number): (String, String, u8)) -> Element {
        Element {
            name,
            symbol,
            atomic_number,
        }
    }

    fn to_tuple(self) -> (String, String, u8) {
        (self.name, self.symbol, self.atomic_number)
    }
}

use std::fmt::Display;
enum List<A: Display> {
    Cons(A, Box<List<A>>),
    Nil,
}

impl<A: Display> List<A> {
    fn cons(head: A, tail: List<A>) -> List<A> {
        Self::Cons(head, Box::<List<A>>::new(tail))
    }

    fn to_string(self) -> String {
        match self {
            Self::Cons(obj, next) => obj.to_string() + &(*next).to_string(),
            Self::Nil => "".to_string()
        }
    }
}

enum Shape {
    Circle(f64),
    Rect(f64, f64),
    Square(f64),
}

use std::f64::consts::PI;
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Self::Circle(radius) => PI * radius * radius,
            Self::Rect(width, height) => width * height,
            Self::Square(side) => side * side,
        }
    }

    fn circ(&self) -> f64 {
        match self {
            Self::Circle(radius) => 2.0 * PI * radius,
            Self::Rect(width, height) => 2.0 * width + 2.0 * height,
            Self::Square(side) => 4.0 * side,
        }
    }
}

#[test]
fn tuple_trying() {
    let t = (7, false);
    assert_eq!(swap(t), (false, 7));
}

#[test]
fn listing() {
    type L = List<String>;
    let list = L::cons("aka".to_owned(), L::cons("pulko".to_owned(), L::Nil));
    println!("{}", list.to_string());
}

