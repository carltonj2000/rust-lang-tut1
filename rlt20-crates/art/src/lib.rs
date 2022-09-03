//! # Art2
//!
//! A library for modeling artistic concepts

pub fn draw() {
    println!("Drawing");
}

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    //! # Kinds
    //!
    //! A library for enums

    #[derive(PartialEq, Eq)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
    #[derive(PartialEq, Eq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use super::kinds::PrimaryColor;
    use super::kinds::SecondaryColor;
    /// Adds one to the number given
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 5;
    /// let answer = rlt20crates::add_one(arg);
    ///
    /// assert_eq!(6, answer);
    /// ```
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        if c1 == PrimaryColor::Red && c2 == PrimaryColor::Yellow {
            SecondaryColor::Orange
        } else {
            SecondaryColor::Purple
        }
    }
}
