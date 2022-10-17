// Used to store blcks of code for re-use. 
/*
Docs: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
- main function is the entry point of many programms. 
- fn keyword allows to declare function. 
- Naming convention: snake_case
- Functions can be defined before or after the main function. Rust only cares
that they are defined somewhere in a scope that can be seen by caller. 

## Parameters ##
- Variables part of the f(x) signature.
- Concrete values of parameters are arguments. 
- In function signature, you must declare the type of each parameter.
- Parameters and arguments are separated by commas ','.

## Statements and Expressions ##
- Function bodies are made up of series of expressions optionally ending in an expression. 
- Statements: instructions that perform some action and do not return any value. 
- Expressions: evaluate to a resulting value.
- Variable assignments and function declarions don't return the assigned value, they are statements.
- Expression is when calling variables, functions, macros, etc. We end up with a return value. 
- A new block created with {} is an expression.
- Expressions do not include ';'.
- If you add ';' at the end of expression, you turn it into a statement, and it will then not return a value. 

## error[E0308]: mismatched types ##
- When adding ';' to the final function expression. 
- It is now turned into a statement so does not return any value and compiler let's you know about it:
    expected `i32`, found `()`
- There is a type mismatch, because it is expecting integer, yet finding unit (empty tuple). 
- Since nothing is returned, it contradicts the function definition and results in the error. 
- Rust compiler makes a suggestion to remove the ';':
    - help: remove this semicolon
- Compiler also provides more detailed info about the error and suggests to run:
rustc --explain E0308
*/
pub fn run() {
    println!("### Functions ###");
    greeting("Hello", "Jane");
    
    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Clousure -> with closures can use outside variables which we can't do with f(x)
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_nums(3, 3));

    println!("Calling function five(): {}", five());
}

fn greeting(greet: &str, name:&str) {
    println!("{} {}, nice to meet you!", greet, name);
}

/* ## Function with return value. ## 
- In rust the return value of the function is synonymmous with the value of the final
expression in the block of the body or function.
- To return early, use a keyword 'return' and specify value. 
- Most functions return the last expression implicitly. 
- 
 */
fn five() -> i32 {
    5
}


fn add(n1: i32, n2: i32) -> i32 {
    // If we don't use semicolon, that is telling what we want returned.
    n1 + n2
}