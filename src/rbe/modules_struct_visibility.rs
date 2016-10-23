// Struct visibility
// Structs have an extra level of visibility with their fields.
// The visibility defaults to private, and can be overridden with
// the pub modifier. This visibility only matters when a struct
// is accessed from outside the module where it is defined, and
// has the goal of hiding information (encapsulation).

mod my {
    // A public struct with a public field of generic type `T`
    pub struct WhiteBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    pub struct BlackBox<T> {
        contents: T,
    }

    impl<T> BlackBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox {
                contents: contents,
            }
        }

        // Get private value (with generic type)
        pub fn get_contents(&self) -> &T {
            &self.contents
        }
    }
}

pub fn main() {
    // Public structs with public fields can be constructed as usual
    let white_box = my::WhiteBox { contents: "public information" };

    // and their fields can be normally accessed.
    println!("The white box contains: {}", white_box.contents);

    // Public structs with private fields cannot be constructed using field names.
    // Error! `BlackBox` has private fields
    //let black_box = my::BlackBox { contents: "classified information" };
    // TODO ^ Try uncommenting this line

    // However, structs with private fields can be created using
    // public constructors
    let black_box = my::BlackBox::new("classified information");

    // and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
    //println!("The black box contains: {}", black_box.contents);
    // TODO ^ Try uncommenting this line

    //==> Activity:
    println!("The black box contains: {}", black_box.get_contents());
    // Explicit type
    let black_box_int: my::BlackBox<u32> = my::BlackBox::new(135);
    println!("The black box INT<i32> contains: {}", *black_box_int.get_contents() as i32 * -1);
}
