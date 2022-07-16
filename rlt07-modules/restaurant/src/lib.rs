mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order()
    }
    fn cook_order() {
        //
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer(toast: "Rye");
    meal.toast = String::from("Wheat");

    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Salad;
}

use std::fmt;
use std::io;
use std::io::Result as IoResult;

fn f1() -> fmt::Result {
    //
}

fn f2() -> io::Result<()> {
    //
}

fn f3() -> IoResult<()> {
    //
}