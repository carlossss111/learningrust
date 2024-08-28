// Hashmaps are not part of the prelude
use std::collections::HashMap;

fn main(){
    // Hashmaps can be created with ::new().
    // They can be accessed with .get(), returning an Option<&V>.
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        
        // This will copy the Option<&V> to Option<i32>,
        // and then unwrap to an i32 or to 0 if there is no entry.
        let score = scores.get("Blue").copied().unwrap_or(0);
        println!("Blue Team's Score: {score}");

        // We can iterate over the hashmap like so.
        // This is in an arbitrary order.
        for (key, value) in &scores {
            println!("{key}: {value}");
        }
    }

    // Non-primitive types that don't implement the Copy trait
    // are moved and the hashmap becomes the owner.
    {
        let field_name = String::from("Favourite Colour");
        let field_value = String::from("Blue");
        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name and field_value are now invalid from this point.
    }
        
    // Values can be overwritten simply with .insert().
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 20);
        println!("{scores:?}"); //10 has been overwritten to 20.
    }

    // or_insert() is used to only insert if a key doesn't exist.
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        // 50 will be inserted for yellow, but not for blue.
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        println!("{scores:?}");
    }
}
