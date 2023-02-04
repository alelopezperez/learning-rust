use std::slice;
extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world!"; //Global are static //'static
static mut COUNTER: u32 = 0;

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    //Here we created both a mut and inmut variables no harm done since we cannot anything

    println!("{:?} {:?}", r1, r2);

    let address = 0x012345usize;
    let r = address as *const i32; //created our own address

    //lets print
    unsafe {
        println!("{}", *r1);
        println!("{}", *r2);
        //println!("{}", *r); // seg fault
    }
    // println!("{}", *r1); cant derenrence a raw pointer

    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }

    // let mut v = vec![1, 2, 3, 4, 5, 6];

    // let r = &mut [..];
    // let (a, b) = r.split_at_mut(5);

    // fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = values.len();
    //     let ptr = values.as_mut_ptr();

    //     assert!(mid <= len);

    //     unsafe {
    //         (
    //             slice::from_raw_parts_mut(ptr, mid),
    //             slice::from_raw_parts_mut(ptr.add(mid), len - mid),
    //         )
    //     }
    // }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER); //acesing mut static is unsafe
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
