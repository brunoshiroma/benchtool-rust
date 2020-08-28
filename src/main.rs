mod benchtool;

extern crate num_bigint;
extern crate num_traits;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage : benchtool-rust 1 10 [5]");
        std::process::exit(1);
    }

    let bench_type: i32 = args[1].parse::<i32>().unwrap();
    let value: i32 = args[2].parse::<i32>().unwrap();
    let mut loops: i32 = 5;

    if args.len() >= 4 {
        loops = args[3].parse::<i32>().unwrap();
    }

    print!("{}", benchtool::executor::benchtool_rust_execute(value, loops, bench_type));
    
}

