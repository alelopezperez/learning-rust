use std::ffi::VaList;

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

//As clouse
fn do_twices<T>(f: T, arg: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}
fn main() {
    println!("Hello, world!");

    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    enum Status {
        Value(u32),
        Stop,
    }

    let l = Vec<Status> = (0u32..20).map(Status::Value).collect() //enuma init is like a fn


}

// fn returns_closure() -> dyn Fn(i32) -> i32 { no conrecte type does not know size
//     |x| x + 1
// }

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
fn returns_closures() -> impl Fn(i32)->i32 {
    Box::new(|x| x + 1)
}