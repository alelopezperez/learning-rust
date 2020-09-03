extern crate rand;
use std::io;
use rand::Rng;
use std::result::Result;
// :: means the the function is part of the type not from the instance
// in other words is static method

// references are immutable by default 
//&guess=immutable reference
//&mut guess = mutable reference

use std::cmp::Ordering; 

fn main() {
    println!("Guess The Number!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    
    loop{
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        //shadowing to change from String to U32
        // It seems that i can do anything in the curly braces and the final varible will be returned
        let guess: u32 = match guess.trim().parse(){
            Result::Ok(num) => {let numa:u32 = 4; println!("Hello {} {}",numa, num); num},
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            },
        }
    }

}
