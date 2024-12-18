use aoc_framework::*;

use rayon::prelude::*;

pub struct Day17;

impl_day!(Day17::{part1, part2}: 2024[17], r"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
",
"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
");

#[derive(Clone)]
struct Cpu {
    a: u64,
    b: u64,
    c: u64,
    pc: usize,
    program: Vec<u8>,
    output: Vec<u8>,
}

impl Cpu {
    fn combo(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => u64::MAX,
        }
    }
    fn cycle(&mut self) -> Option<Option<u8>> {
        let opcode = *self.program.get(self.pc)?;
        let operand = *self.program.get(self.pc + 1)?;
        let mut out = None;
        match opcode {
            0 => self.a /= 2u64.pow(self.combo(operand) as u32),
            1 => self.b ^= operand as u64,
            2 => self.b = self.combo(operand) % 8,
            3 => {
                if self.a != 0 {
                    self.pc = operand as usize;
                    return Some(None);
                }
            }
            4 => self.b ^= self.c,
            5 => {
                let res = (self.combo(operand) % 8) as u8;
                self.output.push(res);
                out = Some(res);
            }
            6 => self.b = self.a / 2u64.pow(self.combo(operand) as u32),
            7 => self.c = self.a / 2u64.pow(self.combo(operand) as u32),
            _ => unreachable!(),
        }
        self.pc += 2;
        Some(out)
    }
}

fn parse_cpu(mut input: impl Iterator<Item = String>) -> Option<Cpu> {
    let (a, b, c) = (&mut input)
        .take(3)
        .flat_map(|ln| ln.split_once(": ").and_then(|(_, s)| s.parse::<u64>().ok()))
        .tuples()
        .next()?;
    input.next();
    let program = input
        .next()
        .and_then(|ln| {
            ln.split_once(": ").map(|(_, s)| {
                s.split(',')
                    .flat_map(|b| b.parse::<u8>().ok())
                    .collect_vec()
            })
        })
        .unwrap_or_default();
    Some(Cpu {
        a,
        b,
        c,
        pc: 0,
        output: Vec::new(),
        program,
    })
}

#[aoc(part = 1, example = "4,6,3,5,6,3,5,2,1,0")]
fn part1(input: impl Iterator<Item = String>) -> String {
    let Some(mut cpu) = parse_cpu(input) else {
        return String::new();
    };
    while cpu.pc < cpu.program.len() {
        if cpu.cycle().is_none() {
            break;
        }
    }
    cpu.output.into_iter().join(",")
}

const TARGET: u64 = 0o2411751540550330;

fn solve_p2(a: u64, tgt: u64) -> Option<u64> {
    if tgt == 0 {
        return Some(a);
    }
    for x in 0..8 {
        let next_a = a << 3 | x;
        let mut b = x ^ 1;
        let c = next_a >> b;
        b ^= 5;
        b ^= c;
        if b & 0b111 == tgt & 0b111 {
            if let Some(res) = solve_p2(next_a, tgt >> 3) {
                return Some(res);
            }
        }
    }
    None
}

fn solve_generic(a: u64, i: usize, cpu: &mut Cpu) -> Option<u64> {
    if i == cpu.program.len() {
        return Some(a);
    }
    let target = cpu.program[cpu.program.len() - 1 - i];
    for x in 0..8 {
        let next_a = a << 3 | x;
        cpu.a = next_a;
        cpu.b = 0;
        cpu.c = 0;
        cpu.pc = 0;
        while let Some(res) = cpu.cycle() {
            if let Some(out) = res {
                if out != target {
                    break;
                }
                if let Some(solution) = solve_generic(next_a, i + 1, cpu) {
                    return Some(solution);
                }
                break;
            }
            if cpu.program[cpu.pc] == b'3' {
                break;
            }
        }
    }
    None
}

#[aoc(part = 2, example = 117440)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    //solve_p2(0, TARGET).unwrap_or(0)
    let Some(mut cpu) = parse_cpu(input) else {
        return 0;
    };
    assert!(cpu
        .program
        .iter()
        .copied()
        .tuples::<(u8, u8)>()
        .contains(&(0, 3)));
    solve_generic(0, 0, &mut cpu).unwrap_or(0)
    //let init_b = cpu.b;
    //let init_c = cpu.b;
    //(4000000000000..)
    //    .step_by(1_000_000_000)
    //    .par_bridge()
    //    .find_map_first(|x| {
    //        dbg!(x);
    //        for i in 0..1_000_000_000 {
    //            let mut a = x + i;
    //            let mut out = 0;
    //            let mut count = 0;
    //            while a != 0 && (TARGET >> (48 - (3 * count))) == out {
    //                let mut b = a % 8;
    //                b ^= 1;
    //                let c = a >> b;
    //                b ^= 5;
    //                b ^= c;
    //                out = out << 3 | (b & 0b111);
    //                a >>= 3;
    //                count += 1;
    //            }
    //            if out == TARGET {
    //                return Some(x + i);
    //            }
    //        }
    //let mut cpu = cpu.clone();
    //dbg!(x);
    //'outer: for a in 0..100_000_000 {
    //    cpu.a = a + x;
    //    cpu.b = init_b;
    //    cpu.c = init_c;
    //    cpu.pc = 0;
    //    cpu.output.clear();
    //    while cpu.pc < cpu.program.len() {
    //        if cpu.cycle().is_none() {
    //            break;
    //        }
    //        if cpu.output.len() > cpu.program.len()
    //            || cpu.output != cpu.program[0..cpu.output.len()]
    //        {
    //            continue 'outer;
    //        }
    //    }
    //    if cpu.output == cpu.program {
    //        return Some(a + x);
    //    }
    //}
    //    None
    //})
    //.unwrap_or(0)
    //for a in 0.. {
    //    cpu.a = a;
    //    cpu.b = init_b;
    //    cpu.c = init_c;
    //    cpu.pc = 0;
    //    cpu.output.clear();
    //    if a % 100_000 == 0 {
    //        dbg!(a);
    //    }
    //    while cpu.pc < cpu.program.len() {
    //        if cpu.cycle().is_none() {
    //            break;
    //        }
    //    }
    //    if cpu.output == cpu.program {
    //        return a;
    //    }
    //}
    //0
}
