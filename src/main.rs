use std::{env, usize};

extern crate simple_stopwatch;
use simple_stopwatch::Stopwatch;
fn main() {
    let sw_first: Stopwatch = Stopwatch::start_new();

    let args: Vec<String> = env::args().collect();

    let n_args: usize = args.len() - 1;
    let prg_name: &String = &args[0];
    let query: &String = &args[1];
    let file_path: &String = &args[2];

    println!("n arguments : {}", n_args);
    println!("Program Name {}", prg_name);
    println!("Searching for {}", query);
    println!("In file {}", file_path);

    args.iter()
        .enumerate()
        .for_each(|(i, x): (usize, &String)| {
            println!("Item {} = {}", i, x);
        });

    let elapsed_ms: f32 = sw_first.ms();
    println!("Time taken: {}ms", elapsed_ms);

    my_function();
}

//from here
//https://crates.io/crates/simple-stopwatch
fn my_function() {
    println!("my_function");
}
