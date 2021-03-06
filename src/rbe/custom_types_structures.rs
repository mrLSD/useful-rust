// A unit struct
#[derive(Debug)]
struct Nil;

// A tuple struct
#[derive(Debug)]
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

pub fn main() {
    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;
    println!("point coordinates: ({}, {})", my_x, my_y);

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };
    println!("Rectangle: {:?}", _rectangle);

    // Instantiate a unit struct
    let _nil = Nil;
    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    println!("Unit struct {:?}", _nil);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    println!("pair struct {:?}", pair);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // Calculates the area of a rectangle
    let rect: Rectangle = Rectangle{
        p1: Point{x:-2.0, y: 3.2},
        p2: Point{x: 3.2, y:-1.8},
    };
    println!("Rectangle Area: {:?}", rect_area(&rect));
    println!("Rectangle Square: {:?}", square(&point, 3.4));
}

// Activity
// Calculates the area of a rectangle
fn rect_area(rect: &Rectangle) -> f32 {
    let mut width_x = rect.p1.x - rect.p2.x;
    if rect.p1.x < rect.p2.x {
        width_x = -width_x;
    }

    let mut width_y = rect.p1.y - rect.p2.y;
    if rect.p1.y < rect.p2.y {
        width_y = -width_y;
    }

    width_x * width_y
}

// Activity
// returns a Rectangle with its lower left corner on the point
fn square(p: &Point, w: f32) -> Rectangle {
    return Rectangle{
        p1: *p,
        p2: Point{x: p.x + w, y: p.y + w},
    };
}