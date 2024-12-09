use aoc_framework::*;

pub struct Day09;

impl_day!(Day09::{part1, part2}: 2024[9], r"2333133121414131402");

fn checksum(blocks: Vec<Option<usize>>) -> u64 {
    blocks
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|b| i * b))
        .sum::<usize>() as u64
}

#[aoc(part = 1, example = 1928)]
fn part1(input: &str) -> u64 {
    let mut blocks = Vec::new();
    for (i, b) in input.trim_end().bytes().enumerate() {
        let n = b - b'0';
        let block = if i % 2 == 0 { Some(i / 2) } else { None };
        for _ in 0..n {
            blocks.push(block);
        }
    }
    let mut free_cursor = 0;
    let mut move_cursor = blocks.len() - 1;
    loop {
        while blocks[move_cursor].is_none() {
            move_cursor -= 1;
        }
        while blocks[free_cursor].is_some() {
            free_cursor += 1;
        }
        if free_cursor >= move_cursor {
            break;
        }
        blocks.swap(free_cursor, move_cursor);
    }
    checksum(blocks)
}

#[aoc(part = 2, example = 2858)]
fn part2(input: &str) -> u64 {
    let mut blocks = Vec::new();
    for (i, b) in input.trim_end().bytes().enumerate() {
        let n = b - b'0';
        if n == 0 {
            continue;
        }
        let block = if i % 2 == 0 { Some(i / 2) } else { None };
        blocks.push((n, block));
    }
    let mut move_cursor = blocks.len();
    while move_cursor > 0 {
        move_cursor -= 1;
        let (size, Some(b)) = blocks[move_cursor] else {
            continue;
        };
        let Some(free) = blocks.iter().position(|(sz, b)| *sz >= size && b.is_none()) else {
            continue;
        };
        if free > move_cursor {
            continue;
        }
        blocks[move_cursor].1 = None;
        if blocks[free].0 == size {
            blocks[free].1 = Some(b);
            continue;
        }
        blocks[free].0 -= size;
        blocks.insert(free, (size, Some(b)));
        move_cursor += 1;
    }
    let mut sum = 0;
    let mut pos = 0;
    for (size, b) in blocks {
        let Some(b) = b else {
            pos += size as usize;
            continue;
        };
        for _ in 0..size {
            sum += pos * b;
            pos += 1;
        }
    }
    sum as u64
}
