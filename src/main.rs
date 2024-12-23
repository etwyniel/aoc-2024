use std::env::args;

use aoc_framework::*;

mod day01;

mod day02;

mod day03;

mod day04;

mod day05;

mod day06;

mod day07;

mod day08;

mod day09;

mod day10;

mod day11;

mod day12;

mod day13;

mod day14;

mod day15;

mod day16;

mod day17;

mod day18;

mod day19;

mod day20;

mod day21;

mod day22;

fn main() -> anyhow::Result<()> {
    let days = [
        day01::Day01::run,
        day02::Day02::run,
        day03::Day03::run,
        day04::Day04::run,
        day05::Day05::run,
        day06::Day06::run,
        day07::Day07::run,
        day08::Day08::run,
        day09::Day09::run,
        day10::Day10::run,
        day11::Day11::run,
        day12::Day12::run,
        day13::Day13::run,
        day14::Day14::run,
        day15::Day15::run,
        day16::Day16::run,
        day17::Day17::run,
        day18::Day18::run,
        day19::Day19::run,
        day20::Day20::run,
        day21::Day21::run,
        day22::Day22::run,
    ];

    let token = std::env::var("AOC_TOKEN").ok();

    if let Some(day) = args()
        .nth(1)
        .and_then(|arg| arg.parse::<usize>().ok())
        .and_then(|day| days.get(day - 1))
    {
        day(token.as_deref());
        return Ok(());
    }

    for day in days {
        day(token.as_deref());
    }

    Ok(())
}
