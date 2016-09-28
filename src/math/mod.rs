use self::factorial::*;
mod factorial;

const nfact: u64 = 10;

pub fn main() {
    factorial::factorial_display(nfact);
}