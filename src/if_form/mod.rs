/// Chapter 4.5 - IF
/// [Chapter 4.5] https://doc.rust-lang.org/book/if.html
pub use self::if_form::*;
pub mod if_form {
    pub fn main(x: i32) {
        println!("IF statement");
        if x == 5 {
            println!("x is five!");
        } else if x == 6 {
            println!("x is six!");
        } else {
            println!("x is not five or six :(");
        }
        if_as_expression(x);
    }

    fn if_as_expression(x: i32) {
        let y = if x == 5 { 10 } else { 20 }; // i32
        println!("if-expression y is: {}", y)
    }
}