extern crate num_bigint;
// extern crate num_traits;

use self::num_bigint::{BigUint, ToBigUint};
//use self::num_traits::{Zero, One};

const one: BigUint = 1.to_biguint().unwrap();

pub fn factorial(num: BigUint) -> BigUint {
	if num == one {
		return num;
	}
	return one;
//	let res = num.sub(one);
//	return res;
	//let res: Ratio<BigInt> = Ratio::from_integer(FromPrimitive::from_u64(num).unwrap());
	//return num * factorial(num+1);
}

pub fn factorial_display(num: u64) {
	let current: BigUint = num.to_biguint().unwrap();
	//factorial(current)
	println!("Factorisl")
}