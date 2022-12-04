use std::{collections::HashSet, error::Error, fs::read_to_string};

use lazy_static::lazy_static;
use regex::Regex;

struct ElfRanges(HashSet<usize>, HashSet<usize>);

impl ElfRanges {
    fn parse(input: &str) -> Option<Self> {
        lazy_static! {
            static ref RANGE_REGEX: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
        }
        let input_match = RANGE_REGEX.captures(input)?;
        let number = |index| input_match.get(index)?.as_str().parse::<usize>().ok();
        Some(Self(
            (number(1)?..=number(2)?).collect(),
            (number(3)?..=number(4)?).collect(),
        ))
    }

    fn is_total_overlap(&self) -> bool {
        self.0.is_subset(&self.1) || self.1.is_subset(&self.0)
    }

    fn is_partial_overlap(&self) -> bool {
        self.0.intersection(&self.1).next().is_some()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let elf_ranges = read_to_string("input.txt")?
        .split('\n')
        .filter_map(ElfRanges::parse)
        .collect::<Vec<_>>();

    let total_overlaps = elf_ranges
        .iter()
        .filter(|range| range.is_total_overlap())
        .count();
    println!("2022-12-04 Part 1: {total_overlaps}");

    let partial_overlaps = elf_ranges
        .iter()
        .filter(|range| range.is_partial_overlap())
        .count();
    Ok(println!("2022-12-04 Part 2: {partial_overlaps}"))
}
