// Linked list structure
#[derive(Debug)]
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

impl<T> List<T> {
    fn new() -> Self {
        List::Nil
    }

    fn add(self, value: T) -> Self {
        List::Cons(value, Box::new(self))
    }

    fn length(&self) -> u32 {
        match *self {
            List::Cons(_, ref tail) => 1 + tail.length(),
            _ => 0,
        }
    }

    fn check(&self) -> bool {
        match *self {
            List::Cons(_, _) => true,
            List::Nil => false,
        }
    }

    fn check_loop(&self) -> bool {
        false
    }
}

pub fn main() {
    let mut list: List<i32> = List::new();
    list = list.add(2);
    list = list.add(5);
    list = list.add(7);

    println!("Linked list: {:?}", list);
    println!("Linked list length: {:?}", list.length());
}