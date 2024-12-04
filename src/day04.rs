use aoc_framework::*;
use grid::Grid;
use point::Point;

pub struct Day04;

impl_day!(Day04::{part1, part2}: 2024[4], r"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
");

const DELTAS: [(isize, isize); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
const LETTERS: [u8; 4] = [b'X', b'M', b'A', b'S'];

fn count_xmas(g: &Grid<u8, 2>) -> u64 {
    g.points_iter()
        .filter(|&p| g[p] == b'X')
        .map(|p| {
            DELTAS
                .into_iter()
                .map(|(dx, dy)| Point([dx, dy]))
                .filter(|&d| (1..4).all(|i| g.get(p + d * i) == Some(&LETTERS[i as usize])))
                .count()
        })
        .sum::<usize>() as u64
}

const DIAG_DELTAS: [[isize; 2]; 2] = [[1, 1], [1, -1]];

fn count_x_mas(g: &Grid<u8, 2>) -> u64 {
    g.points_iter()
        .filter(|&p| g[p] == b'A')
        .filter(|&p| {
            DIAG_DELTAS
                .into_iter()
                .map(Point)
                .map(|d| Some((*g.get(p - d)?, *g.get(p + d)?)))
                .all(|v| matches!(v, Some((b'M', b'S') | (b'S', b'M'))))
        })
        .count() as u64
}

#[aoc(part = 1, example = 18)]
fn part1(input: Vec<u8>) -> u64 {
    let g = Grid::from_bytes(input);
    count_xmas(&g)
}

#[aoc(part = 2, example = 9)]
fn part2(input: Vec<u8>) -> u64 {
    let g = Grid::from_bytes(input);
    count_x_mas(&g)
}
