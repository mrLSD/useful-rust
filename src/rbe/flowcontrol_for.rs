pub fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // this include [1..5]
    for n in 1..6 {
        if n % 2 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("ELSE fizzbuzz: {}", n);
        }
    }
}