fn main() {
    /* Variables can only be changed if immutable. 
       Consts cannot be made immutable and must have an explicit data type. */
    {
        //let x = 1; // immutable, would not run
        let mut x = 1;
        println!("{}", x);
        x = 2;
        println!("{}", x);

        const Y : i32 = 3;
        println!("{}", Y);
    }

    /* Variables can be redeclared and redefined with 'shadowing'.
       This works between datatypes. Without the second 'let' keyword this
       would not compile.*/
    {
        let spaces = "    ";
        let spaces = spaces.len();
        println!("{}", spaces);
    }
}