use std::collections::HashSet;

use aoc_framework::*;
use direction::Direction;
use grid::Grid;
use point::{Point, Point2};

pub struct Day15;

impl_day!(Day15::{part1, part2}: 2024[15], r"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
");

fn is_valid_move(g: &Grid<u8, 2>, pos: Point2, dir: Direction<2>) -> bool {
    match g.get(pos + dir) {
        None | Some(b'#') => false,
        Some(b'O') => is_valid_move(g, pos + dir, dir),
        Some(b'.') => true,
        _ => unreachable!(),
    }
}

#[aoc(part = 1, example = 10092)]
fn part1(mut input: impl Iterator<Item = String>) -> u64 {
    let mut lines = 0;
    let grid_lines = (&mut input)
        .take_while(|ln| !ln.is_empty())
        .inspect(|_| {
            lines += 1;
        })
        .flat_map(|ln| ln.into_bytes())
        .collect_vec();
    let columns = grid_lines.len() / lines;
    let mut grid = Grid::from_data(grid_lines, columns);
    let start = grid.points_iter().find(|&p| grid[p] == b'@').unwrap();
    let mut pos = start;
    for b in input.flat_map(|ln| ln.into_bytes()) {
        let dir = match b {
            b'^' => Direction::NORTH,
            b'>' => Direction::EAST,
            b'v' => Direction::SOUTH,
            b'<' => Direction::WEST,
            _ => unreachable!(),
        };
        if !is_valid_move(&grid, pos, dir) {
            continue;
        }
        let mut cur = pos;
        let mut prev = b'.';
        loop {
            let current = grid[cur];
            grid.set(cur, prev);
            if current == b'.' {
                break;
            }
            prev = current;
            cur += dir;
        }
        pos += dir;
    }
    grid.points_iter()
        .filter(|&p| grid[p] == b'O')
        .map(|Point([x, y])| y * 100 + x)
        .sum::<isize>() as u64
}

fn is_valid_move_p2(
    g: &Grid<u8, 2>,
    pos: Point2,
    dir: Direction<2>,
    moves: &mut HashSet<(Point2, Point2)>,
) -> bool {
    let vertical = dir == Direction::NORTH || dir == Direction::SOUTH;
    let valid = match g.get(pos + dir) {
        None | Some(b'#') => false,
        Some(b'[') => {
            is_valid_move_p2(g, pos + dir, dir, moves)
                && (!vertical || is_valid_move_p2(g, pos + dir + Direction::EAST, dir, moves))
        }
        Some(b']') => {
            is_valid_move_p2(g, pos + dir, dir, moves)
                && (!vertical || is_valid_move_p2(g, pos + dir + Direction::WEST, dir, moves))
        }
        Some(b'@') => is_valid_move_p2(g, pos + dir, dir, moves),
        Some(b'.') => true,
        _ => unreachable!(),
    };
    if valid {
        moves.insert((pos, pos + dir));
    }
    valid
}

fn execute_move(g: &mut Grid<u8, 2>, pos: Point2, dir: Direction<2>) -> bool {
    let mut moves = HashSet::new();
    if !is_valid_move_p2(g, pos, dir, &mut moves) {
        return false;
    }
    moves
        .into_iter()
        .map(|(src, dst)| {
            let obj = g[src];
            g.set(src, b'.');
            (obj, dst)
        })
        .collect_vec()
        .into_iter()
        .for_each(|(obj, dst)| {
            g.set(dst, obj);
        });
    true
}

#[aoc(part = 2, example = 9021)]
fn part2(mut input: impl Iterator<Item = String>) -> u64 {
    let mut lines = 0;
    let grid_lines = (&mut input)
        .take_while(|ln| !ln.is_empty())
        .inspect(|_| {
            lines += 1;
        })
        .flat_map(|ln| ln.into_bytes())
        .flat_map(|b| match b {
            b'#' | b'.' => [b, b],
            b'O' => [b'[', b']'],
            b'@' => [b'@', b'.'],
            _ => unreachable!(),
        })
        .collect_vec();
    let columns = grid_lines.len() / lines;
    let mut grid = Grid::from_data(grid_lines, columns);
    let start = grid.points_iter().find(|&p| grid[p] == b'@').unwrap();
    let mut pos = start;
    for b in input.flat_map(|ln| ln.into_bytes()) {
        let dir = match b {
            b'^' => Direction::NORTH,
            b'>' => Direction::EAST,
            b'v' => Direction::SOUTH,
            b'<' => Direction::WEST,
            _ => unreachable!(),
        };
        if execute_move(&mut grid, pos, dir) {
            pos += dir;
        }
    }
    grid.points_iter()
        .filter(|&p| grid[p] == b'[')
        .map(|Point([x, y])| y * 100 + x)
        .sum::<isize>() as u64
}
