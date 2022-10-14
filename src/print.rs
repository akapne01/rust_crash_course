pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic formating
    println!("{} is from {}", "Alice", "Wonderland");

    // Positional Arguments
    println!("{0} is from {1} and {0} like to {2}", "Alice", "Wonderland", "code");

    // Named Arguemnts
    println!("{name} likes to play {activity}",
    name = "Alice",
    activity = "basketball");

    // Placeholder traits
    println!("Binary {:b}; Hex: {:x}; Octal {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}