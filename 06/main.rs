use std::{collections::HashSet, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?.chars().collect::<Vec<_>>();

    Ok(for size in [4, 14] {
        let (index, _) = input
            .windows(size)
            .enumerate()
            .find(|(_, chars)| chars.iter().collect::<HashSet<_>>().len() == size)
            .ok_or("no match")?;
        println!("2022-12-06 Part {}: {}", size / 10 + 1, index + size);
    })
}
