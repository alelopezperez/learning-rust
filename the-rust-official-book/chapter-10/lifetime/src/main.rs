//struct that have a referecen instead of an owned type
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
fn main() {
    // Dangling Pointer
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r: {}", r);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    //println!("The longest string is {}", result); //outise it will have an error 'a is the smallest between two

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    // 'static means the thiw will live forever; default for string literal
    let s: &'static str = "I have a static lifetime.";
}
// Here we are specofyin or telling the compile that there is a lifetime a
// the a lifetime depend on x and y and that's what we ar returnning
// so &' str(the return type) is the same as does.
// the lifetime are not the same it picks the smallest between x and y

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Here we need to specify the return value lifetime
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

//dangling pointer
// fn longest3<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
