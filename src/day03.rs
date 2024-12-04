use aoc_framework::*;

const FUNCS: [&str; 3] = ["mul(", "do(", "don't("];

pub struct Day03;

impl_day!(Day03::{part1, part2}: 2024[3],
    r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");

struct Lexer<'a> {
    buf: &'a str,
    pos: usize,
}

impl Lexer<'_> {
    fn step(&mut self) -> Option<u8> {
        let res = self.buf.as_bytes().get(self.pos).copied();
        self.pos += 1;
        res
    }

    fn step_to_mul(&mut self) -> Option<()> {
        let offset = self.buf[self.pos..].find("mul(")?;
        self.pos += offset + 4;
        Some(())
    }

    fn step_to_func(&mut self) -> Option<&'static str> {
        let (func, offset) = FUNCS
            .iter()
            .copied()
            .flat_map(|func| self.buf[self.pos..].find(func).map(|ndx| (func, ndx)))
            .min_by_key(|(_, ndx)| *ndx)?;
        self.pos += offset + func.len();
        Some(func)
    }

    fn last(&self) -> Option<u8> {
        self.buf.as_bytes().get(self.pos.saturating_sub(1)).copied()
    }

    fn parse_arg(&mut self) -> Option<u64> {
        let mut n = 0;
        let mut count = 0;
        while let Some(b) = self.step() {
            if !b.is_ascii_digit() {
                break;
            }
            count += 1;
            if count > 3 {
                return None;
            }
            n = n * 10 + (b - b'0') as u64;
        }
        (count > 0).then_some(n)
    }

    fn parse_mul(&mut self) -> Option<(u64, u64)> {
        //self.step_to_mul()?;
        let lhs = self.parse_arg()?;
        let Some(b',') = self.last() else {
            return None;
        };
        let rhs = self.parse_arg()?;
        let Some(b')') = self.last() else {
            return None;
        };
        Some((lhs, rhs))
    }
}

#[aoc(part = 1, example = 161)]
fn part1(input: &str) -> u64 {
    let mut lex = Lexer { buf: input, pos: 0 };
    let mut sum = 0;
    while lex.step_to_mul().is_some() {
        let Some((lhs, rhs)) = lex.parse_mul() else {
            continue;
        };
        sum += lhs * rhs;
    }
    sum
}

#[aoc(part = 2, example = 48)]
fn part2(input: &str) -> u64 {
    let mut lex = Lexer { buf: input, pos: 0 };
    let mut sum = 0;
    let mut enabled = true;
    while let Some(func) = lex.step_to_func() {
        if func == "mul(" {
            if !enabled {
                continue;
            }

            let Some((lhs, rhs)) = lex.parse_mul() else {
                continue;
            };
            sum += lhs * rhs;
            continue;
        }
        let Some(b')') = lex.step() else {
            continue;
        };
        enabled = func == "do(";
    }
    sum
}
