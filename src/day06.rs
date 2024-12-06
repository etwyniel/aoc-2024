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
    let mut g = Grid::from_bytes(input);
    let mut valid = 0;
    let start_pos = g.points_iter().find(|&p| g[p] == b'^').unwrap();
    for p in trace_path(&g) {
        if p == start_pos {
            continue;
        }
        let prev = g[p];
        g.set(p, b'#');
        let mut pos = start_pos;
        let mut dir = Direction::NORTH;
        let mut visited = HashSet::new();
        while g.in_bounds(pos) {
            if visited.contains(&(pos, dir)) {
                valid += 1;
                break;
            }
            visited.insert((pos, dir));
            let mut next = pos + dir;
            while let Some(b'#') = g.get(next) {
                dir += 1;
                next = pos + dir;
            }
            pos = next;
        }
        g.set(p, prev);
    }
    valid
}
