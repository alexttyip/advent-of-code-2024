#![feature(test)]

use itertools::Itertools;
use num::Integer;
use std::collections::HashSet;
use std::mem::{swap, take};
use std::time::Instant;

const N: usize = 141;

const CARDINALS: [isize; 4] = [-(N as isize), 1, N as isize, -1];

fn dijkstra(is_space: [bool; N * N], src: usize) -> ([usize; N * N], Vec<usize>) {
    let mut dist = [usize::MAX; N * N];
    dist[src] = 0;
    let mut path = vec![];

    let mut curr = Some(src);

    while let Some(u) = take(&mut curr) {
        path.push(u);

        let curr_weight = dist[u];

        for du in CARDINALS {
            let Some(v) = u.checked_add_signed(du) else {
                continue;
            };

            if is_space[v] && dist[v] > curr_weight + 1 {
                dist[v] = curr_weight + 1;
                curr = Some(v);
            }
        }
    }

    (dist, path)
}

fn parse() -> ([bool; N * N], usize) {
    let mut is_space = [true; N * N];
    let mut s = 0;

    for (x, line) in include_str!("input.txt").lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            let pos = x * N + y;

            is_space[pos] = match c {
                '.' | 'E' => true,
                'S' => {
                    s = pos;
                    true
                }
                '#' => false,
                _ => panic!(),
            };
        }
    }

    (is_space, s)
}

fn get_2_spaces_over(is_space: &[bool; N * N], u: usize) -> Vec<usize> {
    let mut vec = vec![];

    // Vertical
    for du in [-(N as isize), N as isize] {
        let Some(w) = u.checked_add_signed(du * 2) else {
            continue;
        };

        if w >= (N * N) || !is_space[w] {
            continue;
        }

        vec.push(w);
    }

    // Horizontal
    for du in [-1, 1] {
        let Some(w) = u.checked_add_signed(du * 2) else {
            continue;
        };

        if w >= (N * N) || (u / N) != (w / N) || !is_space[w] {
            continue;
        }

        vec.push(w);
    }

    vec
}

fn manhattan_dist(u: &usize, v: &usize) -> usize {
    let (ux, uy) = u.div_rem(&N);
    let (vx, vy) = v.div_rem(&N);

    let (dx, dy) = (ux.abs_diff(vx), uy.abs_diff(vy));

    dx + dy
}

fn part1() -> usize {
    let (is_space, s) = parse();

    let (dist, path) = dijkstra(is_space, s);

    let mut cheats = HashSet::new();
    let mut ans = 0;

    for u in path {
        for mut v in get_2_spaces_over(&is_space, u) {
            let mut u = u;

            if dist[u] > dist[v] {
                swap(&mut u, &mut v);
            }

            if cheats.insert((u, v)) {
                let new_dist_v = dist[u] + 2;

                if dist[v] - new_dist_v >= 100 {
                    ans += 1;
                }
            }
        }
    }

    ans
}

fn part2() -> usize {
    let (is_space, s) = parse();

    let (dist, path) = dijkstra(is_space, s);
    let mut ans = 0;

    for (&u, &v) in path.iter().tuple_combinations() {
        let distance = manhattan_dist(&u, &v);

        if distance > 20 {
            continue;
        }

        let new_dist_v = dist[u] + distance;

        if dist[v].saturating_sub(new_dist_v) >= 100 {
            ans += 1;
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

    println!("--- Day 20 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 1355);
    assert_eq!(part2, 1007335);
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
