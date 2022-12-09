use std::{collections::HashSet, error::Error, fs::read_to_string, iter::once};

fn perform_move<'r, const N: usize>(
    rope: &'r mut [(isize, isize); N],
    direction: &str,
) -> Option<&'r (isize, isize)> {
    match direction {
        "R" => rope[0].0 += 1,
        "L" => rope[0].0 -= 1,
        "U" => rope[0].1 += 1,
        "D" => rope[0].1 -= 1,
        _ => unreachable!(),
    }

    for i in 0..N - 1 {
        let diff = (rope[i].0 - rope[i + 1].0, rope[i].1 - rope[i + 1].1);
        if diff.0.abs() == 2 || diff.1.abs() == 2 {
            rope[i + 1].0 += diff.0.signum();
            rope[i + 1].1 += diff.1.signum();
        } else {
            return None;
        }
    }

    Some(&rope[N - 1])
}

fn main() -> Result<(), Box<dyn Error>> {
    let rope_moves = read_to_string("input.txt")?
        .split('\n')
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    let (mut short_rope, mut long_rope) = ([Default::default(); 2], [Default::default(); 10]);
    let mut short_tail = once(Default::default()).collect::<HashSet<_>>();
    let mut long_tail = short_tail.clone();

    for rope_move in rope_moves.into_iter() {
        let (direction, amount) = rope_move.split_once(' ').ok_or("invalid move")?;
        for _ in 0..amount.parse()? {
            if let Some(tail) = perform_move(&mut short_rope, direction) {
                short_tail.insert(*tail);
            }
            if let Some(tail) = perform_move(&mut long_rope, direction) {
                long_tail.insert(*tail);
            }
        }
    }

    println!("2022-12-09 Part 1: {}", short_tail.len());
    Ok(println!("2022-12-09 Part 2: {}", long_tail.len()))
}
