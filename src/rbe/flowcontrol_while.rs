pub fn main() {
    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 6 {
        if n % 2 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("else fizzbuzz: {}", n);
        }

        // Increment counter
        n += 1;
    }
}