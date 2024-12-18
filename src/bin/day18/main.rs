#![feature(test)]

use num::Integer;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::time::Instant;

const N: usize = 71;

fn get_neighbours(u: usize) -> Vec<usize> {
    let mut neighbours = vec![];

    if u >= N {
        neighbours.push(u - N);
    }

    if u.div_rem(&N).1 < N - 1 {
        neighbours.push(u + 1);
    }

    if u < N * (N - 1) {
        neighbours.push(u + N);
    }

    if u.div_rem(&N).1 >= 1 {
        neighbours.push(u - 1);
    }

    neighbours
}

fn dijkstra(is_space: [bool; N * N]) -> ([usize; N * N], [Option<usize>; N * N]) {
    let src = 0;
    let mut heap = BinaryHeap::from([(Reverse(0), src)]);
    let mut dist = [usize::MAX; N * N];
    dist[src] = 0;
    let mut prev = [None::<usize>; N * N];

    while let Some((_, u)) = heap.pop() {
        let curr_weight = dist[u];

        for &v in get_neighbours(u).iter() {
            if is_space[v] && dist[v] > curr_weight + 1 {
                dist[v] = curr_weight + 1;
                heap.push((Reverse(dist[v]), v));
                prev[v] = Some(u);
            }
        }
    }

    (dist, prev)
}

fn parse_line(line: &str) -> usize {
    let (x_str, y_str) = line.split_once(",").unwrap();
    let x = x_str.parse::<usize>().unwrap();
    let y = y_str.parse::<usize>().unwrap();

    y * N + x
}

fn part1() -> usize {
    let mut is_space = [true; N * N];

    for pos in include_str!("input.txt").lines().take(1024).map(parse_line) {
        is_space[pos] = false;
    }

    let (dist, _) = dijkstra(is_space);

    dist[N * N - 1]
}

fn part2() -> String {
    let mut is_space = [true; N * N];
    let mut path = HashSet::new();

    for (i, line) in include_str!("input.txt").lines().enumerate() {
        let pos = parse_line(line);

        is_space[pos] = false;

        if i == 1023 || path.contains(&pos) {
            let (dist, prev) = dijkstra(is_space);

            if dist[N * N - 1] == usize::MAX {
                return line.to_owned();
            }

            path.clear();

            let mut u = N * N - 1;
            while let Some(u_prev) = prev[u] {
                path.insert(u);
                u = u_prev;
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

    println!("--- Day 18 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 416);
    assert_eq!(part2, "50,23");
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
