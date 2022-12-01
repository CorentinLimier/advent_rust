use std::env;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];

    if day == "01" {
        days::day01::main();
    } else {
        println!("day not supported")
    }
}
