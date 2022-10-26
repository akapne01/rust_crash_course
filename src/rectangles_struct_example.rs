/// https://doc.rust-lang.org/book/ch05-02-example-structs.html
/// https://doc.rust-lang.org/book/ch05-03-method-syntax.html
/// Methods are defined within the context of a struct, and their first paramter
/// is always self, which represents the instance of the struct the method is being called on.
/// 'self' is used in the method signature only when the method transforms 'self' into something
/// else and you want to prevent the caller from using the original instance after the transformation.
/// '&mut self' would be a parameter, if we would want to write to the struct.
/// Method can have the same name as one of the struct's fields.
/// When you call a method with object.something(), Rust automatically adds in &, &mut, or * so object
/// matches the signature of the method. In other words, the following are the same:
///     p1.distance(&p2); is the same as (&p1).distance(&p2);
///  This automatic referencing behavior works because methods have a clear receiver—the type of self.
/// Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (&self),
/// mutating (&mut self), or consuming (self).
///  The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.
/// 
/// ### Structs ###
/// - Allows to define new types that are meaningul for your domain.
/// - Allows to group associated pieces of data together.
/// - in 'impl' blocks can define functions that are associated with your type, and methods are kind of associated function
/// that let you specify the behaviour that instances of your structs have. 
/// - Structs are not the only way to create custom types. There are enums as well. 

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Creates an implementation block.
// '&self' is short for: 'self: &Self'
// Method borrows the self object, so we need to use '&self'.
// We don't want to take ownership, we just want to read the data
// in the struct, not to write to it.
// All functions in 'impl' block are called associated functions becasue they are associated
// with the type named after impl. We can define assoc. f(x) that don't have 'self' as their
// first parameter (and thus are not methods) because they don't need an instance of the type
// to work with. Assoc. f(x) are often used for constructors that will return a new instance
// of a struct.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Each struct can have multiple impl blocks
impl Rectangle {
    fn print_hello() {
        println!("Hello from rectangle!");
    }
}

pub fn run() {
    println!("");
    println!("### Rectangles Struck Example ###\n");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // method syntax call goes after the obj, similar to Java.
    );

    // {:?} requires a Debug trait, which needs to be implemented explicity.
    // Adding annotation: #[derive(Debug)] above the struct allows to do that.
    println!("rect1 is {:?}", rect1);
    // {:#?} expands and formats the fields nicely. Useful when have more fields.
    println!("rect1 is {:#?}", rect1);

    // Can print using dbg! macro. It takes ownership of an expression as opposed to
    // println! macro that takes a reference.
    // dbg! prints the file name and line number of where that debug call occurs in your
    // code along with the resulting value of that expression, and returns the ownership
    // of the value.
    // dbg! prints to stderr console output stream.
    // println! prints to stdout console output stream.
    let scale = 2;
    let rect = Rectangle {
        // dbg! valid here, because it returns ownership
        width: dbg!(30 * scale),
        height: 50,
    };
    // we don't want dbg! to take ownership of rect2, so we pass a reference to &rect2 instead.
    dbg!(&rect);

    // Using functions associated with the struct:
    let sq = Rectangle::square(3);
    println!("Square created is {:#?}", sq);

    Rectangle::print_hello();
}
