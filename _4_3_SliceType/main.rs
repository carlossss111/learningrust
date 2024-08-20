fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    /*The string can be sliced with the array notation.
    Here 'hello' references indices 0-5 of the string.
    This can be done dynamically, e.g. slice = &s[..s.len()]*/
    {
        let s = String::from("hello world");
        let hello = &s[0..5];
        let world = &s[6..11];
        println!("{}\n{}", hello, world);
    }

    /*Usage Example
    Note that slicing a String returns a str literal*/
    {
        println!("{}", first_word(&String::from("goodbye universe")));
    }

    /*The compiler will protect against the string being dropped.*/
    {
        let s = String::from("hello world");
        let word = first_word(&s);
    
        //s.clear(); // THIS WOULD THROW A COMPILE-TIME ERROR
        println!("the first word is: {word}");
    }

    /*String literals and arrays can all be sliced.*/
    {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(slice, &[2, 3]);
    }
}