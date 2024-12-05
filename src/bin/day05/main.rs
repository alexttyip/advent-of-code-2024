#![feature(test)]

use itertools::Itertools;
use std::cmp::Ordering;
use std::time::Instant;

fn calculate(part1: bool) -> usize {
    let (rules, pages) = include_str!("input.txt").split_once("\n\n").unwrap();

    let mut orderings = [[Ordering::Equal; 100]; 100];

    for mut chunk in &rules
        .split(&['\n', '|'])
        .filter_map(|s| s.parse::<usize>().ok())
        .chunks(2)
    {
        let i1 = chunk.next().unwrap();
        let i2 = chunk.next().unwrap();

        orderings[i1][i2] = Ordering::Less;
        orderings[i2][i1] = Ordering::Greater;
    }

    let mut ans = 0;

    for page in pages.lines() {
        let list = page
            .split(',')
            .filter_map(|item| item.parse::<usize>().ok())
            .collect_vec();

        let list2 = list
            .iter()
            .sorted_unstable_by(|&&lhs, &&rhs| orderings[lhs][rhs])
            .copied()
            .collect_vec();

        if part1 == (list == list2) {
            ans += list2[(list2.len() - 1) / 2];
        }
    }

    ans
}

fn part1() -> usize {
    calculate(true)
}

fn part2() -> usize {
    calculate(false)
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 05 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 5275);
    assert_eq!(part2, 6191);
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
