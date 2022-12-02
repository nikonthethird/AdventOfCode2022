use std::{error::Error, fs::read_to_string};

#[derive(Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn parse(text: &str) -> Option<Self> {
        match text {
            "A" | "X" => Some(Self::Rock),
            "B" | "Y" => Some(Self::Paper),
            "C" | "Z" => Some(Self::Scissors),
            _ => None,
        }
    }

    fn value(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn lose(&self) -> Self {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }

    fn win(&self) -> Self {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }
}

struct ChoicePair(Choice, Choice);

impl ChoicePair {
    fn parse(text: &str) -> Option<ChoicePair> {
        let (fst, snd) = text.split_once(' ')?;
        Some(ChoicePair(Choice::parse(fst)?, Choice::parse(snd)?))
    }

    fn outcome(&self) -> usize {
        match self {
            ChoicePair(Choice::Rock, Choice::Paper)
            | ChoicePair(Choice::Paper, Choice::Scissors)
            | ChoicePair(Choice::Scissors, Choice::Rock) => 6,
            ChoicePair(Choice::Rock, Choice::Rock)
            | ChoicePair(Choice::Paper, Choice::Paper)
            | ChoicePair(Choice::Scissors, Choice::Scissors) => 3,
            ChoicePair(Choice::Rock, Choice::Scissors)
            | ChoicePair(Choice::Paper, Choice::Rock)
            | ChoicePair(Choice::Scissors, Choice::Paper) => 0,
        }
    }

    fn adjust(&self) -> Self {
        ChoicePair(
            self.0,
            match self.1 {
                Choice::Rock => self.0.lose(),
                Choice::Paper => self.0,
                Choice::Scissors => self.0.win(),
            },
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let choices = read_to_string("input.txt")?
        .split("\n")
        .filter_map(ChoicePair::parse)
        .collect::<Vec<_>>();

    let part1 = choices
        .iter()
        .map(|choices| choices.1.value() + choices.outcome())
        .sum::<usize>();
    println!("2022-12-02 Part 1: {part1}");

    let part2 = choices
        .iter()
        .map(|choices| {
            let adjusted_choice = choices.adjust();
            adjusted_choice.1.value() + adjusted_choice.outcome()
        })
        .sum::<usize>();
    Ok(println!("2022-12-02 Part 2: {part2}"))
}
