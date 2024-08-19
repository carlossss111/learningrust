fn main() {
    /*
    There are signed and unsigned ints of different types.
    The default is i32.
    Length	Signed	Unsigned
    8-bit	i8	    u8
    16-bit	i16	    u16
    32-bit	i32	    u32
    64-bit	i64	    u64
    128-bit	i128	u128
    */
    /* ints can overflow in release mode, but in debug mode
    they cause a runtime panic */
    {
        let _x:i8 = 127; // 8 bit signed int
        let _y = 127; // 32 bit signed int
    }

    /* There are only f32 and f64 for floating point nums */
    {
        let _x:f32 = 2.0;
    }

    /* There are true and false boolean values */
    {
        let _x:bool = true;
    }

    /* Chars are 4 bytes in length, representing a unicode scalar value */
    {
        let _x:char = 'a';
    }

    /* Tuples can be of different types.
    They cannot change in size after being declared. */
    {
        let tup: (i32, f64, u8) = (500, 6.4, 1);

        // x = 500, y = 6.4, z = 1
        // with
        let (_x,_y,_z) = tup;
        // or
        let _x = tup.0;
        let _y = tup.1;
        let _z = tup.2;
        
    }

    /* Stack data of fixed length. Declared with the type / val followed by
    the number */
    /* *** THE RUNTIME DOES BOUNDS-CHECKING AND THROWS PANIC *** */
    /* If it is obvious, the compiler may catch this too */
    {
        let _arr:[i32; 3] = [1,2,3];
        let _arr = [1; 3]; // = [1,1,1]
        let _arr = [1,2,3];

        let _first = _arr[0];
    }

    println!("Done.");
}