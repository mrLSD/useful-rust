pub fn main() {
    // Declare a variable binding
    let a_binding;
    let b_binding;

    {
        let x = 2;

        // Initialize the binding
        a_binding = x * x;
    }
    // Initialize another the binding
    b_binding = a_binding * a_binding;

    println!("a binding: {}", a_binding);
    println!("b binding: {}", b_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    //println!("another binding: {}", another_binding);
    // FIXME ^ Comment out this line

    another_binding = 1;

    println!("another binding: {}", another_binding);
}