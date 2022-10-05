fn main() {
    let mut s: String = "String Literal change to a a string".to_string();
    s += "asd";
    println!("{s}");

    let l: &str = "ok ok ok";
    println!("{l}");

    let mut st = String::from("Example");
    st.push_str("more");
    println!("{st}");

    // Binding with primitive/ stack allocated
    let x = 5;
    let y = x;

    let xs = "HOla";
    let ys = xs;

    println!("{x} {y} also {xs} and {ys}");

    // Binding with complex heap allocated data
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}");
    // println!("{s1}"); no because this a a move and equals does no automacally do a full copy,
    //so its a shallow copy and since ownership rules, s1 dies goes to s2

    //how to do it
    //using clone to make a full deep copy no just a move (shallow copy it seems to not be posible unless maybe use box or somthing like smrt_ptrs)
    let s3 = s2.clone();
    println!("{s3}");

    // MOVE COPY  in function

    let str_own = String::from("chaing ownship");
    takes_own(str_own); // we made it mutable
                        //println!("{str_own}"); cannot be done it lot ownership
    let int_own = 334;
    makes_copy(int_own);
    println!("{int_own}");

    let letters = String::from("taking and returning");
    let s4 = takes_gives(letters);

    println!("{s4} "); //cannot print letter lost ownshipg

    let s5 = String::from("hello");

    let (s6, len) = calculate_length(s5);

    println!("The length of '{}' is {} .", s6, len); //s5 lost ownshirp
}

fn takes_own(mut some_str: String) {
    println!("{some_str}");
    some_str.push_str("klk");
    println!("{some_str}");
}

fn makes_copy(mut some_int: u32) {
    some_int += 1;
    println!("{some_int}");
}

fn takes_gives(mut letters: String) -> String {
    letters.push_str("asd");
    return letters;
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
