#![feature(let_chains)]
use std::{collections::HashSet, error::Error, fs::read_to_string};

fn drop_sand(cave: &HashSet<(i16, i16)>, bottom: i16, floor: bool) -> Option<(i16, i16)> {
    let mut sand = (500, 0);
    loop {
        if sand.1 + 1 > bottom && !floor {
            break None;
        } else if cave.get(&(sand.0, sand.1 + 1)).is_none() && sand.1 < bottom + 1 {
            sand.1 += 1;
        } else if cave.get(&(sand.0 - 1, sand.1 + 1)).is_none() && sand.1 < bottom + 1 {
            sand = (sand.0 - 1, sand.1 + 1);
        } else if cave.get(&(sand.0 + 1, sand.1 + 1)).is_none() && sand.1 < bottom + 1 {
            sand = (sand.0 + 1, sand.1 + 1);
        } else {
            break Some(sand);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (mut cave, bottom) = read_to_string("input.txt")?.split('\n').try_fold(
        (HashSet::new(), 0),
        |(mut cave, mut bottom), line| {
            let mut line_coords = line.split(" -> ");
            let start_coord = line_coords.next().ok_or("start_coord")?;
            line_coords.try_fold(start_coord, |coord_1, coord_2| {
                let (x1, y1) = coord_1.split_once(',').ok_or("coord_1")?;
                let (x2, y2) = coord_2.split_once(',').ok_or("coord_2")?;
                let (mut x1, mut y1) = (x1.parse::<i16>()?, y1.parse::<i16>()?);
                let (x2, y2) = (x2.parse::<i16>()?, y2.parse::<i16>()?);
                bottom = bottom.max(y1.max(y2));
                cave.insert((x1, y1));
                while x1 != x2 || y1 != y2 {
                    x1 += (x2 - x1).signum();
                    y1 += (y2 - y1).signum();
                    cave.insert((x1, y1));
                }
                Ok::<_, Box<dyn Error>>(coord_2)
            })?;
            Ok::<_, Box<dyn Error>>((cave, bottom))
        },
    )?;

    let mut drop_counter = 0;
    while let Some(sand) = drop_sand(&cave, bottom, false) {
        drop_counter += 1;
        cave.insert(sand);
    }
    println!("2022-12-14 Part 1: {drop_counter}");

    while let Some(sand) = drop_sand(&cave, bottom, true) && sand != (500, 0) {
        drop_counter += 1;
        cave.insert(sand);
    }
    Ok(println!("2022-12-14 Part 2: {}", drop_counter + 1))
}
