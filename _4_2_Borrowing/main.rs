fn calculate_length(s:&String) -> usize {
    s.len()
}

fn change(some_string:&mut String) {
    some_string.push_str(" World");
}

fn main() {
    /* In this ecample, a REFERENCE to s1 is given to the function.
    This means that the func does not take ownership of s1, and does not
    call drop() upon finishing.
    In short, s points to s1 which points to the String heap.
    This is called BORROWING.*/
    {
        let s1 = String::from("Hello");
        let len = calculate_length(&s1);
        println!("The length of {s1} is {len}.");
    }

    /* References can be defined as mutable.
    Note that you can only have ONE mutable reference to a val at a time,
    (as opposed to any number of immutable references)*/
    {
        let mut s = String::from("Hello ");
        change(&mut s);
        println!("{}",s);
    }

    /*
    THIS IS OK
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    THIS IS OK
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;

    THIS IS NOT OK
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    */
}