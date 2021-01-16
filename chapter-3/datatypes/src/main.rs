fn main() {
    let guess:u32 = "42".parse().expect("Not a number");// Here parse infers from the varible type
    let guess2 = "41".parse::<u32>().expect("Not a number"); // with this am infering the value because I specfied to the parse method
    println!("Guess num value = {}", guess);
    println!("Guess2 num value = {}", guess2);

    // Compound Types -> Tuple

    let tup: (i32, f64, char) = (500, 3.25, 'a');

    // Match assginment via destructuring the tuple tup 
    let (x, y, z) = tup;

    println!("{}", x);
    println!("{}", y);
    println!("{}", z);

    // Accesing a tuple values with its index using dot (.)

    let p = tup.0;
    println!("p = {}", p);
    println!("Tuple position 2 the last position = {}", tup.2);

     // Compound Types -> Array

    let array = [1,2,3,4,5]; // Infering and intializing with custom values;
    println!(" Array with inference {}", array[0]);

    let unitialized_array:[char; 3]; // Unintialized array of 3 chars;
    unitialized_array =  ['a', 'b', 'c']; //intilizing the array;
    println!(" Array with uninlized the initialized {}", unitialized_array[1]);

    let ap :[u8; 3]; // declaring empty array
    ap =[1; 3]; // intializing an array with 3s ones.
    println!(" Array with uninlized then with the same value 3 times {}", ap[2]);

    let apnew = [3; 5]; // combined the two from above;
    println!(" The combination with inference and the same values 5 times {}", apnew[4]);

    /*
    let index =20;
    println!("{}", apnew[index]); error out of bounds; at runtime it's not accesing the invalid memomry like in c or c++
    */ 





}
