fn main() {
    /* println!("Hello, world!");

     let mut s = String::from("hola");
     let r1 = &mut s;
     println!("{r1}");
     let r2 = &mut s;
     println!("{r2}"); //THisn only worwks thank to Non Lexica end of lifetime

    // println!("{r1} {r2}") error
     */

    let mut s = String::from("Klk Wawawa");
    let index = first_word(&s);

    println!("{index}");

    //Slices from string

    let slice = &s[0..1]; // starting point endpoint start pluse (end - start)
    let slice2 = &s[..1]; // asumes starting point 0

    println!("{slice}1");
    let wrd = first_word_slice(&s);
    // s.clear(); // this cannot be done because now we have an imutable reference
    println!("{}", wrd);
    s.clear();

    let literal_s = "THis is a string literal is a string slice";

    // Array Slice
    let a = [1, 2, 3, 4, 5];
    let sa = &a[0..3];
    println!("{}", sa[2]); // 0 an it takes 3 since 0 counts as 1
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len() - 1;
}

fn first_word_slice(s: &str) -> &str {
    //Change &string to &str since we now the size in the moment
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}
