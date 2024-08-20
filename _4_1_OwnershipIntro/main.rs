/*
RULES
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    /* Primitive variable scope is trivial. The same as STACK variables in C */
    {                      // s is not valid here, it’s not yet declared
        let _s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    /* The String data type works with the HEAP so it can be mutated (unlike the literal
    above). 
    The value below would continue to exist in C, but in Rust it is freed when out of scope.
    The Rust runtime calls a special function, called drop().*/
        {
        let _s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no longer valid

    /* Here s1 contains a pointer to character array that is on the heap as above. When 
    s2 is assigned, it will move this pointer value to s2.
    Now there are two values pointing to the same value, so going out of scope could risk
    calling two drop() funcs, a double free.
    To prevent this, Rust considers s1 no longer valid, and only drop()s s2. s1 cannot
    be used anymore for this string, it has lost ownership.
    */
    {
        let s1 = String::from("hello");
        let s2 = s1;
    
        //println!("{s1}, world!"); //INVALID, NO OWNERSHIP
        println!("{s2}, world!"); //VALID, HAS OWNERSHIP
    }

    /* We can create a deep copy with .clone() on Strings. The data is copies over,
    and a new pointer is used for the clone. */
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
    
        println!("s1 = {s1}, s2 = {s2}");
    }

    /* Note that primitives can be copied because they are on the stack. 
    These actually work because they implement the 'Copy' trait. */
    {
        let x = 5;
        let y = x;
    
        println!("x = {x}, y = {y}");
    }

    /* Ownership works with functions like you'd expect. Heap values
    are moved. Stack values are copied. */
    {
        let s = String::from("hello");  // s comes into scope

        takes_ownership(s);             // s's value moves into the function...
                                        // ... and so is no longer valid here
    }
}

fn takes_ownership(s:String) {
    println!("{s}");
}