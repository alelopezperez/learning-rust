fn main() {
    let mut s1 = String::from("helloklk");

    let ps1 = &s1;

    borrow_ref(ps1);
    borrow_ref(&s1);

    println!("{ps1}");

    borrow_mut(&mut s1);
    borrow_mut(&mut s1);

    println!("{s1}");

    let mut main_s = String::from("Hellow");
    //You cannot have two mutable references or more;
    //cannot mismatch mutable reference with immatuable ref
    let r1 = &main_s;
    println!("{r1}"); // if the lawest usage is not repeated it works, also if there is no usage

    //Non-Lexical Lifetimes a wayt to optimize last usage so there is no need to worry about

    let r2 = &mut main_s;
    println!("{r2}");

    //Time For Dangling Pointer reverse now from func to main fn
    let s_dangle = dangle();
}

fn borrow_ref(refr: &String) {
    println!("{refr}");
    let size = refr.len();
    println!("{size}");
}

fn borrow_mut(something: &mut String) {
    something.push_str("string");
}

fn dangle() -> &String {
    let s = String::from("hello");
    return &s;
}

/*
The Rules of References
Let’s recap what we’ve discussed about references:

At any given time, you can have either one mutable reference or any number of immutable references.
References must always be valid.
Next, we’ll look at a different kind of reference: slices.

 */
