use std::{thread::sleep, time::Duration};

use aoc_framework::*;
use point::{Point, Point2};

pub struct Day14;

impl_day!(Day14::{part1, part2}: 2024[14], r"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
");

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

fn parse_point(s: &str) -> Option<Point2> {
    s.split(',')
        .flat_map(|s| s.parse::<isize>().ok())
        .tuples()
        .next()
        .map(|(x, y)| Point([x, y]))
}

fn parse_item(s: &str) -> Option<Point2> {
    parse_point(s.split_once('=')?.1)
}

fn parse_robot(s: &str) -> Option<(Point2, Point2)> {
    s.split(' ').flat_map(parse_item).tuples().next()
}

#[aoc(part = 1)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    let mut quadrants = [0; 4];
    for (pos, vel) in input.flat_map(|ln| parse_robot(&ln)) {
        let last = pos + vel * 100;
        let x = last.x().rem_euclid(WIDTH);
        let y = last.y().rem_euclid(HEIGHT);
        if x == WIDTH / 2 || y == HEIGHT / 2 {
            // Robots that are exactly in the middle (horizontally or vertically)
            // don't count as being in any quadrant
            continue;
        }
        quadrants[(y > HEIGHT / 2) as usize * 2 + (x > WIDTH / 2) as usize] += 1;
    }

    quadrants.into_iter().product()
}

#[aoc(part = 2)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    let mut robots = input.flat_map(|ln| parse_robot(&ln)).collect_vec();
    print!("\x1b[{0}S\x1b[{0}A", HEIGHT + 2); // scroll screen
    print!("\x1b[s"); // Save cursor position
    for i in 0.. {
        print!("\x1b[u\x1b[J");
        println!("{i:>4}s");
        println!();
        for (pos, vel) in &mut robots {
            print!("\x1b[u\x1b[{}B\x1b[{}C#", pos.y() + 2, pos.x());
            *pos += *vel;
            pos.0[0] = pos.0[0].rem_euclid(WIDTH);
            pos.0[1] = pos.0[1].rem_euclid(HEIGHT);
        }
        if i > 7280 {
            sleep(Duration::from_millis(1000));
        }
    }
    0
}
