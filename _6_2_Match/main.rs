enum Coin {
    Penny,
    Two_Pence,
    Five_Pence,
}

// The match can be used with enums to execute associated arms.
fn value_in_pence(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Two_Pence => 2,
        Coin::Five_Pence => 5,
    }
}

// Match can be used with the std Option enum to see whether a value 
// returns null or not and handle it.
// This is a very common use case.
// Also note that match must be exhausted, all arms must be defined.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // Do nothing
        Some(i) => Some(i+1), // Return some(i+1)
    }
}

// Placeholders can be used for all previously undefined arms:
fn dice_roll(roll: i8) {
    match roll {
        3 => println!("Do thing"),
        7 => println!("Do other thing"),
        _ => println!("Do anything else"),
        // can also be:
        // _ => (),
        // or:
        // x => dothingwith(x),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
