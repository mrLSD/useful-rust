// Aliasing to declaration
extern crate rust_tutorials as tutor;

// Export multiple names from module `rust_tutorials::modules::english`
// for shortcut instesd of:
//      use phrases::english::greetings;
//      use phrases::english::farewells;
use tutor::modules::english::{self, greetings as en_greeting};
// it's similar to:
//      use tutor::modules::english::{self};
//      use tutor::modules::english::{greetings as en_greeting};

// Schortcut for all path:
//      tutor::modules::japanese::
use tutor::modules::japanese::*;

fn main() {
    // Shortcut declaration by aliasing
    println!("Hello in English: {}", en_greeting::hello());
    // Full path
    println!("Hello in English: {}", tutor::modules::english::greetings::hello());
    // Access by `self` declaration
    println!("Hello in English: {}", english::greetings::hello());

    // Full declaration
    println!("Goodbye in English: {}", tutor::modules::english::farewells::goodbye());
    // Access by `self` declaration
    println!("Goodbye in English: {}", english::farewells::goodbye());

    // Shoetcut declaration
    println!("Hello in Japanese: {}", tutor::modules::japanese::hello());
    // Full declaration
    println!("Goodbye in Japanese: {}", tutor::modules::japanese::farewells::goodbye());
    // From shortcur
    //      use tutor::modules::japanese::*;
    println!("Goodbye in Japanese: {}", farewells::goodbye());
    tutor::const_and_static::test_static();
    println!("About NAME: [{}]", tutor::const_and_static::NAME);
}
