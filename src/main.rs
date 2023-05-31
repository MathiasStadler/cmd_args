use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let n_args: usize = (&args.len()) -1  ;
    let prg_name: &String = &args[0];
    let query: &String = &args[1];
    let file_path: &String = &args[2];

    

    println!("n arguments : {}", n_args);
    println!("Program Name {}",prg_name);
    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
