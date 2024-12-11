#![feature(test)]

use itertools::Itertools;
use std::cmp::Ordering;
use std::time::Instant;

#[derive(Copy, Clone, Debug)]
struct Block {
    start: usize,
    length: usize,
    id: usize,
}

impl Block {
    const fn new(start: usize, length: usize, id: usize) -> Self {
        Block { start, length, id }
    }

    fn checksum(&self) -> usize {
        self.id * (self.start * 2 + self.length - 1) * self.length / 2
    }
}

impl Eq for Block {}

impl PartialEq<Self> for Block {
    fn eq(&self, other: &Self) -> bool {
        (self.start, self.length, self.id) == (other.start, other.length, other.id)
    }
}

impl PartialOrd<Self> for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.start, self.length, self.id).cmp(&(other.start, other.length, other.id))
    }
}

struct Gap {
    start: usize,
    length: usize,
}

impl Gap {
    const fn new(start: usize, length: usize) -> Self {
        Gap { start, length }
    }
}

fn parse() -> (Vec<Block>, Vec<Gap>) {
    let numbers = include_bytes!("input.txt")
        .iter()
        .filter_map(|b| b.checked_sub(b'0').map(|i| i as usize));

    let mut blocks = vec![];
    let mut gaps = vec![];
    let mut pointer = 0;

    for (i, length) in numbers.enumerate() {
        if i % 2 == 0 {
            blocks.push(Block::new(pointer, length, i / 2));
        } else {
            gaps.push(Gap::new(pointer, length));
        }

        pointer += length;
    }

    gaps.reverse();

    (blocks, gaps)
}

fn calculate_checksum(blocks: &[Block], moved_blocks: &[Block]) -> usize {
    blocks
        .iter()
        .chain(moved_blocks.iter())
        .sorted_unstable()
        .map(Block::checksum)
        .sum::<usize>()
}

fn part1() -> usize {
    let (mut blocks, mut gaps) = parse();

    let mut moved_blocks = vec![];

    while let Some(mut gap) = gaps.pop() {
        let block = blocks.last_mut().unwrap();

        if gap.start >= block.start {
            break;
        }

        if block.length > gap.length {
            moved_blocks.push(Block::new(gap.start, gap.length, block.id));
            block.length -= gap.length;
            continue;
        }

        moved_blocks.push(Block::new(gap.start, block.length, block.id));

        if block.length < gap.length {
            gap.start += block.length;
            gap.length -= block.length;

            gaps.push(gap);
        }

        blocks.pop();
    }

    calculate_checksum(&blocks, &moved_blocks)
}

fn part2() -> usize {
    let (mut blocks, mut gaps) = parse();

    let mut moved_blocks = vec![];

    while let Some(mut gap) = gaps.pop() {
        let mut i = blocks.len() - 1;

        let mut block = blocks.get_mut(i).unwrap();

        while gap.start < block.start {
            if block.length <= gap.length {
                break;
            }

            i -= 1;
            block = blocks.get_mut(i).unwrap();
        }

        if gap.start < block.start {
            block.start = gap.start;
            moved_blocks.push(*block);

            if block.length < gap.length {
                gap.start += block.length;
                gap.length -= block.length;

                gaps.push(gap);
            }

            blocks.remove(i);
        }
    }

    calculate_checksum(&blocks, &moved_blocks)
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 09 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 6334655979668);
    assert_eq!(part2, 6349492251099);
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
