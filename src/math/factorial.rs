extern crate num;
extern crate num_bigint;
extern crate num_traits;

use self::num_bigint::{BigUint, ToBigUint};
use self::num_traits::One;

pub fn factorial(num: u64) -> BigUint {
	let current: BigUint = num.to_biguint().unwrap();
	if num <= 1 {
		return One::one();
	}
	return current * factorial(num - 1);
}

pub fn factorial_display(num: u64) {
	println!("Factorisl {}! = {}", num, factorial(num))
}