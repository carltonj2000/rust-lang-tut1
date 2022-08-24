#![allow(dead_code)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}
pub fn greet(n: &str) -> String {
    format!("Hello {}", n)
}
// pub fn greet() -> String {
//     format!("Hello {}!", "there")
// }

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess must be greater than 1, got {value}")
        } else if value > 100 {
            panic!("Guess must be less than 100, got {value}")
        }
        Guess { value }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    #[should_panic]
    fn failing_test() {
        panic!("make this test fail");
    }
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger: Rectangle = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller: Rectangle = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn larger_cannot_hold_smaller() {
        let larger: Rectangle = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller: Rectangle = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2))
    }

    #[test]
    fn it_doesnot_add_three() {
        assert_ne!(5, add_two(2))
    }

    #[test]
    fn greet_correct_person() {
        let name = "CJ";
        let result = greet(name);
        // let result = greet();
        assert!(
            result.contains(name),
            "Greeting of {} did not contain {}",
            result,
            name
        )
    }

    #[test]
    #[should_panic(expected = "Guess must be less than 100, got 200")]
    fn greater_than_100() {
        Guess::new(200);
    }
    
    #[test]
    fn it_works_result_type() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

}
