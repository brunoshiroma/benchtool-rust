extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::{Zero, One};

pub fn bench(count: i32) -> BigUint{

    let mut current : BigUint  = Zero::zero();
    let mut next: BigUint  = One::one();

    for _i in 0..count {
        let result = current + &next;
        current = next;
        next = result
    }

    return current;

}