use std::fmt;

#[derive(Debug)]
enum List<T> where T: fmt::Debug {
    Cons(T, Box<List<T>>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}
