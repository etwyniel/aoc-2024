use std::collections::HashMap;

use aoc_framework::*;
use direction::Direction;
use grid::Grid;
use point::Point;

pub struct Day12;

impl_day!(Day12::{part1, part2}: 2024[12], r"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
");

#[aoc(part = 1, example = 1930)]
fn part1(input: Vec<u8>) -> u64 {
    let g = Grid::from_bytes(input);
    let Point([w, h]) = g.size();
    let mut visited = Grid::from_data(vec![false; (w * h) as usize], w as usize);
    let mut stack = Vec::new();
    let mut total = 0;
    for p in g.points_iter() {
        if visited[p] {
            continue;
        }
        visited.set(p, true);
        let typ = g[p];
        let mut area = 0;
        let mut perimeter = 0;
        stack.clear();
        stack.push(p);
        while let Some(p) = stack.pop() {
            area += 1;
            for dir in 0..4 {
                let neighbor = p + Direction::new(dir);
                match g.get(neighbor) {
                    Some(&val) if val == typ => {
                        if !visited[neighbor] {
                            stack.push(neighbor);
                            visited.set(neighbor, true);
                        }
                    }
                    _ => {
                        perimeter += 1;
                    }
                }
            }
        }
        total += area * perimeter;
    }
    total
}

#[aoc(part = 2, example = 1206)]
fn part2(input: Vec<u8>) -> u64 {
    let g = Grid::from_bytes(input);
    let Point([w, h]) = g.size();
    let mut visited = Grid::from_data(vec![false; (w * h) as usize], w as usize);
    let mut stack = Vec::new();
    let mut total = 0;
    for p in g.points_iter() {
        if visited[p] {
            continue;
        }
        visited.set(p, true);
        let typ = g[p];
        let mut area = 0;
        stack.clear();
        stack.push(p);
        let mut sides: HashMap<_, Vec<isize>> = HashMap::new();
        while let Some(p) = stack.pop() {
            area += 1;
            for dir in (0..4).map(Direction::new) {
                let neighbor = p + dir;
                if g.get(neighbor) == Some(&typ) {
                    if !visited[neighbor] {
                        stack.push(neighbor);
                        visited.set(neighbor, true);
                    }
                    continue;
                }
                let (k, v) = if dir == Direction::NORTH || dir == Direction::SOUTH {
                    (p.y(), p.x())
                } else {
                    (p.x(), p.y())
                };
                sides.entry((k, dir)).or_default().push(v);
            }
        }
        let perimeter = sides
            .into_values()
            .map(|mut points| {
                points.sort_unstable();
                points
                    .into_iter()
                    .tuple_windows()
                    .map(|(p1, p2)| if p1.abs_diff(p2) > 1 { 1 } else { 0 })
                    .sum::<u64>()
                    + 1
            })
            .sum::<u64>();
        total += area * perimeter;
    }
    total
}
