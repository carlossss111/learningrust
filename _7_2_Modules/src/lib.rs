// Define module with mod keyword, this groups definitions together.
/*
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
*/

// Code within a module is private from it's parent modules by default.
// To make it public use `pub mod`.

// Access modules with ::

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        
        fn serve_order() {}

        fn take_payment() {}
    }
}

