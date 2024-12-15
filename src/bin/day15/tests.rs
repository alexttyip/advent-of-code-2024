#[cfg(test)]
mod unit_tests {
    extern crate test;

    use crate::{
        get_boxes_to_shift_vertically, parse_grid_and_robot, part1, part2, shift, CellType,
        Direction, N,
    };
    use std::collections::HashSet;
    use test::Bencher;

    #[test]
    fn test_shift_right() {
        let (mut grid, mut robot) = parse_grid_and_robot("#.@[][].#".to_owned());

        let dir = Direction::Right;
        let box_left_pos = 3;
        shift(&mut grid, &mut robot, dir, box_left_pos);

        // #..@[][]#
        assert_eq!(
            [
                CellType::Wall,
                CellType::Space,
                CellType::Space,
                CellType::Space,
                CellType::Box,
                CellType::BoxRight,
                CellType::Box,
                CellType::BoxRight,
                CellType::Wall
            ],
            grid[0..9],
        );
        assert_eq!(3, robot);
    }

    #[test]
    fn test_shift_left() {
        let (mut grid, mut robot) = parse_grid_and_robot("#.[][]@.#".to_owned());
        let dir = Direction::Left;
        let box_left_pos = 4;
        shift(&mut grid, &mut robot, dir, box_left_pos);

        // #[][]@..#
        assert_eq!(
            [
                CellType::Wall,
                CellType::Box,
                CellType::BoxRight,
                CellType::Box,
                CellType::BoxRight,
                CellType::Space,
                CellType::Space,
                CellType::Space,
                CellType::Wall
            ],
            grid[0..9],
        );
        assert_eq!(5, robot);
    }

    #[test]
    fn test_getting_down_shiftable_boxes() {
        let (mut grid, _) = parse_grid_and_robot(
            "\
            #######\n\
            #.....#\n\
            #..[].#\n\
            #.[][]#\n\
            #.....#\n\
            #.....#\n\
            #######\n\
            "
            .to_owned(),
        );
        let box_left_pos = 2 * N + 3;
        let boxes_to_move = get_boxes_to_shift_vertically(&mut grid, box_left_pos, Direction::Down);

        assert_eq!(
            HashSet::from([2 * N + 3, 3 * N + 2, 3 * N + 4]),
            boxes_to_move
        );
    }

    #[test]
    fn test_getting_down_shiftable_boxes2() {
        let (mut grid, _) = parse_grid_and_robot(
            "\
            #######\n\
            #.....#\n\
            #..[].#\n\
            #.[][]#\n\
            #..[].#\n\
            #.....#\n\
            #######\n\
            "
            .to_owned(),
        );
        let box_left_pos = 2 * N + 3;
        let boxes_to_move = get_boxes_to_shift_vertically(&mut grid, box_left_pos, Direction::Down);

        assert_eq!(
            HashSet::from([2 * N + 3, 3 * N + 2, 3 * N + 4, 4 * N + 3]),
            boxes_to_move
        );
    }

    #[test]
    fn test_getting_down_unshiftable_boxes() {
        let (mut grid, _) = parse_grid_and_robot(
            "\
            #######\n\
            #.....#\n\
            #..[].#\n\
            #.[][]#\n\
            #...#.#\n\
            #.....#\n\
            #######\n\
            "
            .to_owned(),
        );
        let box_left_pos = 2 * N + 3;
        let boxes_to_move = get_boxes_to_shift_vertically(&mut grid, box_left_pos, Direction::Down);

        assert!(boxes_to_move.is_empty());
    }

    #[test]
    fn test_shift_down() {
        let (mut grid, mut robot) = parse_grid_and_robot(
            "\
            #######\n\
            #..@..#\n\
            #..[].#\n\
            #.[][]#\n\
            #..[].#\n\
            #.....#\n\
            #######\n\
            "
            .to_owned(),
        );
        let dir = Direction::Down;
        let box_left_pos = 2 * N + 3;

        // When
        shift(&mut grid, &mut robot, dir, box_left_pos);

        // Then
        assert_eq!(
            "\
            #######\n\
            #.....#\n\
            #..@..#\n\
            #..[].#\n\
            #.[][]#\n\
            #..[].#\n\
            #######\n\
            "
            .to_owned(),
            grid_to_print(&grid, robot, 7)
        );
    }

    #[test]
    fn test_shift_down2() {
        let (mut grid, mut robot) = parse_grid_and_robot(
            "\
            #######\n\
            #...@.#\n\
            #..[].#\n\
            #.[][]#\n\
            #..[].#\n\
            #.....#\n\
            #######\n\
            "
            .to_owned(),
        );
        let dir = Direction::Down;
        let box_left_pos = 2 * N + 3;

        // When
        shift(&mut grid, &mut robot, dir, box_left_pos);

        // Then
        assert_eq!(
            "\
            #######\n\
            #.....#\n\
            #...@.#\n\
            #..[].#\n\
            #.[][]#\n\
            #..[].#\n\
            #######\n\
            "
            .to_owned(),
            grid_to_print(&grid, robot, 7)
        );
    }
    #[test]
    fn test_shift_down_noop() {
        let (mut grid, mut robot) = parse_grid_and_robot(
            "\
            #######\n\
            #..@..#\n\
            #..[].#\n\
            #.[][]#\n\
            #...#.#\n\
            #.....#\n\
            #######\n\
            "
            .to_owned(),
        );
        let dir = Direction::Down;
        let box_left_pos = 2 * N + 3;

        // When
        shift(&mut grid, &mut robot, dir, box_left_pos);

        // Then
        assert_eq!(
            "\
            #######\n\
            #..@..#\n\
            #..[].#\n\
            #.[][]#\n\
            #...#.#\n\
            #.....#\n\
            #######\n\
            "
            .to_owned(),
            grid_to_print(&grid, robot, 7)
        );
    }

    // --- UP ---

    #[test]
    fn test_getting_up_shiftable_boxes() {
        let (mut grid, _) = parse_grid_and_robot(
            "\
            #######\n\
            #.....#\n\
            #..[].#\n\
            #.[][]#\n\
            #.....#\n\
            #.....#\n\
            #######\n\
            "
            .to_owned(),
        );
        let box_left_pos = 3 * N + 2;
        let boxes_to_move = get_boxes_to_shift_vertically(&mut grid, box_left_pos, Direction::Up);

        assert_eq!(HashSet::from([2 * N + 3, 3 * N + 2,]), boxes_to_move);
    }

    #[test]
    fn test_getting_up_shiftable_boxes2() {
        let (mut grid, _) = parse_grid_and_robot(
            "\
            #######\n\
            #.....#\n\
            #..[].#\n\
            #.[][]#\n\
            #..[].#\n\
            #.....#\n\
            #######\n\
            "
            .to_owned(),
        );
        let box_left_pos = 4 * N + 3;
        let boxes_to_move = get_boxes_to_shift_vertically(&mut grid, box_left_pos, Direction::Up);

        assert_eq!(
            HashSet::from([2 * N + 3, 3 * N + 2, 3 * N + 4, 4 * N + 3]),
            boxes_to_move
        );
    }

    #[test]
    fn test_getting_up_unshiftable_boxes() {
        let (mut grid, _) = parse_grid_and_robot(
            "\
            #######\n\
            #..#..#\n\
            #..[].#\n\
            #.[][]#\n\
            #.....#\n\
            #.....#\n\
            #######\n\
            "
            .to_owned(),
        );
        let box_left_pos = 3 * N + 2;
        let boxes_to_move = get_boxes_to_shift_vertically(&mut grid, box_left_pos, Direction::Up);

        assert!(boxes_to_move.is_empty());
    }

    #[test]
    fn test_shift_up() {
        let (mut grid, mut robot) = parse_grid_and_robot(
            "\
            #######\n\
            #.....#\n\
            #..[].#\n\
            #.[][]#\n\
            #..[].#\n\
            #..@..#\n\
            #######\n\
            "
            .to_owned(),
        );
        let dir = Direction::Up;
        let box_left_pos = 5 * N + 3;

        // When
        shift(&mut grid, &mut robot, dir, box_left_pos);

        // Then
        assert_eq!(
            "\
            #######\n\
            #..[].#\n\
            #.[][]#\n\
            #..[].#\n\
            #..@..#\n\
            #.....#\n\
            #######\n\
            "
            .to_owned(),
            grid_to_print(&grid, robot, 7)
        );
    }

    #[test]
    fn test_shift_up2() {
        let (mut grid, mut robot) = parse_grid_and_robot(
            "\
            #######\n\
            #.....#\n\
            #..[].#\n\
            #.[][]#\n\
            #..[].#\n\
            #...@.#\n\
            #######\n\
            "
            .to_owned(),
        );
        let dir = Direction::Up;
        let box_left_pos = 5 * N + 4;

        // When
        shift(&mut grid, &mut robot, dir, box_left_pos);

        // Then
        assert_eq!(
            "\
            #######\n\
            #..[].#\n\
            #.[][]#\n\
            #..[].#\n\
            #...@.#\n\
            #.....#\n\
            #######\n\
            "
            .to_owned(),
            grid_to_print(&grid, robot, 7)
        );
    }
    #[test]
    fn test_shift_up_noop() {
        let (mut grid, mut robot) = parse_grid_and_robot(
            "\
            #######\n\
            #...#.#\n\
            #..[].#\n\
            #.[][]#\n\
            #..@..#\n\
            #.....#\n\
            #######\n\
            "
            .to_owned(),
        );
        let dir = Direction::Up;
        let box_left_pos = 4 * N + 3;

        // When
        shift(&mut grid, &mut robot, dir, box_left_pos);

        // Then
        assert_eq!(
            "\
            #######\n\
            #...#.#\n\
            #..[].#\n\
            #.[][]#\n\
            #..@..#\n\
            #.....#\n\
            #######\n\
            "
            .to_owned(),
            grid_to_print(&grid, robot, 7)
        );
    }

    fn grid_to_print(grid: &[CellType; N * N], robot: usize, n: usize) -> String {
        let mut output = String::new();

        for x in 0..n {
            for y in 0..n {
                let pos = x * N + y;

                if pos == robot {
                    output += "@";
                    continue;
                }

                output += match grid[pos] {
                    CellType::Box => "[",
                    CellType::Wall => "#",
                    CellType::BoxRight => "]",
                    CellType::Space => ".",
                }
            }

            output += "\n";
        }

        output
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
