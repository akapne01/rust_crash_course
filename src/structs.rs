// Structs - Used to create custom data types
/// A struckt or a structure is a cutom data type that let's you package together
/// and name multiple related values that make up a meaningful group.
/// Struck is like object's data attributes.
/// In struct, each pieace of data is named. It is more flexible than a tuple.
/// https://doc.rust-lang.org/book/ch05-01-defining-structs.html

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct ColorAsTuple(u8, u8, u8);

// Struct with functions
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct new person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set Last Name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}
// Define a struct. Struct definition is like a general template for the type.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct ColourTuple(i32, i32, i32);
struct Point(i32, i32, i32);

// struct AlwaysEqual;

pub fn run() {
    println!("### Structs ###");

    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;

    println!("Colour: {} {} {}", c.red, c.green, c.blue);

    let mut ct = ColorAsTuple(255, 0, 0);
    ct.0 = 200;
    println!("Colour: {} {} {}", ct.0, ct.1, ct.2);

    // Struct with Functions
    let mut p = Person::new("Mary", "Doe");
    println!("Person {} {}", p.first_name, p.last_name);
    p.set_last_name("Willams");
    println!("Person {}", p.full_name());

    println!("Person Tuple {:?}", p.to_tuple());

    // Define the User struct instance
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // Overrides the email address.
    user1.email = String::from("anotheremail@example.com");
    print_user_details(&user1);

    let user2 = build_user(
        String::from("email@example.com"),
        String::from("username123"),
    );
    print_user_details(&user2);

    // Below means set email as defined, copy all other values from user2.
    let user3 = User {
        email: String::from("user3email@example.com"),
        ..user2
    };
    print_user_details(&user3);
    // Struct moves data, so we can no longer use user2 here.

    // Using tuple structs without named fields to create different types
    let black = ColourTuple(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Black colour is {:?}", (black.0, black.1, black.2));
    println!("Origin point is {:?}", (origin.0, origin.1, origin.2));

    // Unit-Like Structs without any fields
    // let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    // Field init shorthand syntax
    // Instead of writing email = email, we just use email.
    // This works when parameter we want to set has the same name.
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user_details(user: &User) {
    println!("# User: ");
    println!("User email: {}", user.email);
    println!("User username: {}", user.username);
    println!("User active?: {}", user.active);
    println!("User sign in count: {}", user.sign_in_count);
    println!("");
}
