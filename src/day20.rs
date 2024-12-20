use std::collections::HashSet;

use aoc_framework::*;
use direction::Direction;
use grid::Grid;
use point::Point;

pub struct Day20;

impl_day!(Day20::{part1, part2}: 2024[20], r"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
");

fn times_grid(grid: &Grid<u8, 2>) -> Grid<Option<u64>, 2> {
    let start = grid.points_iter().find(|p| grid[*p] == b'S').unwrap();
    let Point([w, h]) = grid.size();
    let mut times = Grid::from_data(vec![None; (w * h) as usize], w as usize);
    let mut pos = start;
    let mut last_dir = None;
    'outer: for i in 0u64.. {
        times.set(pos, Some(i));
        if grid[pos] == b'E' {
            break;
        }
        for dir in (0..4).map(Direction::new) {
            if Some(-dir) == last_dir {
                continue;
            }
            let neighbor = pos + dir;
            if let Some(b'.' | b'E') = grid.get(neighbor) {
                pos = neighbor;
                last_dir = Some(dir);
                continue 'outer;
            }
        }
        break;
    }
    times
}

#[aoc(part = 1, example = 5)]
fn part1(input: Vec<u8>) -> u64 {
    let grid = Grid::from_bytes(input);
    let times = times_grid(&grid);
    let threshold = if grid.size().x() > 20 { 100 } else { 20 };
    grid.points_iter()
        .filter(|p| grid[*p] == b'#')
        .map(|p| {
            let mut around = [None; 4];
            for (dir, time) in around.iter_mut().enumerate() {
                *time = times.get(p + Direction::new(dir as u8)).copied();
            }
            (0..2)
                .filter(|&i| match (around[i], around[i + 2]) {
                    (Some(Some(t1)), Some(Some(t2))) => {
                        let saved = t1.abs_diff(t2).saturating_sub(2);
                        saved >= threshold
                    }
                    _ => false,
                })
                .count()
        })
        .sum::<usize>() as u64
}

#[aoc(part = 2, example = 285)]
fn part2(input: Vec<u8>) -> u64 {
    let grid = Grid::from_bytes(input);
    let times = times_grid(&grid);
    let threshold = if grid.size().x() > 20 { 100 } else { 50 };
    let mut cheats = HashSet::new();
    //let mut histogram = [0; 100];
    for (p, t1) in grid.points_iter().flat_map(|p| times[p].map(|t| (p, t))) {
        for dy in 0..=20 {
            for dx in (-20 + dy)..=(20 - dy) {
                if dy == 0 && dx <= 0 {
                    continue;
                }
                let p2 = p + Point([dx, dy]);
                let Some(&Some(t2)) = times.get(p2) else {
                    continue;
                };
                let saved = t1.abs_diff(t2).saturating_sub((dy + dx.abs()) as u64);
                if saved >= threshold {
                    //eprintln!("{p} -{saved:->3}-> {p2}");
                    let inserted = if t1 < t2 {
                        cheats.insert((p, p2))
                    } else {
                        cheats.insert((p2, p))
                    };
                    //if inserted {
                    //    histogram[saved as usize] += 1;
                    //}
                }
            }
        }
    }
    //for (n, count) in histogram.into_iter().enumerate() {
    //    if count > 0 {
    //        eprintln!("{n}: {count}");
    //    }
    //}
    cheats.len() as u64
}
