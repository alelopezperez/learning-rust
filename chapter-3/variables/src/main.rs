// let mut y = 5; //global variable error
const MAX_POINTS: u32 = 10000; // REMEMBER ALWAYS PUT THE TYPE
fn main() {
    let x = 5; // let is like auto; it will infere the type from the equal; i think 5 will be u32 or u64
    println!("the variable x has a value of {}", x);
  //  x=6; // this will be an error because is imutable

    let mut x2 =77;
    println!("new var = {}", x2);
    x2 = 5;
    println!("changed it to {}", x2);

    let x = 6;
    println!(" Creating a new varible with the same name; reasginig; no it's called shadowing {}", x);

   // println!(" Creating a new global varible y = {}", y); error, no global variable mut or not.

    println!("Global const MAX_POINTS = {}", MAX_POINTS);

    let x = x+1;
    println!("Shadowing 6+1 = {}", x);

    let mut ppam = 1;
    ppam = ppam + 50;
    println!("pseudo shadowing with mut {} ", ppam);

    let spaces = "    ";
    let spaces = spaces.len();

    println!("hey num of spaces shadowed from string to num U32 -> {}", spaces);

    let mut word = "hola";
    println!("this works word = {}", word);
    word = "ale";
    println!("this works word  changes to = {}", word);
 
  //  word = word.len(); error trying to change the type of varible without using an imutable with shadowing
    let mut word = word.len();
    println!("shadowing with mut because we are using let -> {}", word); // this gives a warning because am not chagin the value in the program.
    word = word+1;
    println!("shadowing with mut because we are using let -> {}", word); // i changed the value; no more warinings.

    let guess:u32;
  //  println!("guess -> {}", guess); /// unitialized
    guess = 4;
    println!("guess -> {}", guess);
}
