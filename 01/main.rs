use std::{collections::BTreeSet, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let elf_calories_string = read_to_string("input.txt")?;

    let total_elf_calories = elf_calories_string
        .split("\n\n")
        .map(|elf_calories| {
            elf_calories
                .split('\n')
                .filter_map(|n| n.parse::<u32>().ok())
                .sum::<u32>()
        })
        .collect::<BTreeSet<_>>();

    let elf_with_max_calories = total_elf_calories.iter().last().ok_or("no elves")?;
    println!("2022-12-01 Part 1: {elf_with_max_calories}");

    let top_three_elves_with_max_calories = total_elf_calories.iter().rev().take(3).sum::<u32>();
    Ok(println!(
        "2022-12-01 Part 2: {top_three_elves_with_max_calories}"
    ))
}
