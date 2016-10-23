// Since closures may be used as arguments, you might
// wonder if the same can be said about functions. And
// indeed they can! However, because a function can never
// capture variables, closures are strictly more flexible.
// Therefore, any function which can take a closure as an
// argument can also take a function.

// Define a function which takes a function as an argument and calls it.
fn call_function<F: Fn()>(f: F) {
    f()
}

// More explicite declaration
fn call_function_2<F>(f: F) where
    F : Fn() {
    f()
}

// Define a simple function to be used as an input.
fn print() {
    println!("I'm a function!")
}

pub fn main() {
    // Define a closure similar to the `print()` function above.
    let closure = || println!("I'm a closure!");

    call_function(&closure);
    call_function(print);

    call_function_2(&closure);
    call_function_2(&print);

    // As an additional note, the Fn, FnMut, and FnOnce traits
    // dictate how a closure captures variables from the
    // enclosing scope.
}