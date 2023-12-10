use crate::aoc_common::lib::line_iterator;

#[derive(Debug, Default, Eq, PartialEq)]
struct PipeSection {
    chr: char,
    north: bool,
    east: bool,
    south: bool,
    west: bool,
    start: bool,
    visited: bool,
}

impl PipeSection {
    pub fn new(chr: char) -> Self {
        match chr {
            '|' => PipeSection { chr, north: true, south: true, ..Default::default() },
            '-' => PipeSection { chr, east:  true, west:  true, ..Default::default() },
            'L' => PipeSection { chr, north: true, east:  true, ..Default::default() },
            'J' => PipeSection { chr, north: true, west:  true, ..Default::default() },
            '7' => PipeSection { chr, south: true, west:  true, ..Default::default() },
            'F' => PipeSection { chr, east:  true, south: true, ..Default::default() },
            'S' => PipeSection { chr, start: true, ..Default::default() },
            _   => PipeSection { chr, ..Default::default() },
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Position { x: usize, y: usize }

#[derive(Debug, Eq, PartialEq)]
enum Direction { NORTH, EAST, SOUTH, WEST }

#[cfg(test)]
mod pipe_section_constructor_tests {
    use super::*;

    #[test]
    fn test_north_south() {
        assert_eq!(PipeSection::new('|'),
                   PipeSection { chr: '|', north: true, east: false, south: true, west: false, start: false, visited: false });
    }

    #[test]
    fn test_east_west() {
        assert_eq!(PipeSection::new('-'),
                   PipeSection { chr: '-', north: false, east: true, south: false, west: true, start: false, visited: false });
    }

    #[test]
    fn test_north_east() {
        assert_eq!(PipeSection::new('L'),
                   PipeSection { chr: 'L', north: true, east: true, south: false, west: false, start: false, visited: false });
    }

    #[test]
    fn test_north_west() {
        assert_eq!(PipeSection::new('J'),
                   PipeSection { chr: 'J', north: true, east: false, south: false, west: true, start: false, visited: false });
    }

    #[test]
    fn test_south_west() {
        assert_eq!(PipeSection::new('7'),
                   PipeSection { chr: '7', north: false, east: false, south: true, west: true, start: false, visited: false });
    }

    #[test]
    fn test_south_east() {
        assert_eq!(PipeSection::new('F'),
                   PipeSection { chr: 'F', north: false, east: true, south: true, west: false, start: false, visited: false });
    }

    #[test]
    fn test_start() {
        assert_eq!(PipeSection::new('S'),
                   PipeSection { chr: 'S', north: false, east: false, south: false, west: false, start: true, visited: false });
    }

    #[test]
    fn test_ground() {
        assert_eq!(PipeSection::new('.'),
                   PipeSection { chr: '.', north: false, east: false, south: false, west: false, start: false, visited: false });
    }
}

fn get_pipe_sections_for_row(line: &str) -> Vec<PipeSection> {
    return line.chars().map(PipeSection::new).collect();
}

#[cfg(test)]
mod get_pipe_sections_for_row_tests {
    use super::*;

    #[test]
    fn test_get_pipe_sections_for_empty_row() {
        assert_eq!(get_pipe_sections_for_row(""), vec![]);
    }

    #[test]
    fn test_get_pipe_sections_for_row() {
        assert_eq!(get_pipe_sections_for_row(".L-SJ|F7."), vec![
           PipeSection::new('.'),
           PipeSection::new('L'),
           PipeSection::new('-'),
           PipeSection::new('S'),
           PipeSection::new('J'),
           PipeSection::new('|'),
           PipeSection::new('F'),
           PipeSection::new('7'),
           PipeSection::new('.'),
        ]);
    }
}

fn get_map(filename: &str) -> Vec<Vec<PipeSection>> {
    return line_iterator(filename).map(|a| get_pipe_sections_for_row(a.as_str())).collect();
}

#[cfg(test)]
mod get_map_tests {
    use super::*;

    #[test]
    fn test_get_map_test1() {
        assert_eq!(get_map("data/day10/test1.txt"), vec![
            get_pipe_sections_for_row("....."),
            get_pipe_sections_for_row(".S-7."),
            get_pipe_sections_for_row(".|.|."),
            get_pipe_sections_for_row(".L-J."),
            get_pipe_sections_for_row("....."),
        ]);
    }
}

fn find_start(map: &Vec<Vec<PipeSection>>) -> Option<Position> {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x].start {
                return Some(Position { x, y });
            }
        }
    }
    return None;
}

#[cfg(test)]
mod find_start_tests {
    use super::*;

    #[test]
    fn test_find_start_none() {
        assert_eq!(find_start(&vec![]), None);
    }

    #[test]
    fn test_find_start_test1() {
        assert_eq!(find_start(&get_map("data/day10/test1.txt")), Some(Position { x: 1, y: 1 }));
    }

    #[test]
    fn test_find_start_test3() {
        assert_eq!(find_start(&get_map("data/day10/test3.txt")), Some(Position { x: 0, y: 2 }));
    }
}

fn move_dir(from: &Position, dir: &Direction) -> Position {
    match dir {
        Direction::NORTH => Position { x: from.x, y: from.y - 1 },
        Direction::EAST  => Position { x: from.x + 1, y: from.y },
        Direction::SOUTH => Position { x: from.x, y: from.y + 1 },
        Direction::WEST  => Position { x: from.x - 1, y: from.y },
    }
}

#[cfg(test)]
mod move_dir_tests {
    use super::*;

    #[test]
    fn test_move_north() {
        assert_eq!(move_dir(&Position { x: 4, y: 7 }, &Direction::NORTH), Position { x: 4, y: 6 });
    }

    #[test]
    fn test_move_east() {
        assert_eq!(move_dir(&Position { x: 4, y: 7 }, &Direction::EAST), Position { x: 5, y: 7 });
    }

    #[test]
    fn test_move_south() {
        assert_eq!(move_dir(&Position { x: 4, y: 7 }, &Direction::SOUTH), Position { x: 4, y: 8 });
    }

    #[test]
    fn test_move_west() {
        assert_eq!(move_dir(&Position { x: 4, y: 7 }, &Direction::WEST), Position { x: 3, y: 7 });
    }
}

fn can_move(map: &Vec<Vec<PipeSection>>, from: &Position, dir: &Direction) -> bool {
    let next_position = move_dir(from, dir);
    if next_position.y >= map.len() || next_position.x >= map[next_position.y].len() {
        return false;
    }
    let next_pipe_section = &map[next_position.y][next_position.x];
    if next_pipe_section.start {
        return true;
    }
    match dir {
        Direction::NORTH => next_pipe_section.south,
        Direction::EAST => next_pipe_section.west,
        Direction::SOUTH => next_pipe_section.north,
        Direction::WEST => next_pipe_section.east,
    }
}

#[cfg(test)]
mod can_move_tests {
    use super::*;

    #[test]
    fn test_can_move_north_from_start() {
        assert_eq!(can_move(&get_map("data/day10/test1.txt"), &Position { x: 1, y: 1 }, &Direction::NORTH), false);
    }

    #[test]
    fn test_can_move_east_from_start() {
        assert_eq!(can_move(&get_map("data/day10/test1.txt"), &Position { x: 1, y: 1 }, &Direction::EAST), true);
    }

    #[test]
    fn test_can_move_south_from_start() {
        assert_eq!(can_move(&get_map("data/day10/test1.txt"), &Position { x: 1, y: 1 }, &Direction::SOUTH), true);
    }

    #[test]
    fn test_can_move_west_from_start() {
        assert_eq!(can_move(&get_map("data/day10/test1.txt"), &Position { x: 1, y: 1 }, &Direction::WEST), false);
    }

    #[test]
    fn test_can_move_west_to_start() {
        assert_eq!(can_move(&get_map("data/day10/test1.txt"), &Position { x: 2, y: 1 }, &Direction::WEST), true);
    }

    #[test]
    fn test_can_move_west_from_top_right_to_pipe() {
        assert_eq!(can_move(&get_map("data/day10/test1.txt"), &Position { x: 3, y: 1 }, &Direction::WEST), true);
    }

    #[test]
    fn test_can_move_south_from_top_right_to_pipe() {
        assert_eq!(can_move(&get_map("data/day10/test1.txt"), &Position { x: 3, y: 1 }, &Direction::SOUTH), true);
    }
}

fn next_dir(pipe_section: &PipeSection, from_dir: &Direction) -> Option<Direction> {
    if *from_dir != Direction::SOUTH && pipe_section.north {
        return Some(Direction::NORTH);
    }
    if *from_dir != Direction::WEST && pipe_section.east {
        return Some(Direction::EAST);
    }
    if *from_dir != Direction::NORTH && pipe_section.south {
        return Some(Direction::SOUTH);
    }
    if *from_dir != Direction::EAST && pipe_section.west {
        return Some(Direction::WEST);
    }
    return None;
}

#[cfg(test)]
mod next_dir_tests {
    use super::*;

    #[test]
    fn test_next_dir_south_west_pipe_from_south() {
        assert_eq!(next_dir(&PipeSection::new('7'), &Direction::NORTH), Some(Direction::WEST));
    }

    #[test]
    fn test_next_dir_south_west_pipe_from_west() {
        assert_eq!(next_dir(&PipeSection::new('7'), &Direction::EAST), Some(Direction::SOUTH));
    }

    #[test]
    fn test_next_dir_start() {
        assert_eq!(next_dir(&PipeSection::new('S'), &Direction::WEST), None);
    }
}

fn count_loop_steps(map: &Vec<Vec<PipeSection>>, start: &Position, dir: Direction) -> Option<usize> {
    if !can_move(&map, &start, &dir) {
        return None;
    }
    let mut current_position = move_dir(&start, &dir);
    let mut current_from_dir = dir;
    let mut current_section = &map[current_position.y][current_position.x];
    let mut steps = 1;
    while !current_section.start {
        let next_dir = next_dir(current_section, &current_from_dir);
        if next_dir.is_none() {
            return None;
        }
        current_from_dir = next_dir.unwrap();
        if !can_move(&map, &current_position, &current_from_dir) {
            return None;
        }
        current_position = move_dir(&current_position, &current_from_dir);
        current_section = &map[current_position.y][current_position.x];
        steps = steps + 1;
    }
    return Some(steps);
}

#[cfg(test)]
mod count_loop_steps_tests {
    use super::*;

    #[test]
    fn test_count_loop_steps_north_from_start() {
        let map = get_map("data/day10/test1.txt");
        let start = find_start(&map).unwrap();
        assert_eq!(count_loop_steps(&map, &start, Direction::NORTH), None);
    }

    #[test]
    fn test_count_loop_steps_east_from_start() {
        let map = get_map("data/day10/test1.txt");
        let start = find_start(&map).unwrap();
        assert_eq!(count_loop_steps(&map, &start, Direction::EAST), Some(8));
    }

    #[test]
    fn test_count_loop_steps_south_from_start() {
        let map = get_map("data/day10/test1.txt");
        let start = find_start(&map).unwrap();
        assert_eq!(count_loop_steps(&map, &start, Direction::SOUTH), Some(8));
    }

    #[test]
    fn test_count_loop_steps_west_from_start() {
        let map = get_map("data/day10/test1.txt");
        let start = find_start(&map).unwrap();
        assert_eq!(count_loop_steps(&map, &start, Direction::WEST), None);
    }
}

fn solve_for_part1(filename: &str) -> usize {
    let map = get_map(filename);
    let start = find_start(&map).unwrap();
    let directions = [Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST];
    for dir in directions {
        if can_move(&map, &start, &dir) {
            let total_steps = count_loop_steps(&map, &start, dir);
            if total_steps.is_some() {
                let total = total_steps.unwrap();
                return total / 2;
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod solve_for_part1_tests {
    use super::*;

    #[test]
    fn test_solve_for_part1_test1() {
        assert_eq!(solve_for_part1("data/day10/test1.txt"), 4);
    }

    #[test]
    fn test_solve_for_part1_test2() {
        assert_eq!(solve_for_part1("data/day10/test2.txt"), 4);
    }

    #[test]
    fn test_solve_for_part1_test3() {
        assert_eq!(solve_for_part1("data/day10/test3.txt"), 8);
    }

    #[test]
    fn test_solve_for_part1_test4() {
        assert_eq!(solve_for_part1("data/day10/test4.txt"), 8);
    }
}

fn solve_for_part2(_filename: &str) -> i64 {
    return 0;
}

pub fn part1() {
    println!("Day 10 Part 1 result: {}", solve_for_part1("data/day10/input.txt"));
}

pub fn part2() {
    println!("Day 10 Part 2 result: {}", solve_for_part2("data/day10/input.txt"));
}
