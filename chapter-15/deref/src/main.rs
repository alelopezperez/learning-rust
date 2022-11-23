use std::mem::drop;
use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
fn main() {
    let x = 5;
    let y = &x;
    let z = &x;

    println!("{} {}", x, y);

    assert_eq!(5, x);
    // assert_eq!(5, y); we need to deref sometime is donde automatiicaly in assert is seems not. &i32 is not the same as i32
    assert_eq!(5, *y);
    assert_eq!(&x, y);
    assert_eq!(z, y);

    let mut bx = 5;
    let by = Box::new(&bx); // it creates a new value

    println!("{} {}", bx, by);

    assert_eq!(5, bx);
    // assert_eq!(5, by);
    assert_eq!(5, **by);

    bx = 4;
    println!("{} ", bx);
    // println!("{}", by); we created a &mut when using a box to a reference

    let x = 6;
    let y = MyBox::new(x);

    assert_eq!(6, x);
    assert_eq!(6, *y);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // c.drop(); cannot drop  explicitly
    drop(c); // use std mem drop
    println!("CustomSmartPointer dropped before the end of main.");

    let ad = 6;
    let af = 6;

    let pp = &ad;
    let pa = &af;

    assert_eq!(pp, pa);

    let st = String::from("asd");
    let bst = Box::new(st);

    //  println!("{}", st); st does not implemente the copy trait like primitive type
}
