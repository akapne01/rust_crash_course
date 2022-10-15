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
}
