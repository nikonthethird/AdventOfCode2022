use std::{error::Error, fs::read_to_string};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Sensor {
    sensor_x: isize,
    sensor_y: isize,
    beacon_x: isize,
    beacon_y: isize,
}

impl Sensor {
    fn parse(text: &str) -> Option<Self> {
        lazy_static! {
            static ref SENSOR_REGEX: Regex = Regex::new(
                r"^Sensor at x=(?P<s_x>-?\d+), y=(?P<s_y>-?\d+): closest beacon is at x=(?P<b_x>-?\d+), y=(?P<b_y>-?\d+)$"
            ).unwrap();
        }
        let sensor_match = SENSOR_REGEX.captures(text)?;
        Some(Self {
            sensor_x: sensor_match.name("s_x")?.as_str().parse().ok()?,
            sensor_y: sensor_match.name("s_y")?.as_str().parse().ok()?,
            beacon_x: sensor_match.name("b_x")?.as_str().parse().ok()?,
            beacon_y: sensor_match.name("b_y")?.as_str().parse().ok()?,
        })
    }

    fn distance(&self) -> isize {
        (self.sensor_x - self.beacon_x).abs() + (self.sensor_y - self.beacon_y).abs()
    }

    fn blocked_range(&self, row: isize) -> Option<(isize, isize)> {
        let remaining_distance = self.distance() - (self.sensor_y - row).abs();
        if remaining_distance < 0 {
            None
        } else {
            Some((
                self.sensor_x - remaining_distance,
                self.sensor_x + remaining_distance,
            ))
        }
    }
}

fn compute_blocked_ranges(sensors: &Vec<Sensor>, row: isize) -> Vec<(isize, isize)> {
    let mut raw_blocked_ranges = sensors
        .iter()
        .filter_map(|sensor| sensor.blocked_range(row))
        .collect::<Vec<_>>();
    raw_blocked_ranges.sort_by_key(|(start, _)| *start);

    let mut blocked_ranges = Vec::new();
    while !raw_blocked_ranges.is_empty() {
        let mut r1 = raw_blocked_ranges.remove(0);
        let mut index = 0;
        while index < raw_blocked_ranges.len() {
            let r2 = raw_blocked_ranges[index];
            if r1.0 <= r2.0 && r1.1 >= r2.1 {
                raw_blocked_ranges.remove(index);
                continue;
            } else if r1.0 >= r2.0 && r1.1 <= r2.1 {
                r1 = r2;
                raw_blocked_ranges.remove(index);
                continue;
            } else if r1.0 <= r2.0 && r1.1 <= r2.1 && r1.1 >= r2.0 {
                r1.1 = r2.1;
                raw_blocked_ranges.remove(index);
                continue;
            } else if r1.0 >= r2.0 && r1.1 >= r2.1 && r2.1 >= r1.0 {
                r1.0 = r2.0;
                raw_blocked_ranges.remove(index);
                continue;
            }
            index += 1;
        }
        blocked_ranges.push(r1);
    }
    blocked_ranges
}

fn main() -> Result<(), Box<dyn Error>> {
    let sensors = read_to_string("input.txt")?
        .split('\n')
        .filter_map(Sensor::parse)
        .collect::<Vec<_>>();

    let blocked_ranges_row = compute_blocked_ranges(&sensors, 10);
    let blocked_range_len = blocked_ranges_row
        .into_iter()
        .fold(0, |total, (start, end)| total + end - start);
    println!("2022-12-15 Part 1: {blocked_range_len}");

    for row in 0..=4000000 {
        let blocked_ranges_row = compute_blocked_ranges(&sensors, row);
        if blocked_ranges_row.len() == 2 {
            let column = blocked_ranges_row[0].1 + 1;
            println!("2022-12-15 Part 2: {}", column * 4000000 + row);
            break;
        }
    }
    Ok(())
}
