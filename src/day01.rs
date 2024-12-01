use std::collections::{BinaryHeap, HashMap};

use aoc_framework::*;

pub struct Day01;

impl_day!(Day01::{part1, part2}: 2024[1], r"
3   4
4   3
2   5
1   3
3   9
3   3
");

#[aoc(part = 1, example = 11)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();
    input
        .flat_map(|ln| {
            ln.split_once(' ')
                .map(|(l, r)| (l.parse::<i64>().unwrap(), r.trim().parse::<i64>().unwrap()))
        })
        .for_each(|(l, r)| {
            left.push(l);
            right.push(r);
        });
    let mut total = 0;
    while let Some((l, r)) = left.pop().zip(right.pop()) {
        total += (l - r).unsigned_abs();
    }
    total
}

#[aoc(part = 2, example = 31)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    let mut left = HashMap::<_, u64>::new();
    let mut right = HashMap::<_, u64>::new();
    input
        .flat_map(|ln| {
            ln.split_once(' ')
                .map(|(l, r)| (l.parse::<u64>().unwrap(), r.trim().parse::<u64>().unwrap()))
        })
        .for_each(|(l, r)| {
            *left.entry(l).or_default() += 1;
            *right.entry(r).or_default() += 1;
        });
    left.into_iter()
        .map(|(l, n)| l * n * right.get(&l).copied().unwrap_or_default())
        .sum()
}