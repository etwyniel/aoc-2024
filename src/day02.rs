use aoc_framework::*;

pub struct Day02;

impl_day!(Day02::{part1, part2}: 2024[2], r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");

#[aoc(part = 1, example = 2)]
fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|ln| {
            ln.split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .tuple_windows()
                .fold((true, None), |(ok, prev), (l, r)| {
                    let diff = l - r;
                    let sign = diff.signum();
                    (
                        ok && prev.map(|prev| prev == sign).unwrap_or(true) && diff.abs() < 4,
                        Some(sign),
                    )
                })
                .0 as u64
        })
        .sum()
}

fn check_rec(
    skip: Option<i64>,
    skipped: bool,
    prev: Option<i64>,
    mut arr: &[i64],
    last: Option<i64>,
) -> bool {
    if arr.is_empty() {
        return true;
    }
    let l = if let Some(skip) = skip {
        skip
    } else {
        let l = arr[0];
        arr = &arr[1..];
        l
    };
    if arr.is_empty() {
        return true;
    }
    let r = arr[0];
    let diff = r - l;
    let sign = diff.signum();
    let ok = prev.map(|prev| prev == sign).unwrap_or(true) && diff.abs() < 4 && sign != 0;
    (ok && check_rec(None, skipped, Some(sign), arr, Some(l)))
        || (!skipped
            && (check_rec(Some(l), true, prev, &arr[1..], last)
                || check_rec(last, true, prev, arr, None)))
}

#[aoc(part = 2, example = 4)]
fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|ln| ln.split(' ').map(|s| s.parse().unwrap()).collect_vec())
        .filter(|v| check_rec(None, false, None, v, None))
        .count() as u64
}
