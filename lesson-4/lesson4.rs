// Lesson 4: Structs and Enums
// To compile and run: rustc lesson4.rs && ./lesson4.exe

fn main() {
    println!("=== Lesson 4: Structs and Enums ===");

    // STRUCTS - Group related data together
    println!("\n--- Structs ---");

    // Define a struct with 'struct' keyword (see bottom of file)
    // Create an instance by filling in all fields
    let user = User {
        name: String::from("Alice"),
        age: 30,
        active: true,
    };

    println!("Name: {}", user.name);
    println!("Age: {}", user.age);

    // Struct update syntax - copy most fields from another instance
    let user2 = User {
        name: String::from("Bob"),
        ..user  // fill remaining fields from 'user'
    };
    println!("User2: {} (age {})", user2.name, user2.age);

    // Methods on structs (defined with 'impl')
    println!("\n--- Struct Methods ---");

    let rect = Rectangle { width: 10, height: 5 };
    println!("Area: {}", rect.area());
    println!("Is square: {}", rect.is_square());

    // Associated function (no 'self') - called with ::
    let square = Rectangle::square(4);
    println!("Square area: {}", square.area());

    // ENUMS - A type that can be one of several variants
    println!("\n--- Enums ---");

    // Enums can hold data — each variant can have different data
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Text(String::from("hello"));

    print_message(msg1);
    print_message(msg2);
    print_message(msg3);

    // Option<T> - Rust's built-in enum for "maybe a value"
    // It's either Some(value) or None — no null!
    println!("\n--- Option<T> ---");

    let found = find_first_even(&[1, 3, 4, 7]);
    let not_found = find_first_even(&[1, 3, 5]);

    // if let - cleaner than match when you only care about one variant
    if let Some(n) = found {
        println!("Found even: {}", n);
    }
    println!("Not found: {:?}", not_found); // prints None

    println!("\n--- End of Lesson 4 ---");
    println!("Next: Error handling with Result<T, E>");
}

// STRUCT DEFINITIONS

struct User {
    name: String,
    age: u32,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

// impl block attaches methods to the struct
impl Rectangle {
    // &self = borrow self (read-only)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    // Associated function (no self) - like a constructor
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

// ENUM DEFINITIONS

enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },   // named fields (like a struct)
    Text(String),               // tuple-style
}

fn print_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Text(s) => println!("Text: {}", s),
    }
}

fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

// LESSON 4 SUMMARY:
// - struct groups named fields into one type
// - impl adds methods; &self = read, &mut self = modify, no self = constructor
// - enum defines a type that is exactly one of its variants
// - enum variants can carry different data per variant
// - Option<T> replaces null: Some(value) or None
// - match must cover every variant (exhaustive)
// - if let is shorthand when you only care about one variant
