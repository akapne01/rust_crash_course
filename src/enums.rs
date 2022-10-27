// Enums are types which have a few definitve values
// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// Enums give a way of saying a value is one of a possible set of values.
// Enums allow to store any kinds of data. Can even store enum in the enum.

// 1. Moving Avatar
enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Perform action depending on info
    // Match works  similar to switch statement in Java
    match m {
        Movement::Up => println!("Avatar Moving Up"),
        Movement::Down => println!("Avatar Moving Down"),
        Movement::Left => println!("Avatar Moving Left"),
        Movement::Right => println!("Avatar Moving Right"),
    }
}

// 2. IP Addresses
// Data attached to the enum, so we don't need to creeate a struct
#[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 3. Message enum
#[derive(Debug)]
enum Message {
    Quit,                        // No data assoc.
    Move { x: i32, y: i32 },     // Has named fields similar to struct
    Write(String),               // Includes a single String
    ChangeColour(i32, i32, i32), // includes three i32 values
}

impl Message {
    fn call(&self) {
        println!("Message Called is {:?}", self);
    }
}

pub fn run() {
    println!("\n### ENUMS ###\n");
    // 1. Moving Avatars
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);

    // 2. IP Adresses
    let home = IpAddress::V4(127, 0, 0, 1);
    let loopback = IpAddress::V6(String::from("::1"));
    println!("\nHome: {:?}", home);
    println!("Loopback: {:?}", loopback);

    // 3. Message
    let m = Message::Write(String::from("hello"));
    m.call();
}
