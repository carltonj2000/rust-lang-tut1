struct Rectangle(i32, i32);

#[derive(Debug)]
struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rect {
    fn square(size: i32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

fn main() {
    structs_ex();
    println!("Area = {}", area(Rectangle(2, 3)));
    let rect = Rect {
        width: 3,
        height: 4,
    };
    println!("For {:#?} Area = {}", rect, rect.area());
    let rect2 = Rect {
        width: 2,
        height: 4,
    };
    println!(
        "{:?} can {}hold {:?}",
        rect,
        if rect.can_hold(&rect2) { "" } else { "not " },
        rect2
    );
    println!("{:?}", Rect::square(2));
}

fn area(rect: Rectangle) -> i32 {
    rect.0 * rect.1
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn structs_ex() {
    let mut user1 = User {
        email: String::from("c@j.com"),
        username: String::from("c"),
        sign_in_count: 0,
        active: false,
    };
    println!("user {:?}", user1);
    user1.username = String::from("j");
    println!(
        "user {} {} {} {}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );
    let user2 = build_user("j@c.com".to_string(), "jj".to_string());
    println!("user2 {:?}", user2);

    let user3 = User {
        email: "b@b.com".to_string(),
        username: "bb".to_string(),
        ..user2
    };
    println!("user3 {:?}", user3);
}
fn build_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        sign_in_count: 0,
        active: false,
    };
}
