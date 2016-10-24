// 11.1 dead_code
// This signature allow attribute to all module (#!):
//       `#![allow(dead_code)]`
// The compiler provides a dead_code lint that
// will warn about unused functions. An attribute
// can be used to disable the lint.

fn used_function() {}

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

#[allow(dead_code)]
fn noisy_unused_function() {}
// FIXME ^ Add an attribute to suppress the warning

pub fn main() {
    used_function();
}