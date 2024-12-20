use std::collections::HashMap;

use aoc_framework::*;

pub struct Day19;

impl_day!(Day19::{part1, part2}: 2024[19], r"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
");

fn is_valid(pattern: &str, towels: &[String]) -> bool {
    if pattern.is_empty() {
        return true;
    }
    for towel in towels {
        let Some(rem) = pattern.strip_prefix(towel) else {
            continue;
        };
        if is_valid(rem, towels) {
            return true;
        }
    }
    false
}

#[aoc(part = 1, example = 6)]
fn part1(mut input: impl Iterator<Item = String>) -> u64 {
    let towels = input
        .next()
        .unwrap_or_default()
        .split(", ")
        .map(|s| s.to_string())
        .collect_vec();
    input.next();
    input.filter(|s| is_valid(s, &towels)).count() as u64
}

fn count_valid(pattern: &str, towels: &[String], memo: &mut HashMap<String, u64>) -> u64 {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(&res) = memo.get(pattern) {
        return res;
    }
    let res = towels
        .iter()
        .flat_map(|towel| pattern.strip_prefix(towel))
        .map(|rem| count_valid(rem, towels, memo))
        .sum();
    memo.insert(pattern.to_string(), res);
    res
}

#[aoc(part = 2, example = 16)]
fn part2(mut input: impl Iterator<Item = String>) -> u64 {
    let towels = input
        .next()
        .unwrap_or_default()
        .split(", ")
        .map(|s| s.to_string())
        .collect_vec();
    input.next();
    let mut memo = HashMap::new();
    input.map(|s| count_valid(&s, &towels, &mut memo)).sum()
}
