mod simple_fibonacci_loop_bench;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    println!("{}",simple_fibonacci_loop_bench::bench(args[1].parse::<u32>().unwrap()));
}
