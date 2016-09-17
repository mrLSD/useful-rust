// Chapter 4.12 - Structs
// [Chapter 4.12] https://doc.rust-lang.org/book/structs.html
pub mod structs {
    struct Point {
        x: i32,
        y: i32,
    }
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    struct PointRef<'a> {
        x: &'a mut i32,
        y: &'a mut i32,
    }

    pub fn main() {
        let origin = Point{x: 10, y: 20}; // origin: Point
        println!("The origin is at ({}, {})", origin.x, origin.y);

        let mut origin = Point{y: 20, x: 10,}; // origin: Point
        origin.x = 30;
        println!("The origin is at ({}, {})", origin.x, origin.y);
        // Mutable struct
        {
            let r = PointRef{
                x: &mut origin.x,
                y: &mut origin.y,
            };
            *r.x = 5;
            *r.y = 6;
        }
        println!("The origin is at ({}, {})", origin.x, origin.y);

        let point3d_1 = Point3D {
            x:  10,
            y:  20,
            z:  30,
        };
        let point3d_2 = Point3D {
            z:  300,
            ..point3d_1
        };
        println!("The origin is at ({}, {}, {})",
                 point3d_2.x,
                 point3d_2.y,
                 point3d_2.z);
        tuple_stuct();
    }

    fn tuple_stuct() {
        struct Color(i32, i32, i32);
        let black = Color(0, 0, 0);
        let white = Color(0xFF, 0b010, 0xFF);
        println!("", black);
    }
}