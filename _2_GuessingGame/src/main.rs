// Uses stdio library.
// There is also a hidden library called 'Prelude' that is always imported.
use std::io;

// Rand has been added in Cargo.toml
use rand::Rng;

// Include order enum
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Call 'rand::thread_rng()' func to get a rng handle.
    // '1..=100' is an inclusive range expression.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");

    // Loop infinitely until encountering a break clause.
    loop {
        println!("Please input your guess.");

        // Creates a variable called 'guess'.
        // This is mutable. A new String type (UTF-8) is bound to it.
        // The '::' syntaxs indicates 'new()' is an associated func of String.
        let mut guess = String::new();
        
        // Could be written as 'std::io::stdin' if there is no 'use std::io'
        // 'io::stdin()' returns a handle to stdin.
        // '.read_line()' is a member of stdin. '&' passes 'guess' as reference.
        // References are immutable by default, hence 'mut' is written.
        // '.read_line()' returns a 'Result' enum with 'Ok' or 'Err'.
        // Values on the 'Result' type have the '.expect()' method on them, which
        // stops the program with an error message.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        
        // Parse string into a unsigned int 32
        // Also note the new 'guess' variable shadows the old one in this scope.
        // We could use an '.expect()' here too to end the program, instead
        // we can use arms (see matcher below) to do something depending on return val.
        let guess:u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
        };

        // '{}' is a placeholder for 'guess'.
        println!("You guessed: {}", guess);

        // 'cmp()' compares 2 values 
        // 'match' decides what to do with result and is made up of arms.
        // It will check each arm's pattern (like a switch case) and executes
        // the associated code when matching.
        // The break clause breaks from the surrounding loop.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        }
    }
}
