use std::fmt::Debug;

#[allow(dead_code)]
struct YourType;
trait TraitB {}
trait TraitC {}
trait TraitE {}
trait TraitF {}
trait MyTrait<A, D>{}
// Long version:
//impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}
// Expressing bounds with a `where` clause
impl <A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}

trait PrintInOption {
    fn print_in_option(self);
}

// Because we would otherwise have to express this as `T: Debug` or
// use another method of indirect approach, this requires a `where` clause:
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // We want `Option<T>: Debug` as our bound because that is what's
    // being printed. Doing otherwise would be using the wrong bound.
    fn print_in_option(self) {
        println!("Result: {:?}", Some(self));
    }
}

pub fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
