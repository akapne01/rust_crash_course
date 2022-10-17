// Docs: https://doc.rust-lang.org/book/ch03-05-control-flow.html
// - loop keyword creates an infinite loop by default.
// break keyword allows to stop the loop.
// continue allows to skip code and jump to the next iteration.

/*
## Returning values from loops ##
- One use case of loop is to retry an operation that you know might fail.
- You can add value you want returned after the break expression you use to stop the loop.
- That value is returned out of the loop and can be used.

## Loop Labels to Disambiguate between multiple loops ##
- If you have loops within loops, break and continue apply to the innermost loop at that point.
- You can optionally specify a loop label on a loop that can use with break or continue to specify
that those keywords apply to the labeled loop instead of inermost loop.
- Loop lables must begin with a single quote that is not closed.
*/

pub fn run() {
    println!("### Loops ###");

    let mut count = 0;

    // Infinite loop if no break added.
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    }

    // While loop (FizzBuzz)
    /* Numbers from 1 to 100. FizzBuzz: if no divisible by 3, you want to print our fizz.
    If divisible by 5, print out Buzz
    If divisble by both, want to print our FizzBuzz.
    Oterwise just print a number.
    - Runs while condition is true oterwise exists the loop.
     */
    count = 1;
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }

    // For range loop
    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }

    // Loop through collection with while loop
    // This is slow, because the compiler adds tuntime code to perform conditional check 
    // weather index is within the bounds of arrary on every iteration throguh the loop. 
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    println!("# Looping through array #");
    while index < a.len() {
        println!("The value is : {}", a[index]);
        index += 1;
    }

    // Loop through collection using for loop
    // Increased safety of the code and eliminated chance for bugs that might result from missing items
    // as can occur when using a while loop. 
    println!("# Loop trhough array using for loop #");
    
    for element in a {
        println!("The value is: {element}");
    }

    // Return value from the loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value returned by the loop is: {result}");

    // Nested loop with labels
    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End Count of the labeled loop = {count}");
}
