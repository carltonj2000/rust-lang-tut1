#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    #[allow(dead_code)]
    fn some_function() {
        println!("Let's Get Rusty!")
    }
}

fn enums1() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let localhost = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("ip kinds {:?} {:?}", four, six);
    println!("ip address = {:?}", localhost);
    println!(
        "ip address {:?}, kind {:?}",
        localhost.address, localhost.kind
    );
    let localhost2 = IpAddrKind2::V4(127, 0, 0, 1);
    let localhost3 = IpAddrKind2::V6("1".to_string());

    println!("ip address {:?}", localhost2);
    println!("ip address {:?}", localhost3);
}

fn options1() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!(
        "num {:?} str {:?} absent {:?}",
        some_number, some_string, absent_number,
    );

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let z: Option<i8> = None;
    let sum = x + y.unwrap_or(1);
    println!("sum {sum}");
    let sum = x + z.unwrap_or(1);
    println!("sum {sum}");
}

#[derive(Debug)]
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    California,
    Nevada,
    // ...
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("From {:?}", state);
            25
        }
    }
}
fn match1() {
    println!("dime is worth {} cents", value_in_cents(Coin::Dime));
    println!(
        "quarter is worth {} cents",
        value_in_cents(Coin::Quarter(UsState::Nevada))
    );
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main() {
    enums1();
    options1();
    match1();

    let five = Some(5);
    println!("result = {:?}", plus_one(five));

    let val = Some(3);
    match val {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = val {
        println!("three!!")
    }

    if val == Some(3) {
        println!("Three.")
    }
}
