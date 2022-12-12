use std::{collections::HashMap, error::Error, fs::read_to_string};

use pathfinding::directed::bfs::bfs;

fn find_path(
    map: &HashMap<(isize, isize), i32>,
    start: &(isize, isize),
    end: &(isize, isize),
) -> Option<usize> {
    bfs(
        start,
        |&(x, y)| {
            let valid_elevs = 1..=map[&(x, y)] + 1;
            [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .into_iter()
                .flat_map(move |coord| {
                    map.get(&coord)
                        .filter(|elev| valid_elevs.contains(elev))
                        .and(Some(coord))
                })
        },
        |coord| coord == end,
    )
    .map(|path| path.len() - 1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (map, start, end) = read_to_string("input.txt")?
        .split('\n')
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, char)| match char {
                'S' => ((x as isize, y as isize), 1, Some('S')),
                'E' => ((x as isize, y as isize), 26, Some('E')),
                _ => ((x as isize, y as isize), char as i32 - 'a' as i32 + 1, None),
            })
        })
        .fold(
            (HashMap::new(), None, None),
            |(mut map, loc_s, loc_e), (coord, elev, loc)| {
                map.insert(coord, elev);
                let check = |c| if loc == Some(c) { Some(coord) } else { None };
                (map, loc_s.or(check('S')), loc_e.or(check('E')))
            },
        );

    let (start, end) = (start.ok_or("no start")?, end.ok_or("no end")?);
    let len_part1 = find_path(&map, &start, &end).ok_or("no path part 1")?;
    println!("2022-12-12 Part 1: {len_part1}");

    let len_part2 = map
        .iter()
        .filter(|(_, elev)| **elev == 1)
        .filter_map(|(coord, _)| find_path(&map, coord, &end))
        .min()
        .ok_or("no path part 2")?;
    Ok(println!("2022-12-12 Part 2: {len_part2}"))
}
