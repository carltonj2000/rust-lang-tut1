use std::fs::{self, File};
use std::io::{self};

fn main() {
    a();
    err_prop().expect("bad read");
    err_panic();
}

fn err_panic() {
    let name = "./readme.txt";
    let file = File::open(name).expect("Failed to open file.");
    println!("{:?}", file);
}

fn err_prop() -> Result<String, io::Error> {
    fs::read_to_string("./src/main1.rs")

    // simplified above
    // let mut s = String::new();
    // File::open("./src/main.rs")?.read_to_string(&mut s)?;
    // Ok(s)

    // let f = File::open("readme.txt");
    // the ? above does the same as this code here
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // same as ? and Ok(s) above
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
}
fn a() {
    b();
}
fn b() {
    c();
}
fn c() {
    d();
}
fn d() {
    e(21); // 22 to test panic
}
fn e(num: i32) {
    if num == 22 {
        panic!("don't pass 22!");
    }
}
