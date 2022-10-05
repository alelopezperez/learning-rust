fn main() {
    let num = 5;

    if (num < 5) {
        println!("small");
    } else if num > 4 {
        println!("big")
    } else {
        println!("it's five")
    }

    //if number it needs to be a boolean

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let strg = println!("{}", number);

    let mut counter = 0;

    let count = 'counting: loop {
        //infinite loop
        counter += 1;
        // if counter == 1 {
        //     break 'counting;
        // }
        if counter == 10 {
            break counter;
        }
    };

    println!("counter {}", count);

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {element}"); //reference
    }

    for element in a {
        println!("the value is: {element}"); //copy
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    for number in (1..4) {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
