struct Ale {
    num: u32,
}
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
    V8(Ale),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// We can have impl, fn for enum methods

impl Message {
    fn call(&self) {
        println!("KLK we will use match later for this")
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    //Creating enums

    let four = IpAddrKind::V4(123, 3, 3, 1);
    let six = IpAddrKind::V6(String::from("ip6addrklk"));

    route(four);
    route(six);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Option::Some('e');

    let absent_num: Option<u32> = None;

    let sm = some_number.unwrap_or(2) + 5;

    let sm1 = Some(123);
    let ans = plus_one(sm1);

    dice_roll();
}

// ipaddrkind::V4 is a variant no a type, Ipaddking in itself is a type
// fn r(ip:ipaddkind::v4) is invalid
fn route(ip: IpAddrKind) {
    println!("Somting here");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Luvky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quater from {:?}", state);
            25;
            match state {
                UsState::Alabama => {
                    println!("Matching the matcher");
                    25
                }
                UsState::Alaska => {
                    println!("THe best ");
                    25
                }
            }
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1), // ohh ok ok rembemr some is option
    }
}

fn dice_roll() {
    let dice = 9;

    match dice {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        _ => reroll(), // when we dont use the value
        _ => (),       // the doo nothing
    }

    let some_value = Some(3123123);

    if let Some(value) = some_value {
        println!("{value}")
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
