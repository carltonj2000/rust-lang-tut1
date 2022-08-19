#![allow(dead_code)]
fn main() {
    g1();
    g2();
    g3();
    g4();
}

fn g4 () {
    let i = Some(5);
    let f = Some(5.0);
    println!("{:?} {:?}", i, f);
}

fn g3 () {
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T,E> {
        Ok(T),
        Err(E),
    }
}

#[derive(Debug)]
struct Point <T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x 
    }
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f64, f64> {
    fn inc_y(&self) -> f64 {
        &self.y + 1.0
    }
}


fn g2 () {
    let p1 = Point{x:1, y:11};   
    let p2 = Point{x:1.2, y:11.34}; 
    let p3 = Point{x:10, y:14}; 
    let p4 = Point{x: 1.1, y: 'c'};
    let p14 = p1.mixup(p4);
    println!("{:?} {:?} {} {:?}", p2, p3, p2.inc_y(), p14);
}

fn g1 () {
    let list = vec![34, 50, 35, 100, 65];
    let largest = largest_item(list);
    println!("The largets number is {}", largest);
    let list = vec!['m', 'y', 'a', 'q'];
    let largest = largest_item(list);
    println!("The largets number is {}", largest);
}

fn largest_item<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
