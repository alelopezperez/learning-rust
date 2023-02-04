struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}
//only work on instance of floating pints
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Points<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Points<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Points<X2, Y2>) -> Points<X1, Y2> {
        Points {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let int_point = Point { x: 4, y: 4 };
    let float_point = Point { x: 4.3, y: 1.24 };
    let any_point = Point { x: 4.3, y: 1 };

    println!("p.x = {}", int_point.x());
    println!("p.x = {}", float_point.distance_from_origin());
    // error println!("p.x = {}", any_point.distance_from_origin());

    let p1 = Points { x: 5, y: 10.4 };
    let p2 = Points { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

//Using Generrics
// We can tell the our generic needs to have PartialOrd Trait
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(num_list: &[i32]) -> &i32 {
    let mut l = &num_list[0];

    for number in num_list {
        if number > l {
            l = number;
        }
    }
    l
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
