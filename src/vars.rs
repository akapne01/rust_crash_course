/* Variables hold primitive data or references to data.
 * Variables are immutable by default. 
 * Rust is a block-scoped language.
 * https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html 
 * Variables are made mutable by adding mut declaration.
 * Constants:
 * - always immutable
 * - declared using const keyword instead of let
 * - type of the value must be annotated
 * - can be declared at any scope. 
 * - may be set only to a constant expression, not a result of a value computed at runtime.
 * - use all uppercase with underscores between words
 */
pub fn run() {
    println!("### VARIABLES ###");
    let name = "Alice";
    let mut age = 37;
    println!("Age before reassignment: {}", age);
    age = 38;

    // Define a contant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Contant: {}", THREE_HOURS_IN_SECONDS);

    println!("My name is {} and I am {}.", name, age);
    println!("My name is {name} and I am {age}.");

    // Define constant. With const need to explicitly define a type.
    const ID: i32 = 001;
    println!("ID: {ID}");

    // Assign multiple variables at once. 
    let (my_name, my_age) = ("Alice", 37);
    println!("{my_name} is {my_age}.");

    // Variable Shadowing
    let a = 5; // binds value to 5
    let a = a + 1; // creates a new variable with value 6.
    {
        let a = a * 2; // shadows a var from the outer scope. Takes value from the outer scope and multiples by 2.
        println!("The value of a in the inner scope is: {a}");
    }
    // The a that had value 12 is out of scope here so the value of a is 6. 
    println!("The value of a is: {a}");

    /*
    Shadowing is different from marking a variable as mut. If we try to reassign value of an immutable var, we get
    compile error.
    By using let keyword we can perform a few transformation on a value but have the variable be immutable after
    those transformations have been completed.
    Using shadowing and deffining the same variable, we can change the type of the value, but reuse the same name. 
    */
}