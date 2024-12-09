use std::collections::HashMap;

use aoc_framework::*;
use point::{Point, Point2};

pub struct Day08;

impl_day!(Day08::{part1, part2}: 2024[8], r"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
");

#[aoc(part = 1, example = 14)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    let mut station_types: HashMap<u8, Vec<Point2>> = HashMap::new();
    let mut positions: HashMap<Point2, u8> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    input.enumerate().for_each(|(y, ln)| {
        max_y = y as isize;
        ln.bytes().enumerate().for_each(|(x, b)| {
            max_x = max_x.max(x as isize);
            if b == b'.' {
                return;
            }
            let pos = Point([x as isize, y as isize]);
            station_types.entry(b).or_default().push(pos);
            positions.insert(pos, b);
        })
    });
    let mut add_antinode = |p: Point2| {
        if p.x() >= 0 && p.y() >= 0 && p.x() <= max_x && p.y() <= max_y {
            positions.insert(p, b'#');
        }
    };
    for antennas in station_types.values() {
        for (i, &a1) in antennas.iter().enumerate() {
            for &a2 in &antennas[i + 1..] {
                let dist = a1 - a2;
                let an1 = a1 + dist;
                let an2 = a2 - dist;
                add_antinode(an1);
                add_antinode(an2);
            }
        }
    }
    positions.values().filter(|&&b| b == b'#').count() as u64
}

#[aoc(part = 2, example = 34)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    let mut station_types: HashMap<u8, Vec<Point2>> = HashMap::new();
    let mut positions: HashMap<Point2, u8> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    input.enumerate().for_each(|(y, ln)| {
        max_y = y as isize;
        ln.bytes().enumerate().for_each(|(x, b)| {
            max_x = max_x.max(x as isize);
            if b == b'.' {
                return;
            }
            let pos = Point([x as isize, y as isize]);
            station_types.entry(b).or_default().push(pos);
            positions.insert(pos, b);
        })
    });
    let in_bounds = |p: Point2| p.x() >= 0 && p.y() >= 0 && p.x() <= max_x && p.y() <= max_y;
    for antennas in station_types.values() {
        for (i, &a1) in antennas.iter().enumerate() {
            for &a2 in &antennas[i + 1..] {
                let dist = a1 - a2;
                for n in 0.. {
                    let an = a1 + dist * n;
                    if !in_bounds(an) {
                        break;
                    }
                    positions.insert(an, b'#');
                }
                for n in 0.. {
                    let an = a2 - dist * n;
                    if !in_bounds(an) {
                        break;
                    }
                    positions.insert(an, b'#');
                }
            }
        }
    }
    positions
        .values()
        .filter(|&&b| b != b'.' && station_types.get(&b).map(|v| v.len() > 1).unwrap_or(true))
        .count() as u64
}
