#![feature(is_some_and)]
use std::{cmp::Ordering, error::Error, fs::read_to_string, iter::Peekable, str::Chars};

#[derive(Clone, Debug, Eq, PartialEq, Ord)]
enum Packet {
    List(Vec<Packet>),
    Number(u8),
}

impl Packet {
    fn parse<'t>(chars: &mut Peekable<Chars<'t>>) -> Option<Packet> {
        let first_char = chars.next()?;
        if first_char.is_digit(10) {
            let mut number_string = String::from(first_char);
            while chars.peek().is_some_and(|char| char.is_digit(10)) {
                number_string.push(chars.next()?);
            }
            Some(Self::Number(number_string.parse().ok()?))
        } else if first_char == '[' {
            let mut packets = Vec::new();
            loop {
                if chars.peek() == Some(&']') {
                    chars.next()?;
                    break;
                }
                packets.push(Self::parse(chars)?);
                if chars.peek() == Some(&',') {
                    chars.next()?;
                }
            }
            Some(Self::List(packets))
        } else {
            None
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match compare((self, other)) {
            None => Some(Ordering::Equal),
            Some(true) => Some(Ordering::Less),
            Some(false) => Some(Ordering::Greater),
        }
    }
}

fn compare(packet_pair: (&Packet, &Packet)) -> Option<bool> {
    match packet_pair {
        (Packet::Number(left), Packet::Number(right)) if left == right => None,
        (Packet::Number(left), Packet::Number(right)) => Some(left < right),
        (Packet::List(left_list), Packet::List(right_list)) => left_list
            .into_iter()
            .zip(right_list.into_iter())
            .fold(None, |result, packet_pair| {
                result.or_else(|| compare(packet_pair))
            })
            .or(if left_list.len() == right_list.len() {
                None
            } else {
                Some(left_list.len() < right_list.len())
            }),
        (left @ Packet::Number(_), right_list @ Packet::List(_)) => {
            compare((&Packet::List(vec![left.clone()]), right_list))
        }
        (left_list @ Packet::List(_), right @ Packet::Number(_)) => {
            compare((left_list, &Packet::List(vec![right.clone()])))
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let packet_pairs = read_to_string("input.txt")?
        .split("\n\n")
        .filter_map(|packet_pair_text| {
            let (packet_text_1, packet_text_2) = packet_pair_text.split_once('\n')?;
            Some((
                Packet::parse(&mut packet_text_1.chars().peekable())?,
                Packet::parse(&mut packet_text_2.chars().peekable())?,
            ))
        })
        .collect::<Vec<_>>();

    let correct_index_sum = packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(index, (left, right))| {
            if compare((left, right))? {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("2022-12-13 Part 1: {correct_index_sum}");

    let divider_two = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let divider_six = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    let mut all_packets = packet_pairs
        .into_iter()
        .flat_map(|(left, right)| [left, right])
        .chain([divider_two.clone(), divider_six.clone()])
        .collect::<Vec<_>>();

    all_packets.sort();
    let (divider_two_loc, divider_six_loc) = all_packets.into_iter().enumerate().fold(
        (None, None),
        |(divider_two_loc, divider_six_loc), (index, packet)| {
            (
                divider_two_loc.or(if packet == divider_two {
                    Some(index + 1)
                } else {
                    None
                }),
                divider_six_loc.or(if packet == divider_six {
                    Some(index + 1)
                } else {
                    None
                }),
            )
        },
    );

    let decoder_key =
        divider_two_loc.ok_or("no divider two")? * divider_six_loc.ok_or("no divider six")?;
    Ok(println!("2022-12-13 Part 2: {decoder_key}"))
}
