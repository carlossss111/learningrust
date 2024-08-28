
fn main() {
    // Strings can be created with ::new() or .to_string().
    // They can also be made of unicode fonts.
    {
        let _s = String::new();
        let _s = "こんにちは".to_string();
    }

    // push_str() appends to a String.
    {
        let mut s = String::from("foo");
        s.push_str("bar");
        println!("{s}");
    }

    // + operator concaternates. This is sugar for the .add() method.
    {
        let s1 = String::from("Hello ");
        let s2 = String::from("World");
        let s3 = s1 + &s2; // note s1 is moved here and cannot be used
        println!("{s3}");
    }

    // !format macro can be used to use a format string.
    {
        let s1 = String::from("Goodbye");
        let s2 = String::from("Universe");
        let s3 = format!("{s1}, {s2}...");
        println!("{}", s3);
    }

    // Rust does NOT support string indexing (bit cringe?)
    // This is because the length is depending on UTF8 or something.
    // There are workarounds using slices and references but it's
    // messy and the advice is to just avoid it.
    /*
    {
        let s1 = String::from("Hello");
        let h = s1[0]; //Illegal
    }
    */

    // Strings can be iterated over though.
    {
        for c in "Hello".chars() {
            println!("{c}");
        }
    }
}
