#![allow(unreachable_code)]

pub fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
    loop_nested_labeles();
}

fn loop_nested_labeles() {
    let mut i = 0;
    'outer: loop {
        i += 1;
        println!("Entered the outer loop [{}]", i);

        'inner: loop {
            println!("Entered the inner loop [{}]", i);
            if i <= 2 {
                continue 'outer;
            }
            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }
        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}