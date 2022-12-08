use std::{collections::HashMap, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let tree_grid = read_to_string("input.txt")?
        .split('\n')
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, char)| {
                Some(((x as isize, y as isize), char.to_digit(10)? as i8))
            })
        })
        .collect::<HashMap<_, _>>();

    let (visible_from_outside, largest_scenic_score) =
        tree_grid.keys().fold((0, 0), |(count, score), coord| (
            count + is_visible_any_direction(&tree_grid, *coord) as i32,
            score.max(scenic_score(&tree_grid, *coord)),
        ));
    println!("2022-12-08 Part 1: {visible_from_outside}");
    Ok(println!("2022-12-08 Part 2: {largest_scenic_score}"))
}

fn get_neighbors<F>(
    tree_grid: &HashMap<(isize, isize), i8>,
    mut coord: (isize, isize),
    next_coord: F,
) -> Vec<i8>
where
    F: Fn((isize, isize)) -> (isize, isize),
{
    let mut neighbors = Vec::default();
    loop {
        coord = next_coord(coord);
        if let Some(neighbor) = tree_grid.get(&coord) {
            neighbors.push(*neighbor);
        } else {
            break;
        }
    }
    neighbors
}

fn is_visible<F>(
    tree_grid: &HashMap<(isize, isize), i8>,
    coord: (isize, isize),
    next_coord: F,
) -> bool
where
    F: Fn((isize, isize)) -> (isize, isize),
{
    let size = tree_grid.get(&coord).copied().unwrap();
    get_neighbors(tree_grid, coord, next_coord)
        .into_iter()
        .max()
        .unwrap_or(-1)
        < size
}

fn is_visible_any_direction(
    tree_grid: &HashMap<(isize, isize), i8>,
    coord: (isize, isize),
) -> bool {
    is_visible(&tree_grid, coord, |(x, y)| (x + 1, y))
        || is_visible(&tree_grid, coord, |(x, y)| (x - 1, y))
        || is_visible(&tree_grid, coord, |(x, y)| (x, y + 1))
        || is_visible(&tree_grid, coord, |(x, y)| (x, y - 1))
}

fn viewing_distance<F>(
    tree_grid: &HashMap<(isize, isize), i8>,
    coord: (isize, isize),
    next_coord: F,
) -> usize
where
    F: Fn((isize, isize)) -> (isize, isize),
{
    let size = tree_grid.get(&coord).copied().unwrap();
    get_neighbors(tree_grid, coord, next_coord)
        .into_iter()
        .fold(Default::default(), |(found_tree, distance), neighbor| {
            if found_tree {
                (true, distance)
            } else {
                (neighbor >= size, distance + 1)
            }
        })
        .1
}

fn scenic_score(tree_grid: &HashMap<(isize, isize), i8>, coord: (isize, isize)) -> usize {
    viewing_distance(tree_grid, coord, |(x, y)| (x + 1, y))
        * viewing_distance(tree_grid, coord, |(x, y)| (x - 1, y))
        * viewing_distance(tree_grid, coord, |(x, y)| (x, y + 1))
        * viewing_distance(tree_grid, coord, |(x, y)| (x, y - 1))
}
