use std::env;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1{
        panic!("argument day is required");
    }

    let day = &args[1];

    if day == "01" {
        days::day01::main();
    } else {
        panic!("day not supported");
    }
}
