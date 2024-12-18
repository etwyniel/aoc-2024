use aoc_framework::*;

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
        cpu.output.clear();
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
}
