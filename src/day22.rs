use std::collections::{HashMap, HashSet};

use aoc_framework::*;

pub struct Day22;

impl_day!(Day22::{part1, part2}: 2024[22], r"
1
10
100
2024
",
"
1
2
3
2024
");

const MOD: u64 = 16777216;

fn mix_prune(num: u64, res: u64) -> u64 {
    (num ^ res) % MOD
}

fn next(mut num: u64) -> u64 {
    num = mix_prune(num, num * 64);
    num = mix_prune(num, num / 32);
    mix_prune(num, num * 2048)
}

struct Secret(u64);

impl Iterator for Secret {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let cur = self.0;
        self.0 = next(self.0);
        Some(cur)
    }
}

#[aoc(part = 1, example = 37327623)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    input
        .flat_map(|ln| ln.parse::<u64>().ok())
        .map(|n| (0..2000).fold(n, |num, _| next(num)))
        .sum()
}

#[aoc(part = 2, example = 23)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    let mut sequences = HashMap::new();
    input.flat_map(|ln| ln.parse::<u64>().ok()).for_each(|n| {
        let mut seen = HashSet::new();
        Secret(n)
            .take(2000)
            .map(|n| (n % 10) as i8)
            .tuple_windows()
            .map(|(l, r)| (r - l, r))
            .tuple_windows()
            .for_each(|((d0, _), (d1, _), (d2, _), (d3, n))| {
                let seq = [d0, d1, d2, d3];
                if seen.insert(seq) {
                    *sequences.entry(seq).or_default() += n as u64;
                }
            })
    });
    let (_seq, val) = sequences
        .into_iter()
        .max_by_key(|(_, n)| *n)
        .unwrap_or_default();
    val
}
