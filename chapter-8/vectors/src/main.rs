enum SpreadType {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    //Remeber Vector are stored in the heap
    // let mut v:Vec<i32> = Vec::new() is the same if we dont specify and start pushing
    let mut v = Vec::new();
    v.push(1);
    v.push(15);
    v.push(2);

    //using the macro
    let v2 = vec![1, 3, 4];
    println!("{}", v2[1]);
    println!("{}", v[1]);

    //let third
    let third = &v[1];
    println!("{}", third);

    let third = v.get(4); // Usin get returns a Option, so it does not panic
                          // let pan = v[123]; this panics
    match third {
        Some(t) => println!("{t}"),
        None => println!("bue"),
    }
    
    match v.get(2) { //more gracefull way
        Some(t) => println!("{t}"),
        None => println!("bue"),
    }
    

    // for each loop

    for mut i in v.clone() {
        // if i did not put the clone a move operation would have happnned
        i += 1;
        print!("{i}, ");
    }
    println!();

    for i in &mut v {
        *i += 3;
        print!("{i}, ");
    }
    println!();

    for i in &v {
        print!("{i}, ");
    }
    println!();

    let mut t = vec![1, 3, 4, 1, 3, 4];
    for i in &mut t {}

    let mut i = 13;
    let a = &mut i;

    let mut s = String::from("asd");
    let ss = &mut s;

    *ss = String::from("diferent"); // dereference ok ok ok

    *a += 10;

    // a Vector of enum

    let diff_t = vec![SpreadType::Int(3), SpreadType::Float(123.0)];

    for i in &diff_t {}
}

fn change(s: &mut String) {
    *s = String::from("asd");
}
