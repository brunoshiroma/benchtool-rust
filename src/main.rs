extern crate num_bigint;
extern crate num_traits;

mod simple_fibonacci_loop_bench;
mod simple_fibonacci_recursive_bench;

use num_bigint::BigUint;
use num_traits::{Zero};

use std::env;
use std::time::{Duration, Instant};
use std::vec::Vec;
use std::convert::TryInto;
use std::process;



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage : benchtool-rust 1 10 [5]");
        std::process::exit(1);
    }

    let bench_type: u32 = args[1].parse::<u32>().unwrap();
    let value: u32 = args[2].parse::<u32>().unwrap();
    let mut loops: u32 = 5;

    if args.len() >= 4 {
        loops = args[3].parse::<u32>().unwrap();
    }

    let mut exec_times: Vec<Duration> = Vec::with_capacity(loops.try_into().unwrap());

    let mut result: BigUint = Zero::zero();
    for _i in 0..loops {//FOR 'WARMUP' and/or system usage avg
        let now = Instant::now();
        if bench_type == 1{
            result = simple_fibonacci_loop_bench::bench(value);
        }
        else if bench_type == 2{
            result = simple_fibonacci_recursive_bench::bench(value);
        } else {
            print!("unknown bench type {}", bench_type);
            process::exit(-1);
        }

        exec_times.push(now.elapsed());
    }



    let mut elapsed_max: Duration = Duration::from_secs(0);
    let mut elapsed_min: Duration = Duration::from_secs(0);
    let mut elapsed_mean: Duration = Duration::from_secs(0);
    for elapsed in exec_times.iter() {
        if elapsed_max > *elapsed{
            elapsed_max = *elapsed;
        }

        if elapsed_min < *elapsed {
            elapsed_min = *elapsed;
        }
    }

    //calculate the mean...
    for elapsed in exec_times.iter() {
        if elapsed_max != *elapsed || elapsed_min != *elapsed {
            elapsed_mean += *elapsed;
        }
    }

    println!("{:?} {}", elapsed_mean.div_f32(loops as f32).as_millis(), result);
    
}
