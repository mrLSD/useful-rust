pub fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    let mut mutable = 12; // Mutable `i32`.
    assert_eq!(mutable, 12_i32);

    mutable = 14;

    assert_eq!(logical, true);
    assert_eq!(a_float, 1.0_f64);
    assert_eq!(an_integer, 5);
    assert_eq!(default_float, 3.0);
    assert_eq!(default_integer, 7);
    assert_eq!(mutable, 14_i32);
}
