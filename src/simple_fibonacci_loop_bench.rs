extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;

pub fn bench(count: u32) -> BigUint{

    let  mut current : BigUint  = Zero::zero();
    let mut next: BigUint  = One::one();

    for _i in 1..count {

        let result = current + &next;
        // see https://docs.rs/num-bigint/0.2.6/num_bigint/index.html
        current = replace(&mut next, result);

    }

    return current;

}