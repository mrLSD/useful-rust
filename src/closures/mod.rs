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

        let num = 5;
        let plus_num = |x: i32| x + num;
        println!("Assert sum: {} {}", plus_num(10), num);
    }

    fn simelar_closures() -> bool {
        fn  plus_one_v1   (x: i32) -> i32 { x + 1 }
        let plus_one_v2 = |x: i32| -> i32 { x + 1 };
        let plus_one_v3 = |x: i32|          x + 1  ;

        plus_one_v1(10) == plus_one_v2(10) &&
        plus_one_v2(10) == plus_one_v3(10)
    }


}