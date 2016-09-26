/// Rust by examples modules
/// http://rustbyexample.com/

pub use self::rbe::*;
mod hello;
mod hello_comment;
mod hello_formatted_print;
mod hello_debug;
mod hello_display;

pub mod rbe {
    pub fn main() {
        super::hello::main();
        super::hello_comment::main();
        super::hello_formatted_print::main();
        super::hello_debug::main();
        super::hello_display::main();
    }
}