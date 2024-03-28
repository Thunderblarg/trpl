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

    pub fn eat_at_restaurant() {
        crate::front_of_house::hosting::add_to_waitlist();

        // TODO: Hit the trpl discord and see why the book says to use
        // front_of_house::hosting::add_to_waitlist instead of how I have it
        // below, as it's not compiling for me as-is in the book
        hosting::add_to_waitlist();
    }
}

mod back_of_house {
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
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}


pub fn eat_at_restaurant() {
    
    
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


mod customer {
    use crate::front_of_house::hosting;
    
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

fn deliver_order() {}
// pub fn add(left: usize, right: usize) -> usize {
    //     left + right
    // }
    
    // #[cfg(test)]
    // mod tests {
        //     use super::*;
        
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
