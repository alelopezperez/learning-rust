fn main() {
    my_function(4, 'n');
    let p = my_function(123, 'b');
    println!("{}", p)
}

fn my_function(num: i32, ch: char) -> i32 {
    println!("HELLOW FROM HERE, {}{}", num, ch);
    5; //does not return
    4
    // or return 4;
}
