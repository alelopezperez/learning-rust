fn main() {
    //When Panicking we do an unreacovable error that stops the program
    /* No Clean up
       [profile.release]
       panic = 'abort'
    */
    // panic!("Destroy");
    //How to Backtrace

    let v = vec![1, 2, 3];
    //buffer overread
    // automatic panics no buffer overead in rust
    v[99];
}
