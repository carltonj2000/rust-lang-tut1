fn main() {
    ex1();
    i_map();
}

fn ex1() {
    let v1 = vec![1, 2, 4];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("got: {}", value);
    }
}

#[test]
fn ex2() {
    let v1 = vec![1, 2, 4];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&4));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn ex3() {
    let v1 = vec![1, 2, 4];

    let mut v1_iter = v1.into_iter();

    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(4));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn i_sum() {
    let v1 = vec![1, 2, 4];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 7);
}

fn i_map() {
    let v1 = vec![1, 2, 4];

    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("mapped {:?}", v2);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|s: &Shoe| s.size == shoe_size)
        .collect()
}

#[test]
fn filter_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn count_next() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

/*
 * 0 1
 * 1 2 2
 * 2 3 6 
 * 3 4 12
 * 4 5 20
 * 5 None
 */
#[test]
fn i_traits() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);
}
