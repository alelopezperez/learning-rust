const P: u32 = 2;
fn main() {
    let x = 5;
    println!("the values of x is {} and {}", x, x);

    // x =6; will make a compiler error

    let mut y = 4;
    println!("the values of x is {} and {}", y, y);

    y = 3;
    println!("the values of x is {} and {}", y, y);

    println!("the values of x is {} and {}", y, y);

    println!("{} ", P);

    // Shadowing re

    let sh = 12;
    println!("{}", sh);

    let sh = 12 + 3;
    println!("{}", sh);
}
