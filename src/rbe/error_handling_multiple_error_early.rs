// Use `String` as our error type
type IResult<T> = Result<T, String>;

fn double_first(vec: Vec<&str>) -> IResult<i32> {
   // Convert the `Option` to a `Result` if there is a value.
   // Otherwise, provide an `Err` containing this `String`.
    let first = match vec.first() {
        Some(first) => first,
        None => return Err("Please use a vector with at least one element.".to_owned())
    };

    // Double the number inside if `parse` works fine.
    // Otherwise, map any errors that `parse` yields to `String`.
    match first.parse::<i32>() {
        Ok(i) => Ok(2 * i),
        Err(e) => Err(e.to_string()),
    }
}

fn print(result: IResult<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn main() {
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(empty));
    print(double_first(strings));
}