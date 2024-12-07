use rayon::prelude::*;
use std::collections::HashSet;

use aoc_framework::*;
use direction::Direction;
use grid::Grid;
use point::Point2;

pub struct Day06;

impl_day!(Day06::{part1, part2}: 2024[6], r"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
");

fn trace_path(g: &Grid<u8, 2>) -> HashSet<Point2> {
    let mut pos = g.points_iter().find(|&p| g[p] == b'^').unwrap();
    let mut dir = Direction::NORTH;
    let mut visited = HashSet::new();
    while g.in_bounds(pos) {
        visited.insert(pos);
        let mut next = pos + dir;
        if g.get(next) == Some(&b'#') {
            dir += 1;
            next = pos + dir;
        }
        pos = next;
    }
    visited
}

#[aoc(part = 1, example = 41)]
fn part1(input: Vec<u8>) -> u64 {
    let g = Grid::from_bytes(input);
    trace_path(&g).len() as u64
}

#[aoc(part = 2, example = 6)]
fn part2(input: Vec<u8>) -> u64 {
    let g = Grid::from_bytes(input);
    let start_pos = g.points_iter().find(|&p| g[p] == b'^').unwrap();
    trace_path(&g)
        .into_par_iter()
        .filter(|&p| {
            if p == start_pos {
                return false;
            }
            let mut pos = start_pos;
            let mut dir = Direction::NORTH;
            let mut visited = HashSet::with_capacity(1000);
            while g.in_bounds(pos) {
                if visited.contains(&(pos, dir)) {
                    return true;
                }
                visited.insert((pos, dir));
                let mut next = pos + dir;
                while p == next || Some(&b'#') == g.get(next) {
                    dir += 1;
                    next = pos + dir;
                }
                pos = next;
            }
            false
        })
        .count() as u64
}
