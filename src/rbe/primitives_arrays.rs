use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

pub fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    println!("Array xs: {:?}", xs);

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the size of the array
    println!("array xs size: {}", xs.len());

    // Arrays are stack allocated
    println!("array xs occupies {} bytes", mem::size_of_val(&xs));

    println!("array ys size: {}", ys.len());
    println!("array ys occupies {} bytes", mem::size_of_val(&ys));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Out of bound indexing yields a panic
    //println!("{}", xs[5]);

    let ps: &[i32] = &[1, 3, 5, 6, 9];
    analyze_slice(&ps[1 .. 4]);
    println!("Slice: {:?}", &ps[1 .. 4]);
}