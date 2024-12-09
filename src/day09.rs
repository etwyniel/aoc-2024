use std::fmt::Debug;

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

struct Block {
    values: Vec<(u8, usize)>,
    free: u8,
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.values.is_empty() {
            write!(f, "(")?
        }
        for (i, (size, b)) in self.values.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}x{}", size, b)?;
        }
        if !self.values.is_empty() {
            write!(f, ")")?
        }
        if self.free > 0 {
            write!(f, "+{}", self.free)?;
        }
        Ok(())
    }
}

fn update_first_frees(blocks: &[Block], first_frees: &mut [usize; 9]) {
    let mut last = 0;
    for (n, f) in first_frees.iter_mut().enumerate() {
        let mut ndx = (*f).max(last);
        while ndx < blocks.len() && blocks[ndx].free < n as u8 + 1 {
            ndx += 1;
        }
        *f = ndx;
        last = ndx;
    }
}

#[aoc(part = 2, example = 2858)]
fn part2(input: &str) -> u64 {
    let mut blocks = Vec::new();
    for (i, b) in input.trim_end().bytes().enumerate() {
        let n = b - b'0';
        if n == 0 {
            continue;
        }
        if i % 2 == 0 {
            blocks.push(Block {
                values: vec![(n, i / 2)],
                free: 0,
            });
        } else {
            blocks.last_mut().unwrap().free += n;
        }
    }
    let mut first_frees = [0; 9];
    for i in (0..blocks.len()).rev() {
        for j in (0..blocks[i].values.len()).rev() {
            let (size, b) = blocks[i].values[j];
            update_first_frees(&blocks, &mut first_frees);
            let free = first_frees[size as usize - 1];
            if free >= i {
                continue;
            }
            let blk = &mut blocks[free];
            blk.free -= size;
            blk.values.push((size, b));
            let old_blk = &mut blocks[i];
            if j == old_blk.values.len() - 1 {
                old_blk.free += size;
                old_blk.values.pop();
            } else if j == 0 {
                old_blk.values.remove(0);
                blocks[i - 1].free += size;
            } else {
                panic!("I didn't bother with this case")
            }
        }
    }
    let mut sum = 0;
    let mut pos = 0;
    for blk in blocks {
        for (size, b) in blk.values {
            for _ in 0..size {
                sum += pos * b;
                pos += 1;
            }
        }
        pos += blk.free as usize;
    }
    sum as u64
}
