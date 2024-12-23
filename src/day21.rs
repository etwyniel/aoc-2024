use std::fmt::{Debug, Write};

use aoc_framework::*;
use point::{Point, Point2};

pub struct Day21;

impl_day!(Day21::{part1, part2}: 2024[21], r"
029A
980A
179A
456A
379A
");

enum Action {
    Up,
    Right,
    Down,
    Left,
    Press,
}

use Action::*;

impl Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Up => '^',
            Right => '>',
            Down => 'v',
            Left => '<',
            Press => 'A',
        };
        f.write_char(c)
    }
}

type Move = Vec<Action>;

struct Numpad {
    pos: Point2,
    sequence: String,
    offset: usize,
}

impl Numpad {
    fn next_move(&mut self) -> Move {
        let mut result = vec![];
        let Some(button) = self.sequence.as_bytes().get(self.offset) else {
            return result;
        };
        let y: isize = match button {
            b'7' | b'8' | b'9' => 0,
            b'4' | b'5' | b'6' => 1,
            b'1' | b'2' | b'3' => 2,
            _ => 3,
        };
        let x: isize = match button {
            b'7' | b'4' | b'1' => 0,
            b'8' | b'5' | b'2' | b'0' => 1,
            _ => 2,
        };
        let Point([cur_x, cur_y]) = self.pos;
        for dx in 0..(x.saturating_sub(cur_x)) {
            result.push(Right);
        }
        for dx in 0..(cur_x.saturating_sub(x)) {
            result.push(Left);
        }
        for dy in 0..(y.saturating_sub(cur_y)) {
            result.push(Down);
        }
        for dx in 0..(cur_y.saturating_sub(y)) {
            result.push(Up);
        }
        result
    }
}

#[aoc(part = 1, example = 126384)]
fn part1(_input: impl Iterator<Item = String>) -> u64 {
    0
}

#[aoc(part = 2)]
fn part2(_input: impl Iterator<Item = String>) -> u64 {
    0
}
