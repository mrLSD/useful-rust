/// Chapter 4.23 - Closures
/// [Chapter 4.23] https://doc.rust-lang.org/book/closures.html

pub use self::closures::*;
pub mod closures {
    pub fn main() {
        println!("====\nClosure:\n");
        // We did not need to annotate the
        // types of arguments the closure
        // takes or the values it returns
        let plus_one = |x| x + 1;
        println!("Assert check: {}", 3 == plus_one(2));

        // Expression for multi-line closure
        let plus_two = |x| {
            let mut res: i32 = x;
            res += 1;
            res += 1;
            res
        };
        println!("Assert check: {}", 4 == plus_two(2));

        println!("Assert check: {}", simelar_closures());

        // The environment for a closure can include bindings
        // from its enclosing
        // It borrows the binding!
        // we took ownership!
        let num = 5;
        let plus_num = |x: i32| x + num;
        println!("Assert sum: {} {}", plus_num(10), num);

        move_closures();
    }

    // Similar symantics for slosures and Function
    fn simelar_closures() -> bool {
        fn  plus_one_v1   (x: i32) -> i32 { x + 1 }
        let plus_one_v2 = |x: i32| -> i32 { x + 1 };
        let plus_one_v3 = |x: i32|          x + 1  ;

        plus_one_v1(10) == plus_one_v2(10) &&
        plus_one_v2(10) == plus_one_v3(10)
    }

    fn move_closures() {
        let mut num = 5;
        // `num` will be changed
        {
            let mut add_num = |x: i32| num += x;
            add_num(5);
        }
        assert_eq!(10, num);

        // For move closure
        let mut num = 5;
        {
            // Move -> copied `num` value
            // We took ownership of a copy
            let mut add_num = move |x: i32| num += x;
            add_num(5);
        }
        assert_eq!(5, num);
    }

}