struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
// Tuple Struct; struct that looks like tuple
// no named fieldsU
struct Color(i32, i32, u32);
struct Point(i32, i32, u32);

//Unit Struct

struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    // asociated fn or when this happens Type::asc_fn

    fn just_true(rect: &Rectangle) -> bool {
        true
    }
}

// Multiple impl blocks
impl Rectangle {
    fn take(self) {}
}

fn main() {
    // instantiate a struct
    let mut user1 = User {
        username: String::from("alelopezperezz"),
        email: String::from("malejlopez@hotmail.com"),
        active: false,
        sign_in_count: 3,
    };
    user1.sign_in_count = 0;

    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);

    println!(
        "{}",
        build_user(String::from("ale"), String::from("email")).email
    );

    let new_user_from_before = User {
        email: String::from("fromawhaw@gmail.com"),
        ..user1
    };

    println!("{}", new_user_from_before.email);

    let c1 = Color(1, 340, 232);
    let p1 = Point(1, 323, 48390);
    println!("{}{}", c1.1, p1.0);

    let boo = AlwaysEqual;
    let st = String::from("asd");
    let w1 = 30;
    let h1 = 50;
    /// t_tup: (i32, String) = (123, st);
    //println!("{},{}", t_tup.1, st); errorause the string part of the tuple perfomed a move bec
    let t_tup: (i32, String) = (123, st.clone());
    println!("{},{}", t_tup.1, st);

    let rec1 = (w1, h1);
    println!("{}", area(w1, h1));

    println!("{}", area_tuple(rec1));

    let rect = Rectangle {
        width: 300,
        height: 500,
    };

    let area = area_struct(&rect);
    println!("{area}");

    println!("{:#?}", rect);

    // it took rect.take();
    println!("{}", rect.area());
    println!(
        "{}",
        rect.can_hold(&Rectangle {
            width: 1000,
            height: 40
        })
    );
    println!("{}", Rectangle::just_true(&rect));
}

//Init shorthand
fn build_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    };
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(rec: (u32, u32)) -> u32 {
    return rec.0 * rec.1;
}

fn area_struct(rec: &Rectangle) -> u32 {
    rec.height * rec.width
}
