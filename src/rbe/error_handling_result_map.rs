use std::num::ParseIntError;

// With the return type rewritten, we use pattern matching without `unwrap()`.
fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
    match number_str.parse::<i32>() {
        Ok(n)  => Ok(2 * n),
        Err(e) => Err(e),
    }
}

// As with `Option`, we can use combinators such as `map()`.
// This function is otherwise identical to the one above and reads:
// Modify n if the value is valid, otherwise pass on the error.
fn double_number_map(number_str: &str) -> Result<i32, ParseIntError> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn main() {
    // This still presents a reasonable answer.
    let twenty = double_number("10");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = double_number_map("t");
    print(tt);

    fn x() -> Result<i32, i32> {
        println!("-> [1] call");
        Err(2)
    }
    fn y() -> Result<i32, i32> {
        println!("-> [2] call");
        Err(3)
    }
    // It invoke all functions
    let _ = x().and(y());
    fn go(i: i32) -> Result<i32, ()> {
        Ok(i)
    }
    fn go2(i: i32) -> i32 {
        i * 2
    }
    let res = go(4).map(go).map(|i| i.unwrap() + 3);
    println!("Map result: {:?}", res);

    let res = go(4).map(go2).map(|i| i + 3);
    println!("Map result: {:?}", res);
}