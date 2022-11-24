fn main() {
    //Type Alaising

    type km = i32;

    let x: i32 = 5;
    let k: km = 4;
    println!("x + y = {}", x + k);

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() {
        // --snip--
    }

    // The Never type
    fn bar() -> ! {}

    //Match must always return the same type

    //Dynamcally size type

    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    //
    fn generic<T: ?Sized>(t: &T) {
        // --snip--
    }
}
