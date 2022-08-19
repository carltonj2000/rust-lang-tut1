fn main() {
    let x = 5;
    println!("x = {x}");
    let x = "6";
    println!("x = {x}");

    const SUBSCRIBER_COUNT: u32 = 100_000;
    println!("SUBSCRIBER_COUNT = {SUBSCRIBER_COUNT}");

    let tup = ("Carlton", 56);
    let (name, age) = tup;
    let n = tup.0;
    let a = tup.1;

    println!("n={n} a={a}");

    let _error_codes = [200, 404, 500];
    let _zero_arr = [0; 8];

    println!("{}", fn1(name, age));

    if age < 10 {
        println!("You are young.")
    } else {
        println!("You are old.")
    }

    let comment = if age > 55 {
        "getting close to retirement"
    } else {
        "keep on working"
    };
    println!("{}", comment);

    let mut i = 0;
    let mut result = loop {
        if i > 3 {
            break i;
        }
        println!("i = {i}");
        i += 1;
    };
    println!("Completed {result} loops.");

    while result > 0 {
        println!("result = {result}");
        result -= 1;
    }

    let na = [10, 22, 33, 44, 50];
    for elem in na {
        println!("elem = {elem}");
    }

    for number in 1..4 {
        println!("number = {number}");
    }
}

fn fn1(b: &str, a: u32) -> u32 {
    println!("Fn1 {} {}", a, b);
    a + 3
}
