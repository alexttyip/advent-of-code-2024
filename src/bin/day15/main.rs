#![feature(test)]

mod tests;

use itertools::Itertools;
use num::Integer;
use std::cmp::PartialEq;
use std::collections::HashSet;
use std::time::Instant;

const N: usize = 100;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum CellType {
    Wall,
    Box,
    BoxRight,
    Space,
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const fn new(c: char) -> Self {
        match c {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => panic!(),
        }
    }
}

fn updated_coordinate(pos: usize, dir: Direction) -> usize {
    match dir {
        Direction::Up => pos - N,
        Direction::Right => pos + 1,
        Direction::Down => pos + N,
        Direction::Left => pos - 1,
    }
}

fn parse(file: &str) -> ([CellType; N * N], String, usize) {
    let (grid_str, moves) = file.split_once("\n\n").unwrap();

    let (grid, robot) = parse_grid_and_robot(grid_str.to_owned());

    (grid, moves.to_owned(), robot)
}

fn parse_grid_and_robot(grid_str: String) -> ([CellType; N * N], usize) {
    let mut robot = 0;
    let mut grid = [CellType::Space; N * N];

    for (x, line) in grid_str.lines().enumerate() {
        for (y, mut c) in line.chars().enumerate() {
            if c == '@' {
                robot = x * N + y;
                c = '.';
            }

            grid[x * N + y] = match c {
                '#' => CellType::Wall,
                'O' | '[' => CellType::Box,
                ']' => CellType::BoxRight,
                _ => CellType::Space,
            };
        }
    }

    (grid, robot)
}

fn calculate_gps_sum(grid: &[CellType; N * N]) -> usize {
    grid.iter()
        .enumerate()
        .filter_map(|(i, cell)| {
            let (x, y) = i.div_rem(&N);

            (cell == &CellType::Box).then_some(100 * x + y)
        })
        .sum::<usize>()
}

fn part1() -> usize {
    let file = include_str!("input.txt");
    let (mut grid, moves, mut robot) = parse(file);

    for m in moves.chars() {
        if m == '\n' {
            continue;
        }

        let dir = Direction::new(m);
        let neighbour = updated_coordinate(robot, dir);

        match grid[neighbour] {
            CellType::Box => {
                let mut first_space = updated_coordinate(neighbour, dir);

                while grid[first_space] == CellType::Box {
                    first_space = updated_coordinate(first_space, dir);
                }

                if grid[first_space] == CellType::Space {
                    grid.swap(neighbour, first_space);
                    robot = neighbour;
                }
            }
            CellType::Space => {
                robot = neighbour;
            }
            _ => {}
        }
    }

    calculate_gps_sum(&grid)
}

fn get_boxes_to_shift_vertically(
    grid: &mut [CellType; N * N],
    box_left_pos: usize,
    dir: Direction,
) -> HashSet<usize> {
    let neighbour = updated_coordinate(box_left_pos, dir);

    let mut box_left_set = match grid[neighbour] {
        CellType::Box => get_boxes_to_shift_vertically(grid, neighbour, dir),
        CellType::BoxRight => get_boxes_to_shift_vertically(grid, neighbour - 1, dir),
        CellType::Wall => HashSet::new(),
        CellType::Space => HashSet::from([box_left_pos]),
    };

    if box_left_set.is_empty() {
        return box_left_set;
    }

    box_left_set.insert(box_left_pos);

    let box_right_set = match grid[neighbour + 1] {
        CellType::Box => get_boxes_to_shift_vertically(grid, neighbour + 1, dir),
        CellType::BoxRight => get_boxes_to_shift_vertically(grid, neighbour, dir),
        CellType::Wall => HashSet::new(),
        CellType::Space => HashSet::from([box_left_pos]),
    };

    if box_right_set.is_empty() {
        return box_right_set;
    }

    box_left_set.extend(box_right_set);

    box_left_set
}

fn shift(grid: &mut [CellType; N * N], robot: &mut usize, dir: Direction, box_left_pos: usize) {
    if dir == Direction::Right {
        let mut next_space = box_left_pos + 1;
        while grid[next_space] == CellType::Box || grid[next_space] == CellType::BoxRight {
            next_space += 1;
        }

        if grid[next_space] == CellType::Space {
            for pos in (box_left_pos..next_space).rev() {
                grid.swap(pos, pos + 1);
            }

            *robot += 1;
        }

        return;
    }

    if dir == Direction::Left {
        let mut next_space = box_left_pos - 1;
        while grid[next_space] == CellType::Box || grid[next_space] == CellType::BoxRight {
            next_space -= 1;
        }

        if grid[next_space] == CellType::Space {
            for pos in next_space..(box_left_pos + 1) {
                grid.swap(pos, pos + 1);
            }

            *robot -= 1;
        }

        return;
    }

    // vertical
    let boxes_to_shift = get_boxes_to_shift_vertically(grid, box_left_pos, dir);

    if dir == Direction::Up {
        for &pos in boxes_to_shift.iter().sorted_unstable() {
            grid.swap(pos, pos - N);
            grid.swap(pos + 1, pos - N + 1);
        }
    } else {
        for &pos in boxes_to_shift.iter().sorted_unstable().rev() {
            grid.swap(pos, pos + N);
            grid.swap(pos + 1, pos + N + 1);
        }
    }

    if !boxes_to_shift.is_empty() {
        *robot = updated_coordinate(*robot, dir);
    }
}

fn part2() -> usize {
    let file = include_str!("input2.txt");
    let (mut grid, moves, mut robot) = parse(file);

    for m in moves.chars() {
        if m == '\n' {
            continue;
        }

        let dir = Direction::new(m);
        let neighbour = updated_coordinate(robot, dir);

        match grid[neighbour] {
            CellType::Box => shift(&mut grid, &mut robot, dir, neighbour),
            CellType::BoxRight => shift(&mut grid, &mut robot, dir, neighbour - 1),
            CellType::Space => robot = neighbour,
            _ => {}
        }
    }

    calculate_gps_sum(&grid)
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 15 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 1398947);
    assert_eq!(part2, 1397393);
}
