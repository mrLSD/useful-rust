// Chapter 4.12 - Structs
// [Chapter 4.12] https://doc.rust-lang.org/book/structs.html
pub mod structs {
    struct Point {
        x: i32,
        y: i32,
    }

    pub fn main() {
        let origin = Point{x: 10, y: 20}; // origin: Point
        println!("The origin is at ({}, {})", origin.x, origin.y);

        let mut origin = Point{x: 10, y: 20}; // origin: Point
        origin.x = 30;
        println!("The origin is at ({}, {})", origin.x, origin.y);
    }
}