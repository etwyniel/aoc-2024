use std::cmp::Ordering;

use aoc_framework::*;

pub struct Day05;

impl_day!(Day05::{part1, part2}: 2024[5], r"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
");

struct Precedence([bool; 100 * 100]);

impl Precedence {
    fn parse(input: impl Iterator<Item = String>) -> Self {
        let mut mat = [false; 100 * 100];
        input
            .take_while(|ln| !ln.is_empty())
            .flat_map(|ln| {
                ln.split('|')
                    .flat_map(|s| s.parse::<usize>().ok())
                    .tuples()
                    .next()
            })
            .for_each(|(y, x)| mat[y * 100 + x] = true);
        Precedence(mat)
    }

    fn is_before(&self, y: usize, x: usize) -> bool {
        self.0[y * 100 + x]
    }
}

#[aoc(part = 1, example = 143)]
fn part1(mut input: impl Iterator<Item = String>) -> u64 {
    let prec = Precedence::parse(&mut input);
    let mut total = 0;
    'outer: for update in input {
        let mut indices: [Option<usize>; 100] = [None; 100];
        let mut pages = Vec::new();
        update
            .split(',')
            .flat_map(|s| s.parse::<usize>().ok())
            .inspect(|n| pages.push(*n))
            .enumerate()
            .for_each(|(i, n)| indices[n] = Some(i));
        for n in 0..100 {
            let Some(ndx) = indices[n] else { continue };
            for x in 0..100 {
                if !prec.is_before(n, x) {
                    continue;
                }
                let Some(x_ndx) = indices[x] else { continue };
                if ndx > x_ndx {
                    continue 'outer;
                }
            }
        }
        total += pages[pages.len() / 2] as u64;
    }
    total
}

#[aoc(part = 2, example = 123)]
fn part2(mut input: impl Iterator<Item = String>) -> u64 {
    let prec = Precedence::parse(&mut input);
    let mut total = 0;
    for update in input {
        let mut indices: [Option<usize>; 100] = [None; 100];
        let mut pages = Vec::new();
        update
            .split(',')
            .flat_map(|s| s.parse::<usize>().ok())
            .inspect(|n| pages.push(*n))
            .enumerate()
            .for_each(|(i, n)| indices[n] = Some(i));
        let mut correct = true;
        for n in 0..100 {
            let Some(ndx) = indices[n] else { continue };
            for x in 0..100 {
                if !prec.is_before(n, x) {
                    continue;
                }
                let Some(x_ndx) = indices[x] else { continue };
                if ndx > x_ndx {
                    correct = false;
                    break;
                }
            }
        }
        if correct {
            continue;
        }
        pages.sort_by(|&y, &x| {
            if prec.is_before(y, x) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        total += pages[pages.len() / 2] as u64;
    }
    total
}
