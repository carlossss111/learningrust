/* Struct declarations can always be mutable */
struct MyStruct {
    my_num : i32, // declare struct members with this notation
    my_str : String,
}

fn shorthand_example (my_num : i32) -> MyStruct{
    let strt = MyStruct {
        my_num,
        my_str : String::from("Test"),
    };
    strt
}

fn main() {
    /*Instantiate structs like so.
    Consider that when set as mutable, all members can be
    directly changed. */
    {
        let mut instance = MyStruct {
            my_num : 100,
            my_str: String::from("Hello World"),
        };
        println!("{}",instance.my_num);
        instance.my_num = 200;
        println!("{}",instance.my_num);
        println!("{}",instance.my_str);
    }

    /*Member variables can be populated with shorthand.
    See my_num in the func.*/
    {
        let instance : MyStruct = shorthand_example(42);
        println!("{}",instance.my_num);
    }

    /*Member variables can be MOVED (NOT COPIED [unless stack]) between
    structs.*/
    {
        let in1 = MyStruct {
            my_num : 42,
            my_str : String::from("Hello1"),
        };
        let in2 = MyStruct {
            my_num : in1.my_num, // copied (stack)
            my_str : in1.my_str, // moved (heap)
        };
        let in3 = MyStruct {
            ..in2 // as above
        };
        // println!("{} {}", in1.my_num, in1.my_str); // Compiler error - lost ownership of my_str.
        println!("{} {}", in3.my_num, in3.my_str);
    }

    /* Tuple structs exist. They are the same as with tuples except
    they declare the type of elements, not the elements themselves.*/
    {
        struct Color(i32, i32, i32);
        let col = Color(256,256,256);
        println!("{}, {}, {}", col.0, col.1, col.2);
        // VS
        let col = (256, 256, 256);
        println!("{}, {}, {}", col.0, col.1, col.2);
    }

}