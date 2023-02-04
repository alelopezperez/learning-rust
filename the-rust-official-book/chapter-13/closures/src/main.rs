use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}
//Remember option enum has the shirt color or none in this case
impl Inventory {
    fn giveaway(&self, user_prefrence: Option<ShirtColor>) -> ShirtColor {
        user_prefrence.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_prefi = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_prefi);
    println!("The user  has {:?} got {:?}", user_prefi, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("Calculting");
        //   thread::sleep(Duration::from_secs(2));
        num
    };
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| x + 1;
    // let add_one_v4 = |x| x + 1;

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    //let n = example_closure(5); error it inferended from usage as string first like Vec::new

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    let mut list = vec![1, 2, 3];
    let mut borrows_mutably = || list.push(7); //declare conslure as mutable so it can muta
                                               //println!("After calling closure: {:?}", list); before using the var all other mutable rerefernce should stop
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From threade {:?}", list))
        .join()
        .unwrap();
    // println!("{:?}", list) use move cannot do it no onweniship
}

fn add_one_v1(x: u32) -> u32 {
    x + 1
}
