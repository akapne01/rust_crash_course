/* 
Primitive types of Rust
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
    - u: unsigned integers, no negative numbers
    - i: integers
    - Signed integers are stored using Two's Complement: 
    https://en.wikipedia.org/wiki/Two%27s_complement
    - Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses.
    - i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127. 
    - Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.
    - Additionally, the isize and usize types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
Floats: f32, f64
    - Floating-point numbers are represented according to the IEEE-754 standard. The f32 type is a single-precision float, and f64 has double precision.
Boolean (bool)
Characters (char)
    - Represents a Unicode Scalar Value. It can reperesent a lot more than just ASCII.
    - Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
Tuples
Arrays -> Fixed length 

Rust is statically types language, types must be known at compile time.
The Compiler can usually infer what type we want to use based on the value
and how we use it.

https://doc.rust-lang.org/book/ch03-02-data-types.html
Literals can use _ to make it easier to read. For example, 1_000. 
Rust defaults are generally i32. 
The primary situation in which you’d use isize or usize is when indexing some sort of collection.

## Integer Overflow ##
- Let’s say you have a variable of type u8 that can hold values between 0 and 255. If you try to change the variable to a value outside of that range, such as 256, integer overflow will occur, which can result in one of two behaviors.
- When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs. 
- Rust uses the term panicking when a program exits with an error.
- When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping.
- In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on.
- The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error.

## Numereic Operations ##
- Rust supports the basic mathematical operations you’d expect for all of the number types: addition, subtraction, multiplication, division, and remainder. 
- Integer division rounds down to the nearest integer.
- Operators and Symbols in Rust: https://doc.rust-lang.org/book/appendix-02-operators.html

## Compound Types ##
- Tuple
- Array
Both have a separate files in this program. 
*/
pub fn run() {
    // Default is i32
    let x = 1;

    // Default it f64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 45454545454545;

    // Find max size
    println!("Max i32 {}", std::i32::MAX);
    println!("Max i64 {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater = 10 < 5;

    // Character -> can be any unicode
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));

}