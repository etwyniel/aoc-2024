use std::collections::{HashMap, HashSet};

use aoc_framework::*;
use direction::Direction;
use grid::Grid;
use point::Point;

pub struct Day18;

impl_day!(Day18::{part1, part2}: 2024[18], r"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
");

#[aoc(part = 1, example = 22)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    let bytes = input
        .flat_map(|ln| {
            ln.split(',')
                .flat_map(|s| s.parse::<isize>().ok())
                .tuples()
                .next()
        })
        .map(|(x, y)| Point([x, y]))
        .enumerate()
        .map(|(n, p)| (p, n))
        .collect::<HashMap<_, _>>();
    let start = Point([0, 0]);
    let (end, t) = if bytes.len() < 1000 {
        (Point([6, 6]), 12)
    } else {
        (Point([70, 70]), 1024)
    };
    let w = end.x() as usize + 1;
    let dist_data = vec![u64::MAX; w * (end.y() as usize + 1)];
    let mut dist = Grid::from_data(dist_data, w);
    dist.set(start, 0u64);
    let mut q = HashSet::new();
    q.insert(start);
    while let Some(u) = q
        .iter()
        .copied()
        .filter(|p| q.contains(p))
        .min_by_key(|&p| dist[p])
    {
        q.remove(&u);
        let d = dist[u];
        for dir in (0..4).map(Direction::new) {
            let v = u + dir;
            if !dist.in_bounds(v) {
                continue;
            }
            if bytes.get(&v).map(|&time| time < t).unwrap_or(false) {
                continue;
            }
            let alt = d + 1;
            if alt < dist[v] {
                dist.set(v, alt);
                q.insert(v);
            }
        }
    }
    dist[end]
}

#[aoc(part = 2, example = "6,1")]
fn part2(input: impl Iterator<Item = String>) -> String {
    let byte_list = input
        .flat_map(|ln| {
            ln.split(',')
                .flat_map(|s| s.parse::<isize>().ok())
                .tuples()
                .next()
        })
        .map(|(x, y)| Point([x, y]))
        .collect_vec();
    let bytes = byte_list
        .iter()
        .copied()
        .enumerate()
        .map(|(n, p)| (p, n))
        .collect::<HashMap<_, _>>();
    let start = Point([0, 0]);
    let (end, t) = if bytes.len() < 1000 {
        // working on example
        (Point([6, 6]), 12)
    } else {
        (Point([70, 70]), 1024)
    };
    let w = end.x() as usize + 1;
    let mut prev_path = HashSet::new();
    for (cur_time, last) in byte_list.iter().enumerate().skip(t) {
        if !prev_path.is_empty() && !prev_path.contains(last) {
            continue;
        }
        let dist_data = vec![u64::MAX; w * (end.y() as usize + 1)];
        let mut dist = Grid::from_data(dist_data, w);
        let mut prev = HashMap::new();
        dist.set(start, 0u64);
        let mut q = HashSet::new();
        q.insert(start);
        while let Some(u) = q.iter().copied().min_by_key(|&p| dist[p]) {
            if u == end {
                break;
            }
            q.remove(&u);
            let d = dist[u];
            for dir in (0..4).map(Direction::new) {
                let v = u + dir;
                if !dist.in_bounds(v)
                    || bytes.get(&v).map(|&time| time <= cur_time).unwrap_or(false)
                {
                    continue;
                }
                let alt = d + 1;
                if alt < dist[v] {
                    dist.set(v, alt);
                    q.insert(v);
                    prev.insert(v, u);
                }
            }
        }
        if dist[end] == u64::MAX {
            let Point([x, y]) = last;
            return format!("{x},{y}");
        }
        prev_path.clear();
        let mut cur = end;
        while let Some(&p) = prev.get(&cur) {
            prev_path.insert(p);
            cur = p;
        }
    }
    String::new()
}
