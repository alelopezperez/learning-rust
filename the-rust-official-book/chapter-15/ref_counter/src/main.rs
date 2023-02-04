//RC is only for single thread scenarios and is immutable

use std::rc::Rc;

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

// use List::{Cons, Nil};
use List2::{Cons, Nil};

fn main() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); // here it a loses ownership

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a)); // can only clone an Rc becuse that's how we know how many time we ref a;
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after deleting c = {}", Rc::strong_count(&a));

    // let c = Cons(4, Rc::clone(&a));
}
