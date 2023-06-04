use std::env; 

fn run_file(path: %str)

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("Usage: jlox [script]");
        exit(64)
    } else if args.len() == 1 {
        run_file(args[1]);
    } else {
        run_prompt();
    }
}