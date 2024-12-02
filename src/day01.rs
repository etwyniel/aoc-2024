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

#[aoc(part = 1, example = 11, benchmark = 10_000)]
fn part1(input: &str) -> u64 {
    let (mut left, mut right) = input
        .lines()
        .flat_map(|ln| {
            ln.split_once("   ")
                .and_then(|(l, r)| Some((l.parse::<i64>().ok()?, r.parse::<i64>().ok()?)))
        })
        .fold(
            (
                BinaryHeap::with_capacity(1000),
                BinaryHeap::with_capacity(1000),
            ),
            |(mut left, mut right), (l, r)| {
                left.push(l);
                right.push(r);
                (left, right)
            },
        );
    let mut total = 0;
    while let Some((l, r)) = left.pop().zip(right.pop()) {
        total += l.abs_diff(r);
    }
    total
}

#[aoc(part = 2, example = 31, benchmark = 10_000)]
fn part2(input: &str) -> u64 {
    let mut left = HashMap::<_, u64>::with_capacity(1000);
    let mut right = HashMap::<_, u64>::with_capacity(1000);
    input
        .lines()
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
