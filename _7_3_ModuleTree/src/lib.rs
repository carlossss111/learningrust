
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        
        fn _seat_at_table() {}
    }

    mod serving {
        fn _take_order() {}
        
        fn _serve_order() {}

        fn _take_payment() {}
    }
}

fn _deliver_order() {}

mod back_of_house {
    fn _fix_incorrect_order() {
        _cook_order();
        // The super keyword can be used to call things from the
        // parent module, as is done here.
        super::_deliver_order();
    }

    fn _cook_order() {}

    // Structs and enums can be made public, but their fields will
    // remain private by default.
    pub struct _Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
}

pub fn eat_at_restaurant() {
    // This is an absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

