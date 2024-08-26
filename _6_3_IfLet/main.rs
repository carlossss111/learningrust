// If-let syntax allows you to combine if and let in a less verbose way
// Consider:
fn verbose() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is {max}"),
        _ => (),
    }
}

// This behaves in the same way as above, trying the 'Some(max)' arm
// and otherwise doing nothing. It is just shorthand.
fn concise() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is {max}");
    }
}

// An else statement can also be included to cover the '_ => (do thing)'
// state.

fn main() {
    verbose();
    concise();
}
