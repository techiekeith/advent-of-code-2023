use crate::aoc_common::lib::{get_series_of_ints, line_iterator};

fn calculate_distance(len: i64, hold: i64) -> i64 {
    return hold * (len - hold);
}

#[cfg(test)]
mod calculate_distance_tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        assert_eq!(calculate_distance(7, 0), 0);
        assert_eq!(calculate_distance(7, 1), 6);
        assert_eq!(calculate_distance(7, 2), 10);
        assert_eq!(calculate_distance(7, 3), 12);
        assert_eq!(calculate_distance(7, 4), 12);
        assert_eq!(calculate_distance(7, 5), 10);
        assert_eq!(calculate_distance(7, 6), 6);
        assert_eq!(calculate_distance(7, 7), 0);
    }
}

fn smallest_time(time: i64, distance: i64) -> i64 {
    let mut index = 1;
    while index <= time / 2 {
        if calculate_distance(time, index) > distance {
            return index;
        }
        index = index + 1;
    }
    return 0;
}

#[cfg(test)]
mod smallest_time_tests {
    use super::*;

    #[test]
    fn test_smallest_time() {
        assert_eq!(smallest_time(7, 9), 2);
        assert_eq!(smallest_time(15, 40), 4);
        assert_eq!(smallest_time(30, 200), 11);
    }
}

fn winning_permutations(time: i64, distance: i64) -> i64 {
    let lowest = smallest_time(time, distance);
    return 1 + time - lowest * 2;
}

#[cfg(test)]
mod winning_permutations_tests {
    use super::*;

    #[test]
    fn test_winning_permutations() {
        assert_eq!(winning_permutations(7, 9), 4);
        assert_eq!(winning_permutations(15, 40), 8);
        assert_eq!(winning_permutations(30, 200), 9);
    }
}

fn get_part1_data(filename: &str) -> (Vec<i64>, Vec<i64>) {
    let lines = line_iterator(filename);
    let strings: Vec<String> = lines.collect();
    let times = get_series_of_ints::<i64>(strings[0].as_str());
    let distances = get_series_of_ints::<i64>(strings[1].as_str());
    return (times, distances);
}

#[cfg(test)]
mod get_part1_data_tests {
    use super::*;

    #[test]
    fn test_get_part1_data() {
        let (times, distances) = get_part1_data("data/day06/test.txt");
        assert_eq!(times.len(), 3);
        assert_eq!(distances.len(), 3);
    }
}

fn get_part1_result(filename: &str) -> i64 {
    let (times, distances) = get_part1_data(filename);
    let mut index = 0;
    let mut result = 1;
    while index < times.len() {
        result = result * winning_permutations(times[index], distances[index]);
        index = index + 1;
    }
    return result;
}

#[cfg(test)]
mod get_part1_result_tests {
    use super::*;

    #[test]
    fn test_get_part1_result() {
        assert_eq!(get_part1_result("data/day06/test.txt"), 288);
    }
}

fn get_part2_number(line: &str) -> i64 {
    let mut number = 0;
    for chr in line.chars() {
        if chr >= '0' && chr <= '9' {
            number = number * 10 + chr as i64 - 48;
        }
    }
    return number;
}

#[cfg(test)]
mod get_part2_number_tests {
    use super::*;

    #[test]
    fn test_get_part2_number() {
        assert_eq!(get_part2_number("Time:      7  15   30"), 71530);
        assert_eq!(get_part2_number("Distance:  9  40  200"), 940200);
    }
}

fn get_part2_data(filename: &str) -> (i64, i64) {
    let lines = line_iterator(filename);
    let strings: Vec<String> = lines.collect();
    let time = get_part2_number(strings[0].as_str());
    let distance = get_part2_number(strings[1].as_str());
    return (time, distance);
}

#[cfg(test)]
mod get_part2_data_tests {
    use super::*;

    #[test]
    fn test_get_part2_data() {
        let (time, distance) = get_part2_data("data/day06/test.txt");
        assert_eq!(time, 71530);
        assert_eq!(distance, 940200);
    }
}

fn get_part2_result(filename: &str) -> i64 {
    let (time, distance) = get_part2_data(filename);
    return winning_permutations(time, distance);
}

#[cfg(test)]
mod get_part2_result_tests {
    use super::*;

    #[test]
    fn test_get_part2_result() {
        assert_eq!(get_part2_result("data/day06/test.txt"), 71503);
    }
}

pub fn part1() {
    println!("Day 6 Part 1 result: {}", get_part1_result("data/day06/input.txt"));
}

pub fn part2() {
    println!("Day 6 Part 2 result: {}", get_part2_result("data/day06/input.txt"));
}
