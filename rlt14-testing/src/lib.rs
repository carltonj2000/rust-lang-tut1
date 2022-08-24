#![allow(dead_code)]

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    inner_add_two(a)
}

fn inner_add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }

    #[test]
    fn it_works2() {
        assert_eq!(2 + 2, 4);
    }

    // to set println below use cli option  -- --show-output
    #[test]
    fn pnr10_pass() {
        assert_eq!(10, prints_and_returns_10(4))
    }

    // run ignore test using cli option -- --ignored
    #[test]
    #[ignore]
    fn pnr10_fail() {
        assert_eq!(8, prints_and_returns_10(2))
    }

    // cargo test add // runs tests that start with add
    // cargo test tests // runs all tests in module test
    #[test]
    fn add_2to2() {
        assert_eq!(4, add_two(2))
    }

    #[test]
    fn add_3to2() {
        assert_eq!(5, add_two(3))
    }
    
    #[test]
    fn add_2to5i() {
        assert_eq!(7, inner_add_two(5))
    }

}
