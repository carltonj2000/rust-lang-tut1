use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io::stdin};

fn main() {
    let number: u8 = rand::thread_rng().gen_range(1..11);
    println!("{} {}", "Guess the number from 1-10!".yellow(), number);
    loop {
        println!("{}", "Please input your guess.".magenta());
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line.");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter only a number.");
                continue;
            }
        };

        match guess.cmp(&number) {
            Ordering::Equal => {
                println!("{}", "The correct number was guessed.".green());
                break;
            }
            Ordering::Greater => println!("{} {}.", "The number is lower than".blue(), guess),
            Ordering::Less => println!("{} {}.", "The number is higher than".red(), guess),
        };
    }
}
