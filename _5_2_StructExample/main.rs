#[derive(Debug)] //Outer attribute to make the the struct printable
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    // Reminder that this function borrows, not takes ownership
    // because it is an immutable reference.
    rectangle.width * rectangle.height
}

fn main() {
	
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area: {}", area(&rect1));
    println!("{rect1:?}");  // regular debug output
    println!("{rect1:#?}"); // pretty debug output
    dbg!("{rect1:?}");      // stderr debug output
}
