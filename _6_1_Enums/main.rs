
// Below we see how an enum can be attached to a struct.

/*
// Enum allows us to say a value may be can only be of some versions.
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind, // Could be either of the enum values
    address: String,
}

fn main() {
    let _home = IpAddr {
        kind: IpAddrKind::V4, // Create enum with :: syntax
        address: String::from("127.0.0.1"),
    };
}
*/

// --------------------------
// Here we see how datatypes can be put straight into the enum itself,
// to make it more concise.

#[derive(Debug)]
enum IpAddr {
    V4(String), // Enum value is given a data type.
    V6(String), // Note that you can use any datatypes, even Structs, 
                // and as many parameters as necessary.
}

#[derive(Debug)]
enum Message {
    Quit, // More varied example here
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(i32, i32, i32),
}

// Enums can have associated functions too!
impl Message {
    fn call(&self) {
        println!("Called!");
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    println!("{:?}", home);

    let msg = Message::Write(String::from("Hello World"));
    msg.call();
}

/*
See discussion of the std Option<T> enum included in the literature.
*/

