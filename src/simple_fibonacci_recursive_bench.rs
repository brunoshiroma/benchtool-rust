extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::thread;

pub fn bench(count: u32) -> BigUint{

    let num: u64 = count as u64;

    let mut result : BigUint = Zero::zero();


    let res = thread::Builder::new()
        .stack_size(num as usize * 1024)
        .spawn(move || {
        result = calculate_fibonacci(Zero::zero(), One::one(), count, &mut 1);
        result
    });

    return res.unwrap().join().unwrap_or_default();
}

fn calculate_fibonacci(previous: BigUint, current: BigUint, max_interations: u32, current_interation: &mut u32) -> BigUint{
    *current_interation += 1;

    let _previous = current.clone();

    let result = previous + current;

    if *current_interation == max_interations {
        return result;
    }
    else{
        return calculate_fibonacci(_previous, result, max_interations, current_interation);
    }

}