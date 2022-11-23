enum List {
    Cons(i32, Box<List>),
    Nil,
}

//use crate::List::{Cons, Nil}
fn main() {
    let b = Box::new(4);
    println!(" box {:?}", b);
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    let l = List::Cons(1, Box::new(List::Nil));
}
