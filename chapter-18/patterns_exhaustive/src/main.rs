fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Color {}", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    let v = vec!['a', 'b', 'c'];

    let p = v.iter();

    for (ix, val) in v.iter().enumerate() {
        println!("letter {} in pos {}", val, ix)
    }

    let tup = (3, 2, 1);
    let (x, y, z) = tup;
    println!("{x}");
    let (x, y, z) = (1, 2, 3);
    println!("{x}");

    let point = (3, 5);
    print_coordinates(&point);

    // let stamte and for is only for irefutable patterns
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
