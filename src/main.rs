use std::env;

mod day01;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];

    if day == "01" {
        day01::main();
    } else {
        println!("day not supported")
    }
}
