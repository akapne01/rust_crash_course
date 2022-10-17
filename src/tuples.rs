/*
Tuples group together values of different types
Max 12 elements.
Documentation: https://doc.rust-lang.org/book/ch03-02-data-types.html
Tuples:
- Have fixed length, can't be resized.
- Eaqch position in a tuple has a type and can be different. 
The tuple without any values is a unit. This value and its
corresponding type are both written () and represent an empty
return type. Expressions implicitly return the unit value if they 
don't return any other value. 
*/

pub fn run() {
    println!("### Tuples ###");
    let person: (&str, &str, i8) = ("Alice", "Wonderland", 37);

    println!("{} is from {} and is {}", person.0, person.1, person.2);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Performs destructuring, because it breaks a single tuple into 3 parts.
    let (x, y, z) = tup;
    println!("The value of x is {x}");
    println!("The value of y is {y}");
    println!("The value of z is {z}");

    // Unit
    let un = ();
    println!("An empty tuple, or unit: {:?}", un);
}