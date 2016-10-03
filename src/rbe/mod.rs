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
mod custom_types_enums;
mod custom_types_enums_use;
mod custom_types_enums_clike;
mod custom_types_enums_linkedlist;
mod custom_types_constants;

mod variable;
mod variable_mutability;
mod variable_scope;
mod variable_declare;

mod casting;
mod casting_literals;

mod scoping_rules_raii;
mod scoping_rules_ownership;

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
        super::custom_types_enums::main();
        super::custom_types_enums_use::main();
        super::custom_types_enums_clike::main();
        super::custom_types_enums_linkedlist::main();
        super::custom_types_constants::main();

        super::variable::main();
        super::variable_mutability::main();
        super::variable_scope::main();
        super::variable_declare::main();

        super::casting::main();
        super::casting_literals::main();

        super::scoping_rules_raii::main();
        super::scoping_rules_ownership::main();
    }
}