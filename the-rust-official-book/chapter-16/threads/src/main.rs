use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spwaned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // handle.join().unwrap();

    let v = vec![1, 2, 3];

    let han = thread::spawn(move || println!("Heres vec {:?}", v));

    han.join().unwrap();
    // println!("Heres vec {:?}", v); we moved it
}

// the handle join works in order, so if it goes before any code it will block at the point where it was called
