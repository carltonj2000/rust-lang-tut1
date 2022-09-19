use std::thread;
use std::time::Duration;

fn main() {
    ex1();
    ex2();
}

fn ex2() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    
    // drop(v);
    handle.join().unwrap();
}

fn ex1() {
    let hndl = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // hndl.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    hndl.join().unwrap();

}
