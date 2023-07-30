mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_the_house {

    pub enum Appetizer {
        Soup,
        Salad
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Mango")
            }
        }
    }

    pub fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {

    }
}

fn deliver_order() {}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path 
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // with use
    hosting::add_to_waitlist();

    let mut meal = back_of_the_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The following line will generate a compile time error since we are not supposed to 
    // access the seasonal fruit.
    // meal.seasonal_fruit = String::from("Peach");

    let order1 = back_of_the_house::Appetizer::Soup;
    let order2 = back_of_the_house::Appetizer::Salad;
}