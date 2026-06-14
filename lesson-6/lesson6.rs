// Lesson 6: Generics and Traits
// To compile and run: rustc lesson6.rs && ./lesson6.exe

fn main() {
    println!("=== Lesson 6: Generics and Traits ===");

    // GENERICS - Write code that works for many types
    // <T> is a type placeholder filled in when the function is called.
    println!("\n--- Generic Functions ---");

    // largest works for any type that can be compared (see the where clause)
    println!("Largest i32: {}", largest(&[3, 7, 2, 9, 4]));
    println!("Largest char: {}", largest(&['a', 'z', 'm']));

    // GENERIC STRUCTS - A struct that holds any type
    println!("\n--- Generic Structs ---");

    let int_pair = Pair { first: 1, second: 2 };
    let str_pair = Pair { first: "hello", second: "world" };

    println!("int_pair: {:?}", int_pair);
    println!("str_pair: {:?}", str_pair);

    // TRAITS - Shared behavior that types can implement
    // A trait is like an interface: it defines methods a type must provide.
    println!("\n--- Traits ---");

    let dog = Dog { name: String::from("Rex") };
    let cat = Cat;

    // Both implement Animal, so both have .speak() and .name()
    println!("{} says {}", dog.name(), dog.speak());
    println!("{} says {}", cat.name(), cat.speak());

    // TRAITS AS PARAMETERS - Accept "any type that implements Animal"
    // This is polymorphism without inheritance.
    println!("\n--- Trait Bounds ---");

    describe(&dog);
    describe(&cat);

    // DEFAULT METHODS - A trait can provide a default implementation
    println!("\n--- Default Trait Methods ---");

    // Cat uses the default introduce(); Dog could override it.
    println!("{}", dog.introduce());
    println!("{}", cat.introduce());

    println!("\n--- End of Lesson 6 ---");
    println!("Next: Collections (Vec, HashMap) and iterators");
}

// GENERIC FUNCTION
// <T: PartialOrd + Copy> means: T must support comparison (>) and be copyable.
// These are "trait bounds" — constraints on what T is allowed to be.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut biggest = list[0];
    for &item in list {
        if item > biggest {
            biggest = item;
        }
    }
    biggest
}

// GENERIC STRUCT
// #[derive(Debug)] auto-generates the {:?} formatting for us.
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

// TRAIT DEFINITION
// Any type implementing Animal must provide name() and speak().
trait Animal {
    fn name(&self) -> String;
    fn speak(&self) -> String;

    // Default method: types get this for free unless they override it.
    fn introduce(&self) -> String {
        format!("I am {} and I say {}", self.name(), self.speak())
    }
}

struct Dog {
    name: String,
}

struct Cat;

// IMPLEMENTING A TRAIT FOR A TYPE
impl Animal for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn speak(&self) -> String {
        String::from("Woof")
    }
}

impl Animal for Cat {
    fn name(&self) -> String {
        String::from("a cat")
    }
    fn speak(&self) -> String {
        String::from("Meow")
    }
}

// TRAIT BOUND AS A PARAMETER
// "&impl Animal" = a reference to any type that implements Animal.
fn describe(animal: &impl Animal) {
    println!("This animal is called {}", animal.name());
}

// LESSON 6 SUMMARY:
// - Generics (<T>) let one piece of code work for many types
// - Trait bounds (T: PartialOrd) constrain what a generic type must support
// - A trait defines shared behavior (methods) types can implement
// - impl Trait for Type provides that behavior for a specific type
// - Traits can have default methods, overridable per type
// - &impl Trait accepts any type implementing the trait (polymorphism)
// - #[derive(Debug)] auto-generates {:?} formatting
