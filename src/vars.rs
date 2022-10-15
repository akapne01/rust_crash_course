/* Variables hold primitive data or references to data.
 * Variables are immutable by default. 
 * Rust is a block-scoped language. 
 */
pub fn run() {
    let name = "Alice";
    let mut age = 37;
    age = 38;

    println!("My name is {} and I am {}.", name, age);
    println!("My name is {name} and I am {age}.");

    // Define constant. With const need to explicitly define a type.
    const ID: i32 = 001;
    println!("ID: {ID}");

    // Assign multiple variables at once. 
    let (my_name, my_age) = ("Alice", 37);
    println!("{my_name} is {my_age}.");
}