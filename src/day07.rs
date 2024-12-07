use aoc_framework::*;

pub struct Day07;

impl_day!(Day07::{part1, part2}: 2024[7], r"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
");

fn check(target: u64, total: u64, nums: &[u64]) -> bool {
    if nums.is_empty() {
        return target == total;
    }
    let n = nums[0];
    let rem = &nums[1..];
    if total == 0 {
        return check(target, n, rem);
    }
    check(target, total + n, rem) || check(target, total * n, rem)
}

#[aoc(part = 1, example = 3749)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    input
        .flat_map(|ln| {
            let (target_str, lst) = ln.split_once(": ")?;
            let target = target_str.parse().ok()?;
            let nums = lst.split(' ').flat_map(|s| s.parse().ok()).collect_vec();
            check(target, 0, &nums).then_some(target)
        })
        .sum()
}

fn concat(mut i: u64, j: u64) -> u64 {
    let mut temp = j;
    while temp > 0 {
        i *= 10;
        temp /= 10;
    }
    i + j
}

fn check2(target: u64, total: u64, nums: &[u64]) -> bool {
    if nums.is_empty() {
        return target == total;
    }
    let n = nums[0];
    let rem = &nums[1..];
    if total == 0 {
        return check2(target, n, rem);
    }
    check2(target, total + n, rem)
        || check2(target, total * n, rem)
        || check2(target, concat(total, n), rem)
}

#[aoc(part = 2, example = 11387)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    input
        .flat_map(|ln| {
            let (target_str, lst) = ln.split_once(": ")?;
            let target = target_str.parse().ok()?;
            let nums = lst.split(' ').flat_map(|s| s.parse().ok()).collect_vec();
            check2(target, 0, &nums).then_some(target)
        })
        .sum()
}
