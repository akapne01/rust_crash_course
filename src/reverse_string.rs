pub fn run() {
    let string_to_reverse = "Rust";
    let reversed_string = reverse(&string_to_reverse);

    println!("String: {string_to_reverse}");
    println!("Reversed String: {reversed_string}");
}

fn reverse(input: &str) -> String {
    let result = input.chars().rev().collect();
    result
}
