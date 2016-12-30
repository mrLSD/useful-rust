// Use `String` as our error type
type IResult<T> = Result<T, String>;

fn double_first(vec: Vec<&str>) -> IResult<i32> {
    let first = try!(vec.first()
        .ok_or("Please use a vector with at least one element.".to_owned()));

    let value = try!(first.parse::<i32>()
        .map_err(|e| e.to_string()));

    Ok(2 * value)
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
