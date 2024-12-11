#![feature(test)]

use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

type Grid = [[u8; N]; N];
type Point = (usize, usize);

const N: usize = 53;

fn parse() -> (Grid, Vec<Point>) {
    let mut grid = [[0u8; N]; N];

    let mut heads = vec![];

    for (x, line) in include_str!("input.txt").lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            let digit = c.to_digit(10).unwrap() as u8;
            grid[x][y] = digit;

            if digit == 0 {
                heads.push((x, y));
            }
        }
    }

    (grid, heads)
}

fn bfs(grid: &Grid, start: Point) -> usize {
    let mut queue = VecDeque::from([start]);
    let mut dest = HashSet::new();

    while let Some((x, y)) = queue.pop_front() {
        if grid[x][y] == 9 {
            dest.insert((x, y));
            continue;
        }

        for (dx, dy) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let Some((xx, yy)) = x.checked_add_signed(dx).zip(y.checked_add_signed(dy)) else {
                continue;
            };

            if xx >= grid.len() || yy >= grid.len() {
                continue;
            }

            if grid[xx][yy].checked_sub(grid[x][y]) == Some(1) {
                queue.push_back((xx, yy));
            }
        }
    }

    dest.len()
}

fn part1() -> usize {
    let (grid, heads) = parse();
    heads.iter().map(|point| bfs(&grid, *point)).sum()
}

fn dfs(grid: &Grid, (x, y): Point, map: &mut HashMap<Point, u32>) -> u32 {
    if let Some(score) = map.get(&(x, y)) {
        return *score;
    }

    if grid[x][y] == 9 {
        map.insert((x, y), 1);
        return 1;
    }

    let mut score = 0;

    for (dx, dy) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let Some((xx, yy)) = x.checked_add_signed(dx).zip(y.checked_add_signed(dy)) else {
            continue;
        };

        if xx >= grid.len() || yy >= grid.len() {
            continue;
        }

        if grid[xx][yy].checked_sub(grid[x][y]) == Some(1) {
            score += dfs(grid, (xx, yy), map);
        }
    }

    map.insert((x, y), score);

    score
}

fn part2() -> u32 {
    let (grid, heads) = parse();

    let mut map = HashMap::new();

    heads.iter().map(|point| dfs(&grid, *point, &mut map)).sum()
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 10 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 776);
    assert_eq!(part2, 1657);
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
