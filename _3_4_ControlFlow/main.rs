fn main() {

    /* There are if/else if/else statements.
    They are more explicit than C and do not have parantheses.
    The blocks within them are sometimes called arms.*/
    {
        let cond = false;
        let meaning = 42;
        if cond {
            println!("True.");
        }
        else if meaning == 42 {
            println!("The meaning of life, the universe, and everything is 42.")
        }
        else {
            println!("False.");
        }
    }

    /* Conditions can be used inline when defining variables.
    Each arm must have the same datatype. */
    {
        let number = if true { 5 } else { 6 };
        println!("Other num: {}", number);
    }

    /* Unconditional loops can be declared */
    {
        let mut i = 0;
        loop {
            print!("{} ", i);
            i+=1;
            if i == 4 { break };
        }
        println!();
    }

    /* Loops can have labels applied to them for clarity and for breaking */
    {
        let mut i = 0;
        'outer: loop {
            'inner: loop {
                if i == 4 { break 'outer };
                if i == 5 { break 'inner };
                i += 1;
            }
        }
    }

    /* While Loops can be pretty stock */
    {
        let mut i = 3;
        while i >= 0 {
            print!("{i} ");
            i -= 1;
        }
        println!();
    }

    /* For loops are python-like (yuck) */
    {
        let a = [10,11,12,13];
        for elem in a {
            print!("{elem} ");
        }
        println!();

        // Note how there is no 'let' for i.
        for i in -4..0 {
            print!("{i} ");
        }
        println!();

        for i in (-4..0).rev() {
            print!("{i} ");
        }
        println!();
    }
}