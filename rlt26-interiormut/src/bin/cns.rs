use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(10));
    let a = Rc::new(Cons(
        Rc::new(RefCell::new(5)),
        Rc::new(Cons(Rc::clone(&value), Rc::new(Nil))),
    ));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("ref count a = {}", Rc::strong_count(&a));
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
