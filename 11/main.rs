use std::{cmp::Reverse, error::Error, fs::read_to_string, rc::Rc};

use lazy_static::lazy_static;
use num::integer::lcm;
use regex::Regex;

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Rc<dyn Fn(usize) -> usize>,
    test: (usize, usize, usize),
    count: usize,
}

impl Monkey {
    fn parse(text: &str) -> Option<Monkey> {
        lazy_static! {
            static ref ITEMS_REGEX: Regex =
                Regex::new(r"^\s+Starting items: (?P<items>.*)$").unwrap();
            static ref OPERATION_REGEX: Regex =
                Regex::new(r"^\s+Operation: new = (?P<operation>.+)$").unwrap();
            static ref INNER_OPERATION_REGEX: Regex =
                Regex::new(r"^old (?P<operator>\+|\*) (?P<parameter>old|\d+)$").unwrap();
            static ref TEST_REGEX: Regex =
                Regex::new(r"^\s+Test: divisible by (?P<test>\d+)$").unwrap();
            static ref TRUE_REGEX: Regex =
                Regex::new(r"^\s+If true: throw to monkey (?P<true>\d+)$").unwrap();
            static ref FALSE_REGEX: Regex =
                Regex::new(r"^\s+If false: throw to monkey (?P<false>\d+)$").unwrap();
        }
        let mut lines = text.split('\n');
        lines.next()?; // Skip the monkey index line.
        let items_match = ITEMS_REGEX.captures(lines.next()?)?;
        let operation_match = OPERATION_REGEX.captures(lines.next()?)?;
        let test_match = TEST_REGEX.captures(lines.next()?)?;
        let true_match = TRUE_REGEX.captures(lines.next()?)?;
        let false_match = FALSE_REGEX.captures(lines.next()?)?;
        let items = items_match
            .name("items")?
            .as_str()
            .split(", ")
            .filter_map(|n| n.parse().ok())
            .collect::<Vec<_>>();
        let operation = operation_match
            .name("operation")
            .map(|operation_match| operation_match.as_str())
            .and_then(|operation| {
                let operation_match = INNER_OPERATION_REGEX.captures(operation)?;
                let operator = operation_match.name("operator")?.as_str().to_owned();
                let parameter = match operation_match.name("parameter")?.as_str() {
                    "old" => None,
                    number => number.parse().ok(),
                };
                Some(Rc::new(move |worry| match operator.as_str() {
                    "+" => worry + parameter.unwrap_or(worry),
                    "*" => worry * parameter.unwrap_or(worry),
                    _ => unreachable!(),
                }))
            })?;
        let test = (
            test_match.name("test")?.as_str().parse().ok()?,
            true_match.name("true")?.as_str().parse().ok()?,
            false_match.name("false")?.as_str().parse().ok()?,
        );

        Some(Self {
            items,
            operation,
            test,
            count: 0,
        })
    }
}

fn play_round<F>(monkeys: &mut Vec<Monkey>, cap_worry_level: F)
where
    F: Fn(usize) -> usize,
{
    for current_monkey in 0..monkeys.len() {
        for worry_level in monkeys[current_monkey].items.drain(..).collect::<Vec<_>>() {
            let new_worry_level = cap_worry_level((monkeys[current_monkey].operation)(worry_level));
            let target_monkey = if new_worry_level % monkeys[current_monkey].test.0 == 0 {
                monkeys[current_monkey].test.1
            } else {
                monkeys[current_monkey].test.2
            };
            monkeys[target_monkey].items.push(new_worry_level);
            monkeys[current_monkey].count += 1;
        }
    }
}

fn play_game<F>(monkeys: &mut Vec<Monkey>, rounds: u16, cap_worry_level: F)
where
    F: Fn(usize) -> usize,
{
    (0..rounds).for_each(|_| play_round(monkeys, &cap_worry_level));

    monkeys.sort_by_key(|Monkey { count, .. }| Reverse(*count));

    let monkey_business = monkeys[0..=1]
        .into_iter()
        .fold(1, |product, Monkey { count, .. }| product * *count);
    println!("2022-12-11 Part {}: {monkey_business}", rounds / 10000 + 1);
}

fn main() -> Result<(), Box<dyn Error>> {
    let monkeys = read_to_string("input.txt")?
        .split("\n\n")
        .filter_map(Monkey::parse)
        .collect::<Vec<_>>();

    let modulus = monkeys
        .iter()
        .fold(1, |modulus, monkey| lcm(modulus, monkey.test.0));

    play_game(&mut monkeys.clone(), 20, |worry_level| {
        (worry_level as f64 / 3.0).floor() as usize
    });

    Ok(play_game(&mut monkeys.clone(), 10_000, |worry_level| {
        worry_level % modulus
    }))
}
