use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn stream_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Eq, PartialEq)]
enum Weapon {
    Rock,
    Paper,
    Scissors,
}

impl Weapon {
    fn score(&self) -> i32 {
        match self {
            Weapon::Rock => return 1,
            Weapon::Paper => return 2,
            Weapon::Scissors => return 3,
        }
    }

    fn score_against(&self, opponent: Weapon) -> i32 {
        let equality = 3;
        let win = 6;
        let lose = 0;
        if *self == opponent {
            return equality;
        }
        match (self, opponent) {
            (Weapon::Rock, Weapon::Scissors) => return win,
            (Weapon::Paper, Weapon::Rock) => return win,
            (Weapon::Scissors, Weapon::Paper) => return win,
            _ => return lose,
        }
    }
}

fn to_weapon(weapon_str: &str) -> Weapon {
    match weapon_str {
        "A" | "X" => return Weapon::Rock,
        "B" | "Y" => return Weapon::Paper,
        "C" | "Z" => return Weapon::Scissors,
        _ => panic!("Weapon {weapon_str} not supported"),
    }
}

fn compute_round_score(opponent: &str, me: &str) -> i32 {
    let opponent_weapon = to_weapon(opponent);
    let me_weapon = to_weapon(me);
    return me_weapon.score() + me_weapon.score_against(opponent_weapon);
}

fn compute_tournament_score(strategy_guide: io::Lines<io::BufReader<File>>) -> i32 {
    let mut score = 0;
    for round in strategy_guide {
        if let Ok(r) = round {
            let (me, opponent) = r.split_once(" ").unwrap();
            score += compute_round_score(me, opponent);
        }
    }
    return score;
}

pub fn main() {
    if let Ok(lines) = stream_input("input/02.txt") {
        let score_tournament = compute_tournament_score(lines);
        println!("Score tournament {score_tournament}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_round_score_equality() {
        let me = "X";
        let opponent = "A";
        let score = compute_round_score(opponent, me);
        assert_eq!(score, 4)
    }

    #[test]
    fn test_compute_round_score_win() {
        let me = "Y";
        let opponent = "A";
        let score = compute_round_score(opponent, me);
        assert_eq!(score, 8)
    }

    #[test]
    fn test_compute_round_score_lost() {
        let me = "Y";
        let opponent = "C";
        let score = compute_round_score(opponent, me);
        assert_eq!(score, 2)
    }
}
