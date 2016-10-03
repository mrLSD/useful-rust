#[derive(Debug)]
struct NewType<T> {
    x: i32,
    y: f32,
    z: T,
}

// `NanoSecond` is a new name for `u64`.
// Types must have CamelCase names
type NanoSecond = u64;
type Inch = u64;
type Coordinates1 = NewType<i32>;
type Coordinates2<T> = NewType<T>;

// Use an attribute to silence warning.
#[allow(non_camel_case_types)]
type u64_t = u64;
// TODO ^ Try removing the attribute

pub fn main() {
    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;
    let coord1: Coordinates1 = Coordinates1{x: 1, y: 2.1, z: 3};
    let coord2: Coordinates2<f32> = Coordinates2{x: 1, y: 2.1, z: 3.1};

    println!("Struct alias for Coord<i32> 1: {:?}", coord1);
    println!("Struct alias for Coord<f32> 2: {:?}", coord2);

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}