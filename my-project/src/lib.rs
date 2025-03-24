mod front_of_house;

mod back_of_house {
    pub enum Appetizer {
        Salad,
        Soup,
    }

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

        pub fn start_preparing(&self) {
            println!(
                "Oye! Toast with {} bread and {}!!!",
                self.toast, self.seasonal_fruit
            )
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

// Bringing hosting to scope using `use` keyword
use crate::front_of_house::hosting;

mod customer {
    // above `use` is no longer used in this scope so with need to
    // add here as well
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

pub fn eat_at_restaurant() {
    // Paths
    //////////////////////////////////////////////////////

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // hosting is in scope because we brought it in with `use` above
    hosting::add_to_waitlist();

    // Public and private Structs and Enums
    //////////////////////////////////////////////////////

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Changed mind about bread type
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    back_of_house::Breakfast::start_preparing(&meal);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
