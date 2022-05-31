#[allow(dead_code)]

#[allow(dead_code)]
mod back_of_house {
    // making struct and enum public
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruits: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { toast: String::from(toast), 
                        seasonal_fruits: String::from("apples")}
        }
        
    }
    fn fix_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

}

mod front_of_house;

#[allow(dead_code)]
mod customer {
    pub fn eat_at_restuarant() {
        use crate::front_of_house::hosting; 
        hosting::add_to_waitlist();
        use crate::back_of_house::{Breakfast, Appetizer};
        

    
        // order a breakfast in the summer with rye toast
        let mut order = Breakfast::summer("Rye");
        order.toast = String::from("Wheat");
        println!("I'd like {} toast please!", order.toast);
    
        let order1 = Appetizer::Salad;
        let order2 = Appetizer::Soup;
    
    }
}


fn deliver_order() {}
