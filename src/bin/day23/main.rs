#![feature(test)]

use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

fn parse() -> HashMap<String, HashSet<String>> {
    let mut graph = HashMap::<String, HashSet<String>>::new();

    for line in include_str!("input.txt").lines() {
        let (lhs, rhs) = line.split_once("-").unwrap();
        let lhs = lhs.to_string();
        let rhs = rhs.to_string();

        graph.entry(lhs.clone()).or_default().insert(rhs.clone());
        graph.entry(rhs).or_default().insert(lhs);
    }

    graph
}

fn part1() -> usize {
    let graph = parse();

    let mut paths = HashSet::new();

    for curr in graph.keys().filter(|node| node.starts_with("t")) {
        let mut queue = VecDeque::from([[Some(curr), None, None]]);

        while let Some(possible_path) = queue.pop_back() {
            let p = possible_path.partition_point(|node| node.is_some());

            if p == possible_path.len() {
                if graph
                    .get(possible_path[possible_path.len() - 1].unwrap())
                    .is_some_and(|set| set.contains(curr))
                {
                    paths.insert(
                        possible_path
                            .iter()
                            .copied()
                            .flatten()
                            .sorted_unstable()
                            .collect_vec(),
                    );
                }

                continue;
            }

            let neighbours = graph.get(possible_path[p - 1].unwrap()).unwrap();

            for node in neighbours {
                if !possible_path.contains(&Some(node)) {
                    let mut possible_path = possible_path;
                    possible_path[p] = Some(node);

                    queue.push_back(possible_path);
                }
            }
        }
    }

    paths.len()
}

fn part2() -> String {
    let graph = parse();

    let mut longest_path = String::new();
    let mut visited = HashSet::new();

    for curr in graph.keys() {
        let mut queue =
            VecDeque::<(HashSet<String>, String)>::from([(HashSet::new(), curr.clone())]);

        while let Some((mut possible_path, new_node)) = queue.pop_back() {
            let neighbours = graph.get(&new_node).unwrap();

            if !neighbours.is_superset(&possible_path) {
                continue;
            }

            if !possible_path.insert(new_node.clone()) {
                continue;
            }

            let s = possible_path.iter().sorted_unstable().join(",");

            if !visited.insert(s.clone()) {
                continue;
            }

            if s.len() > longest_path.len() {
                longest_path = s;
            }

            queue.extend(
                graph
                    .get(&new_node)
                    .unwrap()
                    .iter()
                    .map(|node| (possible_path.clone(), node.clone())),
            )
        }
    }

    longest_path
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 23 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 926);
    assert_eq!(part2, "az,ed,hz,it,ld,nh,pc,td,ty,ux,wc,yg,zz");
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
