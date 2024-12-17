#![feature(test)]

use itertools::Itertools;
use std::collections::VecDeque;
use std::time::Instant;

fn resolve_combo_operand(registers: &[usize; 3], operand: usize) -> usize {
    match operand {
        0..=3 => operand,
        4..=6 => registers[operand - 4],
        _ => panic!(),
    }
}

fn parse() -> ([usize; 3], Vec<usize>) {
    let (registers_str, program_str) = include_str!("input.txt").split_once("\n\n").unwrap();

    let mut registers_iter = registers_str
        .lines()
        .filter_map(|line| line[12..].parse::<usize>().ok());

    let mut registers = [0; 3];
    registers.fill_with(|| registers_iter.next().unwrap());

    let program = program_str[9..]
        .chars()
        .filter_map(|c| {
            if c == ',' {
                None
            } else {
                c.to_digit(10).map(|i| i as usize)
            }
        })
        .collect_vec();

    (registers, program)
}
fn execute(program: &[usize], mut registers: [usize; 3]) -> Vec<usize> {
    let mut ptr = 0;
    let mut output = vec![];

    while ptr < program.len() {
        let opcode = program[ptr];
        let operand = program[ptr + 1];
        let combo_operand = resolve_combo_operand(&registers, operand);
        ptr += 2;

        match opcode {
            0 | 6 | 7 => registers[opcode % 5] = registers[0] / 2usize.pow(combo_operand as u32),
            1 => registers[1] ^= operand,
            2 => registers[1] = combo_operand % 8,
            3 => {
                if registers[0] != 0 {
                    ptr = operand
                }
            }
            4 => registers[1] ^= registers[2],
            5 => output.push(combo_operand % 8),
            _ => panic!(),
        }
    }

    output
}

fn part1() -> String {
    let (registers, program) = parse();

    let output = execute(&program, registers);

    output.into_iter().join(",")
}

fn part2() -> usize {
    let (_, program) = parse();

    let mut queue = VecDeque::from([0]);

    while let Some(a_head) = queue.pop_front() {
        for potential_tail in 0..8 {
            let a = a_head * 8 + potential_tail;

            let attempt = execute(&program, [a, 0, 0]);

            if program.ends_with(&attempt) {
                if program == attempt {
                    return a;
                }

                queue.push_back(a);
            }
        }
    }

    panic!("No solution")
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 17 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, "6,7,5,2,1,3,5,1,7");
    assert_eq!(part2, 216549846240877);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
