// Use `String` as our error type
type IResult<T> = Result<T, String>;

fn double_first(vec: Vec<&str>) -> IResult<i32> {
    vec.first()
       // Convert the `Option` to a `Result` if there is a value.
       // Otherwise, provide an `Err` containing this `String`.
       .ok_or("Please use a vector with at least one element.".to_owned())
       .and_then(|s| s.parse::<i32>()
                      // Map any errors that `parse` yields to `String`.
                      .map_err(|e| e.to_string())
                      // `Result<T, String>` is the new return type,
                      // and we can now double the number inside.
                      .map(|i| 2 * i))
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