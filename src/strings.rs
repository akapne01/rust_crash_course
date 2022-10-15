/*
Primitive str = Immutable fixed-length string somewhere in memory
String = Growable, heap-allocated data structure. Use when you need 
to modify your own string data. 
*/

pub fn run() {
    let hello = "Hello ";
    let mut hello2 = String::from(hello);

    // Get length
    println!("Length: {}.", hello.len());

    // Push a char
    hello2.push('W');
    
    // Push a string
    hello2.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello2.capacity());

    // Check if string is empty
    println!("Is Empty: {}", hello2.is_empty());

    // Contains a substring? 
    println!("Contains 'World': {}", hello2.contains("World"));

    // Replace
    println!("Replace: {}", hello2.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello2.split_whitespace() {
        println!("Loop: {}", word);
    }

    // Create string with certain capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

     // Assertion testing
     assert_eq!(2, s.len());
     assert_eq!(10, s.capacity());

    println!("{s}");

   

    // Print out the result
    println!("{:?}", (hello, hello2));
}