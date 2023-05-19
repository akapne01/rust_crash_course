/// https://doc.rust-lang.org/book/ch06-03-if-let.html
/// Code snipppets 1 and 2 are the same.

pub fn run() {
    // Code Snippet 1.
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Code Snippet 2.
    // takes a pattern and expression separted by an equal sign. It works the same way as match
    // and the patter is it's first arm.
    // Pattern = Some(max) and the max binds to value inside tge Some. We can use max in the body
    // of if let blcok.
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let num: i32 = 5;
    match num {
        1 => println!("One"),
        2 | 3 => println!("Two or Three"),
        4 | 5 | 6 => println!("Four or Five or Six"),
        7 => println!("Seven"),
        _ => println!("Invalid number"),
    };
}
