/// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
/// In Option enum the value could be something, or it could be nothing.
/// If you contain first of the list containing values, you would get a value.
/// If you request the first item of an empty list, you would get nothing.
/// Compiler can check if you have handled all the cases you should be handling.
/// Rust doesn't have null or None values.
/// Null is a value that is invalid and is currently missing for some reason.

/// Enum in std library. It is included in the prelude, and doesn't require difinition.
/// T is a generic type paarameter.
/// In some enum can hold a piece of data of any type.
/// This enum is defined as follows:
/// enum Option<T> {
///     None,
///     Some(T),
/// }
///
/// When we have some, we know that value is present
/// When we have none, we now that the value is missing.
/// In order to have a value that can possibly be null, you must
/// explicitly opt in by making the type of that value Option<T>.
/// Then when you use that value, you are asked to explicitly handle
/// the case when the value is null. Everywhere that a value has a type
/// that isn't an Option<T>, you can safely assume that the value isn't null.
///
/// # match #
/// Requires all cases to be covered or it will not compile.
/// Matches in Rust are exhaustive: we must exhaust every last possibility in order for code
/// to be valid.
/// Especially, in case of Option<T>, we need to handle None case.
/// - Match pattern is evaluated in order. 
/// - _ match any value, and do not bind to that value.
/// - _ => () Translates to nothing happens. 
/// 
/// Using enums we can also take a special actions for a few particular values, but for all other
/// values take one default action.

#[allow(warnings)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // etc.
}
#[allow(warnings)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[allow(warnings)]
pub fn run() {
    println!("\n### Optional Enums ###");
    let some_number = Some(5);
    let some_char = Some('e');
    // Compiler can't infer the type that should be under Option
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // match y {
    //     Ok(y) => let sum = x + y

    // }
    // let sum = x + y;

    let cents = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("Value in cents: {cents}");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}
