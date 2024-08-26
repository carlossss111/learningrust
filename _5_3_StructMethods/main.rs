#[derive(Debug)] 
struct Rectangle {
    width: u32,
    height: u32,
}

// Everything inside the 'impl' is associated with the struct.
// '&self' is short for 'self: &Self', it is a parameter like any other.
// '&self' can be immutable/muttable, (rarely) owned or (often) borrowed.
// Methods are called 'associated functions' in rust.
impl Rectangle {
    // Constructors are not special, they just return Self types.
    // They are often called 'new', but this isn't baked into the
    // language, as shown below.
    // Associated functions WITHOUT self as a parameter can be called
    // with the :: syntax.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // Associated functions with self can be called with obj.func().
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Methods can have the same name as attributes
    // This is commonly done with getters.
    // Also note the arrow operator '->' is not used for dereferencing,
    // rust does automatic referencing and dereferencing based on context.
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area: {}", rect1.area());
    
    if rect1.width() {
        println!("Rectangle has non-zero width: {}", rect1.width);
    }

    let rect2 = Rectangle::square(3);
    dbg!("{:?}", rect2);
}
