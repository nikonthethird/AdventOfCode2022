use std::{collections::BTreeMap, error::Error, fs::read_to_string};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let instr_regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$")?;
    let (stacks, instrs) = read_to_string("input.txt")?.split_once("\n\n").map(|(stacks, instrs)| (
        stacks.split('\n').rev().skip(1).fold(BTreeMap::<_, Vec<_>>::new(), |stacks, line| {
            line.chars().enumerate().fold(stacks, |mut stacks, (index, char)| {
                if index % 4 == 1 && char != ' ' {
                    stacks.entry(index / 4 + 1).or_default().push(char)
                }
                stacks
            })
        }),
        instrs.split('\n').filter_map(|instr| {
            let instr_match = instr_regex.captures(instr)?;
            let get = |index| instr_match.get(index)?.as_str().parse().ok();
            Some((get(1)?, get(2)?, get(3)?))            
        }).collect::<Vec<_>>()
    )).ok_or("no separator")?;

    let modified_stacks = instrs.iter().fold(
        [ Some(stacks.clone()), Some(stacks) ],
        |[ stacks_single, stacks_all ], (count, source, target)| [
            (0..*count).fold(stacks_single, |mut stacks_single, _| {
                let item = stacks_single.as_mut()?.get_mut(source)?.pop()?;
                stacks_single.as_mut()?.get_mut(target)?.push(item);
                stacks_single
            }),
            stacks_all.and_then(|mut stacks_all| {
                let source_items = stacks_all.get_mut(source)?;
                let items = source_items.drain(source_items.len() - count..).collect::<Vec<_>>();
                stacks_all.get_mut(target)?.extend(items.into_iter());
                Some(stacks_all)
            })
        ]
    );
    
    Ok(modified_stacks.into_iter().flatten().enumerate().for_each(|(index, modified_stack)| {
        let top_crates = modified_stack.values().filter_map(|stack| stack.last()).collect::<String>();
        println!("2022-12-05 Part {}: {top_crates}", index + 1);
    }))
}
