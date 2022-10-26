pub fn run() {
    println!("### SLICE TYPE ###");
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("{word}");

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn first_word(s: &String) -> usize {
    // We need to go element by element to check weather value is a space.
    // Converts to an array of bytes
    let bytes = s.as_bytes();

    // We create an iterator over array of bytes
    // iter -> returns each element in a collection
    // enumerate -> wraps the result of iter and returns
    // each element as a part of tuple instead.
    // First element returned is index, and the second
    // is reference to the element.
    // Because enumerate returns a tuple, we can use patterns
    // to destructure that tuple.

    // If we find a byte literal string, we return the position.
    // Otherwise we return the length of that string.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
