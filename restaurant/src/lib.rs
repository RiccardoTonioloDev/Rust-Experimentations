mod front_of_house;

mod back_of_house {
    use crate::front_of_house::hosting;

    pub struct Breakfast {
        pub toast: String, //solo questo attributo sarà visualizzabile al di fuori
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

    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }
    fn cook_order() {
        hosting::add_to_waitlist();
    }
}

pub fn eat_at_restaurant() {
    //path assoluto
    //crate::front_of_house::hosting::add_to_waitlist();

    //path relativo
    //front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
