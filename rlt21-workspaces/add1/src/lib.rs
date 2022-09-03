use rand::prelude::*;

pub fn add1(x: i32) -> i32 {
    x + 1 
}

pub fn add_rand(x: i32) -> i32 {
    let y: i32 = rand::thread_rng().gen();
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add1(2));
    }
}
