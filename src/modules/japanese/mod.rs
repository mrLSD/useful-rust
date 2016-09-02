// Re-exporting with pub use
// for shortcust functions at exported modules
//      rust_tutorials::modules::japanese::hello
//      rust_tutorials::modules::japanese::goodbye
// instead:
//      rust_tutorials::modules::japanese::greetings::hello
//      rust_tutorials::modules::japanese::farewells::goodbye
pub use self::greetings::hello;
// We could alternatively use the wildcard syntax
// // to include everything
pub use self::farewells::*;

pub mod greetings;
pub mod farewells;