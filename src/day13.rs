use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

use aoc_framework::*;

pub struct Day13;

impl_day!(Day13::{part1, part2}: 2024[13], r"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
");

#[derive(Clone, Copy)]
struct Equation {
    a: i64,
    b: i64,
    c: i64,
}

impl Debug for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = if self.b < 0 { '-' } else { '+' };
        let b = self.b.abs();
        write!(f, "{}x {sign} {}y = {}", self.a, b, self.c)
    }
}

impl Mul<i64> for Equation {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        let Equation { a, b, c } = self;
        Equation {
            a: a * rhs,
            b: b * rhs,
            c: c * rhs,
        }
    }
}

impl Add for Equation {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Equation {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
            c: self.c + rhs.c,
        }
    }
}

fn parse_button(ln: &str) -> Option<(i64, i64)> {
    ln.split_once(": ")?
        .1
        .split(", ")
        .flat_map(|s| s.split_once('+').map(|(_, val)| val))
        .flat_map(|val| val.parse().ok())
        .tuples()
        .next()
}

struct System {
    ex: Equation,
    ey: Equation,
}

impl System {
    fn solve(&self) -> Option<(i64, i64)> {
        let System { ex, ey } = self;
        let bx = ex.b;
        let by = ey.b;
        let eq_a = *ex * by + *ey * -bx;
        let a = eq_a.c / eq_a.a;
        if a * eq_a.a != eq_a.c {
            return None;
        }
        let b = (ex.c - ex.a * a) / ex.b;
        if a * ex.a + b * ex.b != ex.c {
            return None;
        }
        Some((a, b))
    }
}

fn parse_system(mut input: impl Iterator<Item = String>) -> Option<System> {
    let (ax, ay) = parse_button(&input.next()?)?;
    let (bx, by) = parse_button(&input.next()?)?;
    let (dx, dy) = input
        .next()?
        .split_once(": ")?
        .1
        .split(", ")
        .flat_map(|s| s.split_once('=').map(|(_, val)| val))
        .flat_map(|val| val.parse().ok())
        .tuples()
        .next()?;
    // skip empty line
    input.next();
    let ex = Equation {
        a: ax,
        b: bx,
        c: dx,
    };
    let ey = Equation {
        a: ay,
        b: by,
        c: dy,
    };
    Some(System { ex, ey })
}

#[aoc(part = 1, example = 480)]
fn part1(mut input: impl Iterator<Item = String>) -> u64 {
    let mut total = 0;
    loop {
        let Some(system) = parse_system(&mut input) else {
            break;
        };
        if let Some((a, b)) = system.solve() {
            total += a * 3 + b;
        }
    }
    total as u64
}

#[aoc(part = 2)]
fn part2(mut input: impl Iterator<Item = String>) -> u64 {
    let mut total = 0;
    loop {
        let Some(mut system) = parse_system(&mut input) else {
            break;
        };
        system.ex.c += 10000000000000;
        system.ey.c += 10000000000000;
        if let Some((a, b)) = system.solve() {
            total += a * 3 + b;
        }
    }
    total as u64
}
