use std::collections::HashMap;

use aoc_framework::*;
use direction::Direction;
use grid::Grid;
use point::{Point, Point2};

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
        let mut sides: HashMap<_, Vec<Point2>> = HashMap::new();
        while let Some(p) = stack.pop() {
            area += 1;
            for dir in (0..4).map(Direction::new) {
                let neighbor = p + dir;
                match g.get(neighbor) {
                    Some(&val) if val == typ => {
                        if !visited[neighbor] {
                            stack.push(neighbor);
                            visited.set(neighbor, true);
                        }
                    }
                    _ => {
                        let edge = if dir == Direction::NORTH || dir == Direction::SOUTH {
                            (None, Some(p.y()), dir)
                        } else {
                            (Some(p.x()), None, dir)
                        };
                        sides.entry(edge).or_default().push(p);
                    }
                }
            }
        }
        let perimeter: u64 = sides
            .into_values()
            .map(|mut points| {
                points.sort_by(|p1, p2| {
                    if p1.x() == p2.x() {
                        p1.y().cmp(&p2.y())
                    } else {
                        p1.x().cmp(&p2.x())
                    }
                });
                points.into_iter().tuple_windows().fold(1, |acc, (p1, p2)| {
                    if p1.dist_manhattan(p2) > 1 {
                        acc + 1
                    } else {
                        acc
                    }
                })
            })
            .sum();
        total += area * perimeter;
    }
    total
}
