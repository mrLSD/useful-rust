// Chapter 4.26 - `const` and `static`
// @link https://doc.rust-lang.org/book/const-and-static.html
pub const N: i32 = 5 + 1;

// Almost always, if you can choose between the two, choose const.

static NAME: &'static str = "Steve";
static mut AGE: i32 = 25;

pub fn test_static() {
    println!("\n=====\nConst and Static");
    // Access to change statuc values via unsafe
    unsafe {
        AGE += N;
        // We must use unsage for mutable statuc
        println!("Name: {} [Age: {}]", NAME, AGE);
    }
    // Not mutable static without `unsafe`
    println!("Name: {}", NAME);
}