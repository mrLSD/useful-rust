/// Rust by examples modules
/// http://rustbyexample.com/

pub use self::rbe::*;

mod hello;
mod hello_comment;
mod hello_formatted_print;
mod hello_debug;
mod hello_display;
mod hello_format;

mod primitives;
mod primitives_literals;
mod primitives_tuples;
mod primitives_arrays;

mod custom_types_structures;
mod scoping_rules_raii;

pub mod rbe {
    pub fn main() {
        super::hello::main();
        super::hello_comment::main();
        super::hello_formatted_print::main();
        super::hello_debug::main();
        super::hello_display::main();
        super::hello_format::main();

        super::primitives::main();
        super::primitives_literals::main();
        super::primitives_tuples::main();
        super::primitives_arrays::main();

        super::custom_types_structures::main();
        super::scoping_rules_raii::main();
    }
}