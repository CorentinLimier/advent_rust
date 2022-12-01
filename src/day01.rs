use std::fs;

/* https://adventofcode.com/2022/day/1 */

fn get_input(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
}

fn split_blocs(input_content: String) -> Vec<String> {
    let blocs = Vec::from_iter(input_content.split("\n\n").map(String::from));
    blocs
}

fn sum_bloc(bloc: String) -> i32 {
    let mut result = 0;
    for line in bloc.lines() {
        let number = line.parse::<i32>().unwrap();
        result += number;
    }
    result
}

pub fn main() {
    let input_content = get_input("input/01.txt");
    let blocs = split_blocs(input_content);

    let mut calories_vector = Vec::new();

    for bloc in blocs {
        let calories = sum_bloc(bloc);
        calories_vector.push(calories)
    }

    calories_vector.sort_by(|a, b| b.cmp(a));

    println!("Highest calories : {}", calories_vector[0]);
    println!(
        "Sum 3 highest calories : {}",
        &calories_vector[0..3].iter().sum::<i32>()
    );
}
