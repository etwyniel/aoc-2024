use std::collections::HashMap;

use aoc_framework::*;

pub struct Day11;

impl_day!(Day11::{part1, part2}: 2024[11], r"125 17");

fn split(n: u64) -> Option<(u64, u64)> {
    let mut n_digits = 0;
    let mut temp = n;
    while temp > 0 {
        n_digits += 1;
        temp /= 10;
    }
    if n_digits % 2 == 1 {
        return None;
    }
    let mut l = n;
    let mut r = 0;
    for i in 0..n_digits / 2 {
        r += (l % 10) * 10u64.pow(i);
        l /= 10;
    }
    Some((l, r))
}

#[allow(unused)]
fn iterate(nums: &mut Vec<u64>) {
    for i in 0..nums.len() {
        if nums[i] == 0 {
            nums[i] = 1;
            continue;
        }
        if let Some((l, r)) = split(nums[i]) {
            nums[i] = l;
            nums.push(r);
        } else {
            nums[i] *= 2024;
        }
    }
}

#[allow(unused)]
fn run(input: &str, iterations: usize) -> u64 {
    let mut nums = input
        .trim()
        .split(' ')
        .flat_map(|s| s.parse::<u64>().ok())
        .collect_vec();
    for _ in 0..iterations {
        iterate(&mut nums);
    }
    nums.len() as u64
}

#[aoc(part = 1, example = 55312)]
fn part1(input: &str) -> u64 {
    //run(input, 25)
    run_with_memo(
        input
            .trim()
            .split(' ')
            .flat_map(|s| s.parse::<u64>().ok())
            .collect_vec(),
        25,
    )
}

#[allow(unused)]
fn run_with_cull(input: &str) -> u64 {
    let mut precalc = [[0; 30]; 10];
    for (n, precalc_n) in precalc.iter_mut().enumerate() {
        let mut v = vec![n as u64];
        for res in precalc_n {
            iterate(&mut v);
            *res = v.len() as u64;
        }
    }
    let mut total = 0;
    let mut nums = input
        .trim()
        .split(' ')
        .flat_map(|s| s.parse::<u64>().ok())
        .collect_vec();
    for it in 0..75 {
        iterate(&mut nums);
        if (45..74).contains(&it) {
            let mut i = 0;
            loop {
                if i >= nums.len() {
                    break;
                }
                if nums[i] >= 10 {
                    i += 1;
                    continue;
                }
                total += precalc[nums[i] as usize][(75 - it) - 2];
                let last = nums.pop().unwrap();
                if i == nums.len() {
                    break;
                }
                nums[i] = last;
            }
        }
    }
    total + nums.len() as u64
}

fn run_with_memo(nums: Vec<u64>, total_steps: u64) -> u64 {
    let mut memo = HashMap::new();
    let mut stack = nums.iter().copied().map(|n| (total_steps, n)).collect_vec();
    let mut total = 0;
    while let Some((steps, n)) = stack.last().copied() {
        if steps == 0 {
            memo.insert((steps, n), 1);
            stack.pop();
            continue;
        }
        let mut get_next = |(steps, n)| {
            if let Some(&count) = memo.get(&(steps, n)) {
                return Some(count);
            }
            let mut get_or_push = |item| {
                if let Some(&res) = memo.get(&item) {
                    Some(res)
                } else {
                    stack.push(item);
                    None
                }
            };
            if n == 0 {
                return get_or_push((steps - 1, 1));
            }
            let Some((l, r)) = split(n) else {
                return get_or_push((steps - 1, n * 2024));
            };
            get_or_push((steps - 1, l))
                .zip(get_or_push((steps - 1, r)))
                .map(|(l, r)| l + r)
        };
        if let Some(res) = get_next((steps, n)) {
            if steps == total_steps {
                total += res;
            }
            memo.insert((steps, n), res);
            stack.pop();
        }
    }
    total
}

#[aoc(part = 2)]
fn part2(input: &str) -> u64 {
    run_with_memo(
        input
            .trim()
            .split(' ')
            .flat_map(|s| s.parse::<u64>().ok())
            .collect_vec(),
        75,
    )
}
