// As output parameters
//
// Closures as input parameters are possible, so returning closures
// as output parameters should also be possible. However, returning
// closure types are problematic because Rust currently only supports
// returning concrete (non-generic) types. Anonymous closure types
// are, by definition, unknown and so returning a closure is only
// possible by making it concrete. This can be done via boxing.
//
//T he valid traits for returns are slightly different than before:
//
//    Fn: normal
//    FnMut: normal
//    FnOnce: There are some unusual things at play here, so the FnBox
// type is currently needed, and is unstable. This is expected to change
// in the future.
//
// Beyond this, the move keyword must be used, which signals that all
// captures occur by value. This is required because any captures by
// reference would be dropped as soon as the function exited, leaving
// invalid references in the closure.

fn create_fn() -> Box<Fn()> {
    let text = "Fn".to_owned();

    Box::new(move || println!("This is a: {}", text))
}

fn create_fnmut() -> Box<FnMut()> {
    let text = "FnMut".to_owned();

    Box::new(move || println!("This is a: {}", text))
}

pub fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();
}
