use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
fn main() {
    println!("Hello, world!");

    let mut map = HashMap::new();
    map.insert(String::from("ads"), 10);
    map.insert(String::from("askld"), 1123);

    let f = "ads";
    println!("{}", map["ads"]);
    println!("{}", map.get(f).unwrap());

    let cpy = map.get(f).unwrap().clone(); // get needs a ref of the type of key
    let capi = map.get(f).copied().unwrap_or(1);

    for k in &map {
        println!("{}", k.1);
    }

    for (_, v) in &map {
        println!("{}", v);
    }

    for (k, v) in map {
        println!("{} {}", k, v);
    }

    // Hash Maps Take owner of type that do note implement the copy trait (=)

    let ss = String::from("123123");
    let ss1 = String::from("123123");

    let num = 696969;
    let mut hmap = HashMap::new();
    hmap.insert(&ss, num);
    println!("{num}");

    println!("{ss}");

    println!("{}", hmap[&ss1]);

    // Hashmaps  entry gives back mut ref of the value; the other just gives a copy if it can or moves

    let letsee = hmap.entry(&ss).or_insert(0);
    //let letsee2 = hmap.entry(&ss).or_insert(0); // more than once error;

    //*letsee2 += 1;

    *letsee += 1;
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let mut last = HashMap::new();
    last.insert(String::from("asd"), String::from("asd"));

    let ok = last["asd"].clone();
}
