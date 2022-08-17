use std::fs::File;
use std::io::ErrorKind;

fn main() {
    a();
    let name = "./readme.txt";
    let file = File::open(name);
    let file = match file {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(name) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => panic!("Problem opening file: {:?}", other_error),
        },
    };
    print!("{:?}", file);
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
