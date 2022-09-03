#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping '{}'", self.data);
    }
}



fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff")};
    let d = CustomSmartPointer { data: String::from("other stuff")};
    println!("drop {:?}", c);
    drop(c);
    println!("drop {:?}", d);
}
