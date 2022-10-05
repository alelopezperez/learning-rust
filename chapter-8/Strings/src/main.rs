fn main() {
    // Strign is a vector of bytes

    let mut s = String::new();

    let ex = "intit".to_string(); // string slices

    let hello = String::from("こんにちは"); // Thanks to utf-8 on rust

    println!("{hello}");

    s += "ades new text";
    s.push_str("adding more");
    s += &ex;
    s.push_str(&ex);
    s = hello + &ex; //no &hello because we need to take ownership and add &String coerced to &str how? idk
    println!("{s} ");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{s}");

    let hello = String::from("Hola");
    println!("{}", hello.len());

    let new_hello = "Здравствуйте";
    let sli = &new_hello[0..4]; // this can crash //panic e.g &new_hello[0..1]
    println!("{sli}");

    //how to actually iterate
    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }
}
