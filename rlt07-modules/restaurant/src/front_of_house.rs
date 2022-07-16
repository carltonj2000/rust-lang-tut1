use rand::{Rng, CryptoRng, ErrorKind::Transient};
use std::io:{self, Write}; // * for all pub items instead of {...}

mod front_of_house {
    pub mod hosting;

    mod serving {
        fn take_order() {
            //
        }

        fn serve_order() {
            //
        }

        fn take_payment() {
            //
        }
    }
}

pub use crate::front_of_house::hosting;

pub fs eat_at_restaurant() {
    // absolute
    crate::front_of_house::hosting::add_to_wait_list();

    // relative
    front_of_house::hosting::add_to_wait_list();

    // scoped
    hosting::add_to_wait_list();
    
    let num = rand::thread_rng().gen_range(1, 11);
}


fn serve_order() {
    //
}
