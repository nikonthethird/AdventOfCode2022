use std::{error::Error, fs::read_to_string};

enum Command {
    Noop,
    AddX(isize),
}

impl Command {
    fn parse(input: &str) -> Option<Self> {
        match input.split_at(4) {
            ("noop", _) => Some(Self::Noop),
            ("addx", param) => Some(Self::AddX(param.trim().parse().ok()?)),
            _ => unreachable!(),
        }
    }

    fn cycle_count(&self) -> isize {
        match self {
            Self::Noop => 1,
            Self::AddX(_) => 2,
        }
    }

    fn apply(&self, x: &mut isize) {
        if let Self::AddX(param) = self {
            *x += param;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let commands = read_to_string("input.txt")?
        .split('\n')
        .filter_map(Command::parse)
        .collect::<Vec<_>>();

    let (mut cycle, mut x, mut signal_strength) = (1, 1, 0);
    let mut cycle_iter = (20..=220).step_by(40).peekable();
    let mut crt = [' '; 6 * 40];

    for command in commands {
        for cycle_offset in 1..=command.cycle_count() {
            if (x - 1..=x + 1).contains(&((cycle - 1) % 40)) {
                crt[(cycle - 1) as usize] = '█';
            }
            if cycle_offset == command.cycle_count() {
                command.apply(&mut x);
            }
            cycle += 1;
            if cycle_iter.peek() == Some(&cycle) {
                signal_strength += x * cycle_iter.next().unwrap_or_default();
            }
        }
    }

    println!("2022-12-10 Part 1: {signal_strength}\n2022-12-10 Part 2:");
    Ok(crt.chunks(40).for_each(|l| println!("{}", String::from_iter(l))))
}
