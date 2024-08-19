// Has parameters and return type
fn sum(a:i32, b:i32) -> i32 {
    return a + b;
}

// Tails are where the last expression in a function has no semicolon so is returned
fn sum_with_tail(a:i32, b:i32) -> i32 {
    a + b
}

fn main() {
    println!("{}\n{}", sum(2,2), sum_with_tail(2,2));
}