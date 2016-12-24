// Linked list structure
#[derive(Debug)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

impl<T> List<T> {
    pub fn new() -> Self {
        List::Nil
    }

    pub fn add(self, value: T) -> Self {
        List::Cons(value, Box::new(self))
    }

    pub fn length(&self) -> u32 {
        match *self {
            List::Cons(_, ref tail) => 1 + tail.length(),
            _ => 0,
        }
    }

    pub fn check(&self) -> bool {
        match *self {
            List::Cons(_, _) => true,
            List::Nil => false,
        }
    }

    pub fn check_loop(&self) -> bool {
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