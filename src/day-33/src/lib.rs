pub use crate::front_of_house::hosting; // Yes because by using pub we're making it avaiable for other ti bring into their socpe. 
use std::collections::HashMap;
use std::fmt::Result as ResultFmt;

pub fn placeholder() -> ResultFmt {
    Ok(())
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1,2);

	    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries"); Because seasonal fruit is private the outside code cannot read it

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

mod front_of_house;

fn deliver_order() {}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
        pub enum Appetizer {
            Soup,
            Salad,
        }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

