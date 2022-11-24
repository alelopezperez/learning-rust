fn main() {
    // Matching Literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Named Variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("50"),
        Some(y) => println!("Matched y {y}"), // the same as some(pppp) we are just naming, so it shadowed y
        _ => println!("The actual value {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // Mutiple Patterns

    let x = 1;

    match x {
        1 | 2 => println!("mutlple match "),
        3 => println!("Single match"),
        _ => print!("anything"),
    }

    // Matching with ranges of numbers and chars

    let x = 5;

    match x {
        1..=5 => println!("one through five inclusive"),
        _ => println!("All of the other ones"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    //Destuctuin Structs with match

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    //shorter version if we use the same fields name
    let Point { x, y } = p;

    let p = Point { x: 1, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // Destructurin Enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(x, y, z) => println!("{},{},{}", x, y, z),
    };

    // Nested Matches

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Messages {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Messages::ChangeColor(Color::Hsv(0, 160, 255));
    let msgs = Messages::ChangeColor(Color::Hsv(0, 160, 255));

    match &msg {
        Messages::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Messages::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Messages::Write(text) => println!("Text message: {}", text),
        Messages::ChangeColor(color) => {
            match color {
                Color::Rgb(r, g, b) => println!("this i from rgb {}{}{}", r, g, b),
                Color::Hsv(h, s, v) => println!("this from h s v {} {} {}", h, s, v),
            };
        }
    }

    match msg {
        Messages::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Messages::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Messages::Write(text) => println!("Text message: {}", text),
        Messages::ChangeColor(Color::Hsv(h, s, v)) => println!("this from h s v {} {} {}", h, s, v),
        _ => println!("no more"),
    }

    // Destructuring Structs and Tuples

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    println!("feet and inches and point {} {} {} {}", feet, inches, x, y);

    //Ignoring Evenrything
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("ignored"),
        _ => println!("everyhting else"),
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, t, _, _) => {
            println!("Some numbers: {first}, {t}")
        }
    }

    //ignorin on use varibles

    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        //it still binds so we lose ownership
        println!("found a string");
    }

    //println!("{:?}", s);

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        // this does not bind at all
        println!("found a string");
    }

    println!("{:?}", s);

    //Ignore remaining

    struct Points {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Points { x: 0, y: 0, z: 0 };

    match origin {
        Points { x, .. } => println!("Only care for x {x}"),
    }

    let numbers = (2, 4, 8, 16, 32);

    match &numbers {
        (f, .., last) => println!("Only care of first and last {f}, {last}"),
    }

    match numbers {
        // (.., m, ..) => println!("ambigouse"),
        _ => println!("ambugous"),
    }

    // Match Guards aka as match with if statement

    let _n = Some(4);

    match _n {
        Some(x) if x % 2 == 0 => println!("oh it's even"),
        Some(_) => println!("it's not even and ignoring value"),
        None => println!("None case"),
    }

    // Noth shawing y
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(x) if x == y => println!("used y outside to do a comparison"),
        Some(_) => println!("all other nums"),
        None => (),
    }

    // More stuf

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        // this (4 | 5 | 6) if y => ...
        // not this 4 | 5 | (6 if y) => ...
        _ => println!("no"),
    }

    // @ Bindings

    enum MSG {
        Hello { id: i32 },
    }

    let msg = MSG::Hello { id: 5 };

    match msg {
        MSG::Hello { id: id_var @ 1..=5 } => println!("using @"), //testing and saving
        MSG::Hello { id: 10..=12 } => {
            //only testing
            println!("Found an id in another range")
        }
        MSG::Hello { id } => println!("Found some other id: {}", id), //only saving and not testing
    }
}
