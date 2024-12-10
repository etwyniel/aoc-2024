use std::collections::{HashMap, HashSet};

use aoc_framework::*;
use direction::Direction;
use grid::Grid;
use point::Point2;

pub struct Day10;

impl_day!(Day10::{part1, part2}: 2024[10], r"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
");

#[aoc(part = 1, example = 36)]
fn part1(input: Vec<u8>) -> u64 {
    let g = Grid::from_bytes(input);
    let mut trailheads: HashMap<point::Point<2>, HashSet<Point2>> = HashMap::new();
    for start in g.points_iter().filter(|&p| g[p] == b'0') {
        let mut stack = vec![start];
        while let Some(p) = stack.pop() {
            let n = g[p];
            for i in 0..4 {
                let neighbor = p + Direction::new(i);
                if g.get(neighbor).copied() == Some(n + 1) {
                    if n == b'8' {
                        trailheads.entry(neighbor).or_default().insert(start);
                    } else {
                        stack.push(neighbor);
                    }
                }
            }
        }
    }
    trailheads.values().map(|s| s.len() as u64).sum()
}

#[aoc(part = 2, example = 81)]
fn part2(input: Vec<u8>) -> u64 {
    let g = Grid::from_bytes(input);
    let mut trailheads: HashMap<point::Point<2>, u64> = HashMap::new();
    for start in g.points_iter().filter(|&p| g[p] == b'0') {
        let mut stack = vec![start];
        while let Some(p) = stack.pop() {
            let n = g[p];
            for i in 0..4 {
                let neighbor = p + Direction::new(i);
                if g.get(neighbor).copied() == Some(n + 1) {
                    if n == b'8' {
                        *trailheads.entry(neighbor).or_default() += 1;
                    } else {
                        stack.push(neighbor);
                    }
                }
            }
        }
    }
    trailheads.values().sum()
}
