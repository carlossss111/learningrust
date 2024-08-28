// Vectors are defined in the prelude, so do not need to be imported

fn main() {
    
    // Empty vectors can be created with the type.
    {
        let _v: Vec<i32> = Vec::new();
    }

    // The 'vec!' macro can be used to initialise a new vector.
    {
        let _v = vec![1, 2, 3];
    }

    // Vectors can be pushed to, here i32 is inferred
    {
        let mut v = Vec::new();
        v.push(10);
        v.push(20);
    }

    // Vectors can be accessed with either references or options.
    // The first method has bounds checking and will just panic on fail
    // The second gives you an option of what to do on fail.
    {
        let v = vec![0, 1, 2, 3, 4];

        let third: &i32 = &v[2];
        println!("The third element is {third}");

        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element"),
        }
    }

    // The copy can only be copied, not referenced in this example,
    // otherwise we break the borrowing rules as .push() could
    // move the underlying heap data to another location.
    {
        let mut v = vec![0, 1, 2, 3, 4];
        let first = v[0];
        // let first = &v[0]; // Rules broken, compiler fail
        v.push(5);
        println!("The first element is: {first}");
    }

    // Immutable and mutable references to the vector can be iterated
    // over.
    {
        let mut v = vec![1, 2, 3];
        for i in &mut v {
            *i += 10; // Dereference operator woaw
        }

        for i in &v {
            println!("{i}");
        }
    }

    // Enums can be used to store multiple types in a vector.
    /*
    {
        enum Cell {
            Int(i32),
            Float(f64),
        }
        let row = vec![Cell::Int(3), Cell::Float(1.1)];
    }
    */

    // Vectors are dropped when they leave the scope
    {
        let v = vec![0, 1, 2, 3];
    }// drop() called here on the vector and all it's elements
}
