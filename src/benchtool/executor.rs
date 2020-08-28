use std::time::{Duration, Instant};
use std::vec::Vec;
use std::convert::TryInto;
use std::process;

use num_bigint::BigUint;
use num_traits::{Zero};

pub fn benchtool_rust_execute(iteration: i32, repeat: i32, bench_type: i32) -> String{

    let value = iteration;
    let loops = repeat;

    let mut exec_times: Vec<Duration> = Vec::with_capacity(loops.try_into().unwrap());

    let mut result: BigUint = Zero::zero();
    for _i in 0..loops {//FOR 'WARMUP' and/or system usage avg
        let now = Instant::now();
        if bench_type == 1{
            result = crate::benchtool::simple_fibonacci_loop_bench::bench(value);
        }
        else if bench_type == 2{
            result = crate::benchtool::simple_fibonacci_recursive_bench::bench(value);
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

    return format!("{0} {1}", elapsed_mean.div_f32(loops as f32).as_millis(), result);
}