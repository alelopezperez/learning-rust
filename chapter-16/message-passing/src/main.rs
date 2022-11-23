use std::time::Duration;
use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    println!("Before new thread");
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let val = String::from("hi!");
        println!("HI from new thread");
        thread::sleep(Duration::from_secs(2));
        tx1.send(val).unwrap();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for i in vals {
            tx1.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let received = rx.recv().unwrap(); // blocks an wait for somthing // it's like a join
                                       //try_rc does not wait and if there is nothing it will return err
    println!("Got: {}", received);

    for r in rx {
        println!("GOt {}", r); // in this for loop it just wait until the channel is closed (or drop from the thread)
    }
}
