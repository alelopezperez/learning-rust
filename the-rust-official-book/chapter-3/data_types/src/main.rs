fn main() {
    // SCALAR DATE TYPES

    // Integers
    /*
           8-bit	i8	u8
           16-bit	i16	u16
           32-bit	i32	u32
           64-bit	i64	u64
           128-bit	i128	u128
           arch	isize usize

    */

    // Floating-point Numbers
    // Booleans
    // Character

    //Compund Types
    //Tuple

    let tup: (&str, i32) = ("gola", 123);

    //destructuring
    let (x, y) = tup;

    println!("{} {} {}, {}", x, y, tup.1, tup.0);

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

    arr[0] = 1
}
