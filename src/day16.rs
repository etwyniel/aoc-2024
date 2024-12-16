use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use aoc_framework::*;
use direction::Direction;
use grid::Grid;

pub struct Day16;

impl_day!(Day16::{part1, part2}: 2024[16], r"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
");

#[aoc(part = 1, example = 7036)]
fn part1(input: Vec<u8>) -> u64 {
    let grid = Grid::from_bytes(input);
    let start = grid.points_iter().find(|&p| grid[p] == b'S').unwrap();
    let end = grid.points_iter().find(|&p| grid[p] == b'E').unwrap();
    let start_dir = Direction::EAST;
    let mut dist = HashMap::new();
    let mut visited = HashSet::new();
    let mut open = HashSet::new();
    open.insert((start, start_dir));
    dist.insert((start, start_dir), 0);

    while let Some((u @ (pos, dir), distance)) = open
        .iter()
        .copied()
        .min_by_key(|p| dist[p])
        .map(|p| (p, dist[&p]))
    {
        visited.insert(u);
        open.remove(&u);
        if let Some(b'E') = grid.get(pos) {
            continue;
        }
        for i in -1..=1 {
            let neighbor = pos + (dir + i);
            match grid.get(neighbor) {
                None | Some(b'#') => continue,
                _ => {}
            }
            let v = (neighbor, dir + i);
            if visited.contains(&v) {
                continue;
            }
            let alt = distance + 1 + if i == 0 { 0 } else { 1000 };
            let current_dist = dist.entry(v).or_insert(u64::MAX);
            if alt < *current_dist {
                *current_dist = alt;
                open.insert(v);
            }
        }
    }
    (0..4)
        .map(Direction::new)
        .flat_map(|dir| dist.get(&(end, dir)).copied())
        .min()
        .unwrap_or(u64::MAX)
}

#[aoc(part = 2, example = 45)]
fn part2(input: Vec<u8>) -> u64 {
    let grid = Grid::from_bytes(input);
    let start = grid.points_iter().find(|&p| grid[p] == b'S').unwrap();
    let end = grid.points_iter().find(|&p| grid[p] == b'E').unwrap();
    let start_dir = Direction::EAST;
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();
    let mut visited = HashSet::new();
    let mut open = HashSet::new();
    open.insert((start, start_dir));
    dist.insert((start, start_dir), 0);

    while let Some((u @ (pos, dir), distance)) = open
        .iter()
        .copied()
        .min_by_key(|p| dist[p])
        .map(|p| (p, dist[&p]))
    {
        visited.insert(u);
        open.remove(&u);
        if let Some(b'E') = grid.get(pos) {
            continue;
        }
        for i in -1..=1 {
            let neighbor = pos + (dir + i);
            match grid.get(neighbor) {
                None | Some(b'#') => continue,
                _ => {}
            }
            let v = (neighbor, dir + i);
            if visited.contains(&v) {
                continue;
            }
            let alt = distance + 1 + if i == 0 { 0 } else { 1000 };
            let current_dist = dist.entry(v).or_insert(u64::MAX);
            match alt.cmp(current_dist) {
                Ordering::Less => {
                    *current_dist = alt;
                    prev.insert(v, vec![u]);
                    open.insert(v);
                }
                Ordering::Equal => {
                    prev.get_mut(&v).unwrap().push(u);
                    open.insert(v);
                }
                _ => {}
            }
        }
    }
    let min_dist = (0..4)
        .map(Direction::new)
        .flat_map(|dir| dist.get(&(end, dir)).copied())
        .min()
        .unwrap_or(u64::MAX);
    let mut on_best_path = HashSet::new();
    let mut stack = (0..4)
        .map(Direction::new)
        .map(|dir| (end, dir))
        .filter(|p| dist.get(p) == Some(&min_dist))
        .collect_vec();
    while let Some(p) = stack.pop() {
        on_best_path.insert(p.0);
        stack.extend(prev.get(&p).iter().flat_map(|v| v.iter()));
    }
    on_best_path.len() as u64
}
