#![feature(test)]

use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::time::Instant;

const N: usize = 141;

fn parse() -> ([bool; N * N], usize, usize) {
    let mut is_space = [true; N * N];
    let mut src = 0;
    let mut dest = 0;

    for (x, line) in include_str!("input.txt").lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            let pos = x * N + y;

            if c == 'S' {
                src = pos;
                continue;
            }

            if c == 'E' {
                dest = pos;
                continue;
            }

            if c == '#' {
                is_space[pos] = false;
            }
        }
    }

    (is_space, src, dest)
}

fn get_turn_weight(dir1: usize, dir2: usize) -> usize {
    let diff = dir1.abs_diff(dir2);
    match diff {
        0..3 => diff * 1000 + 1,
        _ => 1001,
    }
}

fn get_neighbours(u: usize) -> [usize; 4] {
    [u - N, u + 1, u + N, u - 1]
}

fn dijkstra(is_space: [bool; N * N], src: usize) -> [[usize; 4]; N * N] {
    let mut heap = BinaryHeap::from([(Reverse(0), src, 1)]);
    let mut dist = [[usize::MAX; 4]; N * N];
    dist[src].fill(0);

    while let Some((_, u, dir)) = heap.pop() {
        let curr_weight = dist[u][dir];

        for (turn_dir, &v) in get_neighbours(u).iter().enumerate() {
            if is_space[v] {
                let weight = get_turn_weight(dir, turn_dir);

                if dist[v][turn_dir] > curr_weight + weight {
                    dist[v][turn_dir] = curr_weight + weight;
                    heap.push((Reverse(dist[v][turn_dir]), v, turn_dir));
                }
            }
        }
    }

    dist
}

fn part1() -> usize {
    let (is_space, src, dest) = parse();

    let dist = dijkstra(is_space, src);

    *dist[dest].iter().min().unwrap()
}

fn part2() -> usize {
    let (is_space, src, dest) = parse();

    let dist = dijkstra(is_space, src);

    let dest_min_cost_dir_idx = dist[dest].iter().position_min().unwrap();

    let mut path = HashSet::new();

    let mut queue = BinaryHeap::from([(
        dist[dest][dest_min_cost_dir_idx],
        dest,
        dest_min_cost_dir_idx,
    )]);

    while let Some((cost, u, dir)) = queue.pop() {
        path.insert(u);

        if u == src {
            break;
        }

        for v in get_neighbours(u) {
            for (turn_dir, &v_cost) in dist[v].iter().enumerate() {
                let weight = get_turn_weight(dir, turn_dir);

                if Some(v_cost) == cost.checked_sub(weight) {
                    queue.push((v_cost, v, turn_dir));
                }
            }
        }
    }

    path.len()
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 16 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 143564);
    assert_eq!(part2, 593);
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
