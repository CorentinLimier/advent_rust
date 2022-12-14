// https://adventofcode.com/2022/day/1

use std::fs;

struct Elf {
    calories: Vec<i32>,
}

impl Elf {
    fn new(calories: Vec<i32>) -> Self {
        Elf { calories: calories }
    }

    fn sum_calories(&self) -> i32 {
        self.calories.iter().sum()
    }
}

fn get_input(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
}

fn transform_input(input_content: String) -> Vec<Elf> {
    let mut elves = Vec::new();
    let blocs = input_content.split("\n\n");
    for bloc in blocs {
        let elf = Elf::new(bloc.lines().map(|x| x.parse::<i32>().unwrap()).collect());
        elves.push(elf)
    }

    elves
}

pub fn main() {
    let input_content = get_input("input/01.txt");
    let elves = transform_input(input_content);

    let mut calories_by_elf: Vec<i32> = elves.iter().map(|x| x.sum_calories()).collect();

    calories_by_elf.sort_by(|a, b| b.cmp(a));

    println!("Highest calories : {}", calories_by_elf[0]);
    println!(
        "Sum 3 highest calories : {}",
        &calories_by_elf[0..3].iter().sum::<i32>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_calories() {
        let calories = vec![1, 2, 3];
        let elf: Elf = Elf::new(calories);
        assert_eq!(elf.sum_calories(), 6)
    }
}
