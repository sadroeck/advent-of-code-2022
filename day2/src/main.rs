use std::fs::read_to_string;

const DAY_2_URI: &'static str = "./input.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(DAY_2_URI)?;
    let score = calculate_score_v1(&input);
    println!("Score V1 is {score}");

    let score = calculate_score_v2(&input);
    println!("Score V2 is {score}");
    Ok(())
}

fn calculate_score_v1(input: &str) -> usize {
    input.lines().map(|line| {
        let opponent = Move::from(line.get(0..1).unwrap());
        let mine = Move::from(line.get(2..3).unwrap());
        let score = Move::battle_score(mine, opponent) + mine.score();
        score
    }).sum()
}

fn calculate_score_v2(input: &str) -> usize {
    input.lines().map(|line| {
        let opponent = Move::from(line.get(0..1).unwrap());
        let mine = BattleResult::from(line.get(2..3).unwrap()).move_required(opponent);
        let score = Move::battle_score(mine, opponent) + mine.score();
        score
    }).sum()
}

#[derive(PartialEq, Clone, Copy, Eq, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}


impl Move {
    #[inline]
    fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    #[inline]
    fn battle_score(mine: Move, opponent: Move) -> usize {
        match (mine, opponent) {
            (Move::Rock, Move::Paper) | (Move::Paper, Move::Scissors) | (Move::Scissors, Move::Rock) => 0, // Loss
            (Move::Paper, Move::Rock) | (Move::Scissors, Move::Paper) | (Move::Rock, Move::Scissors) => 6, // Win
            (x, y) if x == y => 3, // Tie
            _ => unreachable!(),
        }
    }
}

impl From<&str> for Move {
    #[inline]
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

enum BattleResult {
    Win,
    Lose,
    Tie,
}

impl From<&str> for BattleResult {
    #[inline]
    fn from(value: &str) -> Self {
        match value {
            "X" => Self::Lose,
            "Y" => Self::Tie,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }
}

impl BattleResult {
    fn move_required(&self, opponent: Move) -> Move {
        match (self, opponent) {
            (Self::Tie, _) => opponent,
            (Self::Win, Move::Paper) => Move::Scissors,
            (Self::Win, Move::Rock) => Move::Paper,
            (Self::Win, Move::Scissors) => Move::Rock,
            (Self::Lose, Move::Paper) => Move::Rock,
            (Self::Lose, Move::Rock) => Move::Scissors,
            (Self::Lose, Move::Scissors) => Move::Paper,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_v1() {
        let input = "A Y\nB X\nC Z";
        let score = calculate_score_v1(input);
        assert_eq!(score, 15);
    }

    #[test]
    fn example_v2() {
        let input = "A Y\nB X\nC Z";
        let score = calculate_score_v2(input);
        assert_eq!(score, 12);
    }
}