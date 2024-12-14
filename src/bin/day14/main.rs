#![feature(test)]

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;

const NX: i32 = 101;
const NY: i32 = 103;

fn parse() -> Vec<(i32, i32, i32, i32)> {
    let file = include_str!("input.txt");

    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut bots = vec![];

    for caps in re.captures_iter(file) {
        bots.push(
            caps.extract::<4>()
                .1
                .iter()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect_tuple::<(_, _, _, _)>()
                .unwrap(),
        );
    }

    bots
}

fn part1() -> i32 {
    let bots = parse();

    let mut quadrants = [0; 4];

    for (px, py, vx, vy) in bots.iter() {
        let px_100 = (px + vx * 100 + NX * 100) % NX;
        let py_100 = (py + vy * 100 + NY * 100) % NY;

        if py_100 == NY / 2 || px_100 == NX / 2 {
            continue;
        }

        let mut quadrant_idx = (py_100 > NY / 2) as usize;
        quadrant_idx *= 2;
        quadrant_idx += (px_100 > NX / 2) as usize;

        quadrants[quadrant_idx] += 1;
    }

    quadrants.iter().product()
}

fn part2() -> i32 {
    return 0;

    let bots = parse();

    for i in 0..10000 {
        let mut map: HashMap<(i32, i32), i32> = HashMap::new();

        for (px, py, vx, vy) in bots.iter() {
            let px_100 = (px + vx * i + NX * i) % NX;
            let py_100 = (py + vy * i + NY * i) % NY;
            // let px_100 = px; //(px + vx * 100 + NX * 100) % NX;
            // let py_100 = py; //(py + vy * 100 + NY * 100) % NY;

            *map.entry((px_100, py_100)).or_default() += 1;
        }

        let mut found = false;

        for y in 0..NY {
            for x in 0..NX {
                if map.contains_key(&(x, y))
                    && (map.contains_key(&(x + 1, y)))
                    && (map.contains_key(&(x + 2, y)))
                    && (map.contains_key(&(x + 3, y)))
                    && (map.contains_key(&(x + 4, y)))
                    && (map.contains_key(&(x + 5, y)))
                    && (map.contains_key(&(x + 6, y)))
                {
                    found = true;
                }
            }
        }

        if !found {
            continue;
        }

        println!("i: {}", i);

        /*        for caps in re.captures_iter(file) {
                    let (_, [px, py, vx, vy]) = caps.extract::<4>();
                    // .1
                    // .iter()
                    // .filter_map(|s| s.parse::<i32>().ok())
                    // .collect_tuple()
                    // .unwrap();
                    // dbg!(px, py, vx, vy);

                    let px = px.parse::<i32>().unwrap();
                    let py = py.parse::<i32>().unwrap();
                    let vx = vx.parse::<i32>().unwrap();
                    let vy = vy.parse::<i32>().unwrap();

                    let px_100 = (px + vx * i + NX * 100) % NX;
                    let py_100 = (py + vy * i + NY * 100) % NY;
                    // let px_100 = px; //(px + vx * 100 + NX * 100) % NX;
                    // let py_100 = py; //(py + vy * 100 + NY * 100) % NY;

                    *map.entry((px_100, py_100)).or_default() += 1;
                }
                map
                   .iter()
                   .filter_map(|(k, v)| {
                       if k.1 == NY / 2 || k.0 == NX / 2 {
                           None
                       } else {
                           Some(v)
                       }
                   })
                   .sum::<i32>()
        */
        let mut quadrants = [0; 4];

        for y in 0..NY {
            for x in 0..NX {
                // if y == NY / 2 || x == NX / 2 {
                //     // print!(" ");
                //     continue;
                // }
                //
                // let mut quadrant_idx = 0;
                // quadrant_idx += (y > NY / 2) as usize;
                // quadrant_idx *= 2;
                // quadrant_idx += (x > NX / 2) as usize;
                //
                // quadrants[quadrant_idx] += map.get(&(x, y)).unwrap_or(&0);

                if let Some(n) = map.get(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    println!();
    println!("---");
    println!();
    // dbg!(&quadrants);

    // quadrants.iter().product()
    0
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 14 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 230172768);
    // assert_eq!(part2, 0);
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
