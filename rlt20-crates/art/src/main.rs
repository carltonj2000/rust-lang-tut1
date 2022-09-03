mod art2;
mod lib;

fn main() {
    println!("Hello, world! {}", art2::add_one(41));
    lib::draw();
    if lib::mix(lib::PrimaryColor::Red, lib::PrimaryColor::Blue) == lib::SecondaryColor::Green {
        println!("Oj");
    } else {
        println!("what");
    }
}
