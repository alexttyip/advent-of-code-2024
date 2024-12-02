#![feature(test)]

use itertools::Itertools;
use std::time::Instant;

fn check(diffs: &[i16]) -> bool {
    diffs.iter().map(|diff| diff.signum()).all_equal()
        && diffs.iter().all(|&diff| 1 <= diff.abs() && diff.abs() <= 3)
}

fn part1() -> usize {
    include_str!("input.txt")
        .lines()
        .filter_map(|line| {
            line.split_ascii_whitespace()
                .tuple_windows()
                .try_fold(0, |signum, (s1, s2)| {
                    let diff = s2.parse::<i16>().ok()? - s1.parse::<i16>().ok()?;

                    (1 <= diff.abs() && diff.abs() <= 3 && (signum == 0 || diff.signum() == signum))
                        .then_some(diff.signum())
                })
        })
        .count()
}

fn part2() -> u16 {
    let mut ans = 0;

    for line in include_str!("input.txt").lines() {
        let diffs = line
            .split_ascii_whitespace()
            .filter_map(|s| s.parse::<i16>().ok())
            .tuple_windows()
            .map(|(i1, i2)| i2 - i1)
            .collect_vec();

        if check(&diffs) || check(&diffs[1..]) || check(&diffs[..diffs.len() - 1]) {
            ans += 1;
            continue;
        }

        for i in 0..diffs.len() - 1 {
            let mut skipped_diff = diffs.clone();
            skipped_diff[i] += skipped_diff.remove(i + 1);

            if check(&skipped_diff) {
                ans += 1;
                break;
            }
        }
    }

    ans
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 02 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 631);
    assert_eq!(part2, 665);
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
