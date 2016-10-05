// Each condition is followed by a block.
// if-else conditionals are expressions, and,
// all branches must return the same type.
pub fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let int_tst: i32 = 39 / 4;
    // if-else should return same tipe
    // i.e. let big: f32_n
    let big_n =
        if n < 5 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10. * n as f32
        } else {
            println!(", and is a big number, reduce by two");

            // This expression must return an `i32` as well.
            n as f32 / 2.
            // TODO ^ Try suppressing this expression with a semicolon.
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {} [{}]", n, big_n, int_tst);
}