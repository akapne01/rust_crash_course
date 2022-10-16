use std::env;

pub fn run() {
    println!("### Read arguments from command line interface");
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Alice";
    let status = "100%";
    /* First argument is the target directory of the executable.

    Example. If running:
    cargo run hello world
    then prints:
    Args: ["target/debug/rust_crush_course", "hello", "world"]

     */
    println!("Args: {:?}", args);
    println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command.");
    }
}
