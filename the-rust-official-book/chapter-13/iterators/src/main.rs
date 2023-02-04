#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

fn main() {
    // Lazy Iterator

    //into_iter takes owenishio
    //iter_mut mut referecne

    let v1 = vec![1, 2, 4];
    let v1_iter = v1.iter();

    for val in v1_iter {
        //takes owenr ship of the iter
        // doin this goes throu the vector
        println!("got : {val}")
    }

    for val in &v1 {
        //without & we lose ownership
        println!("normal for {val}");
    }

    println!("{:?}", v1);

    //using next diretly
    // next change the mutates self

    let mut v1_iter = v1.iter();
    println!("{:?}", v1_iter);
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter);

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum(); //this take ownership of v1_ITER

    println!("{total}, {:?}", v1);

    let v1: Vec<i32> = vec![1, 2, 3];
    // map : Iterator adaptors  they are lazy they dont execute
    // collect :CONSUMING ADAPTOR
    let c1: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", c1);
    println!("{:?}", v1);

    let f1: Vec<i32> = v1.iter().copied().filter(|x| *x == 1).collect();
}
