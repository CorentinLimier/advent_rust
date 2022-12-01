// https://adventofcode.com/2022/day/1

use std::fs;

fn get_input(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
}

fn transform_input(input_content: String) -> Vec<Vec<i32>> {
    let mut elves = Vec::new();
    let blocs = input_content.split("\n\n");
    for bloc in blocs{
        let elf_calories = bloc.lines().map(|x| x.parse::<i32>().unwrap()).collect();
        elves.push(elf_calories)
    }

    elves
}

pub fn main() {
    let input_content = get_input("input/01.txt");
    let elves = transform_input(input_content);

    let mut calories_by_elf: Vec<i32> = elves.iter().map(|x| x.iter().sum()).collect();

    calories_by_elf.sort_by(|a, b| b.cmp(a));

    println!("Highest calories : {}", calories_by_elf[0]);
    println!(
        "Sum 3 highest calories : {}",
        &calories_by_elf[0..3].iter().sum::<i32>()
    );
}
