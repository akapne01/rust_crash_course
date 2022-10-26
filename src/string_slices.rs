/// A String slice is a reference to part of a String.
/// Slice is a kind of reference, so it does not have onership.
pub fn run() {
    println!("### String Slices ###");

    let s = String::from("hello world");

    /* # Slice #
    specified in the extra [0..5] bit.
    We create slices using a range within brackets by specifying
    [starting_index..ending_index],
    where starting_index is the first position in the slice and
    ending_index is one more than the last position in the slice.
    Internally, the slice stores the starting position and length
    of the slice, which corresponds to the ending_index minus starting_index.
    So in the case of let world = &s[6..11];, world would be a slice that contains
    a pointer to the byte at index 6 of s with a length value of 5.
    With Rust’s .. range syntax, if you want to start at index zero,
    you can drop the value before the two periods.
    In other words, these are equal:
        let slice = &s[0..2];
        let slice = &s[..2];
    By the same token, if your slice includes the last byte of the String,
    you can drop the trailing number. That means these are equal:
        let len = s.len();
        let slice = &s[3..len];
        let slice = &s[3..];
    You can also drop both values to take a slice of the entire string.
    So these are equal:
        let len = s.len();
        let slice = &s[0..len];
        let slice = &s[..];
    Note: String slice range indices must occur at valid UTF-8 character boundaries.
    If you attempt to create a string slice in the middle of a multibyte character,
    your program will exit with an error.
    More info on how to handle it:
    https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
    */
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{:?}", (hello, world));

    let s = String::from("Hello World I'm learning Rust!");
    // Function first_word accepts &str. Can pass a reference to a string slice
    let word = first_word(&s[..]);
    let word_2 = first_word(&s[6..]);
    println!("The first word of the string '{s}' is '{word}'.");
    println!("The second word of the string '{s}' is '{word_2}'.");

    // Other Slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
