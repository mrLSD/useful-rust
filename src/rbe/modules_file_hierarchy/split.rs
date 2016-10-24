// split.rs
// This declaration will look for a file named `my.rs` or `my/mod.rs` and will
// insert its contents inside a module named `my` under this scope
use super::my as my;

fn function() {
    println!("called `function()`");
}

pub fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}
