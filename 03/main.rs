use std::{collections::HashSet, error::Error, fs::read_to_string};

struct Rucksack {
    comp1: HashSet<char>,
    comp2: HashSet<char>,
}

impl Rucksack {
    fn parse(input_line: &str) -> Self {
        let (comp1_chars, comp2_chars) = input_line.split_at(input_line.len() / 2);
        Self {
            comp1: comp1_chars.chars().collect(),
            comp2: comp2_chars.chars().collect(),
        }
    }

    fn union(&self) -> HashSet<char> {
        self.comp1.union(&self.comp2).copied().collect()
    }
}

fn priority(c: char) -> usize {
    if c.is_ascii_lowercase() {
        c as usize - 'a' as usize + 1
    } else {
        c as usize - 'A' as usize + 27
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let rucksacks = read_to_string("input.txt")?
        .split('\n')
        .map(Rucksack::parse)
        .collect::<Vec<_>>();

    let priority_sum = rucksacks
        .iter()
        .map(|rucksack| {
            rucksack
                .comp1
                .intersection(&rucksack.comp2)
                .copied()
                .map(priority)
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("2022-12-03 Part 1: {priority_sum}");

    let badge_priority_sums = rucksacks
        .chunks_exact(3)
        .map(|group| {
            group
                .into_iter()
                .fold(None, |total_option: Option<HashSet<_>>, rucksack| {
                    if let Some(total) = total_option {
                        Some(total.intersection(&rucksack.union()).copied().collect())
                    } else {
                        Some(rucksack.union())
                    }
                })
                .into_iter()
                .flatten()
                .map(priority)
                .sum::<usize>()
        })
        .sum::<usize>();
    Ok(println!("2022-12-03 Part 2: {badge_priority_sums}"))
}
