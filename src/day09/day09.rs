use crate::aoc_common::lib::{get_series_of_ints, sum, sum_up};

fn dereference_i64(reference: &i64) -> i64 {
    return *reference;
}

fn is_sequence_all_zeroes(sequence: &Vec<i64>) -> bool {
    return sequence.iter().map(dereference_i64).reduce(sum).unwrap() == 0;
}

#[cfg(test)]
mod is_sequence_all_zeroes_tests {
    use super::*;

    #[test]
    fn test_is_sequence_all_zeroes_nonzero() {
        let sequence: Vec<i64> = vec![1, 2, 3, 4, 5];
        assert_eq!(is_sequence_all_zeroes(&sequence), false);
    }

    #[test]
    fn test_is_sequence_all_zeroes_zero() {
        let sequence: Vec<i64> = vec![0, 0, 0, 0, 0];
        assert_eq!(is_sequence_all_zeroes(&sequence), true);
    }
}

fn get_next_sequence(sequence: &Vec<i64>) -> (Vec<i64>, bool) {
    let mut index: usize = 1;
    let mut next_sequence: Vec<i64> = vec![];
    while index < sequence.len() {
        next_sequence.push(sequence[index] - sequence[index - 1]);
        index = index + 1;
    }
    let done = is_sequence_all_zeroes(&next_sequence);
    return (next_sequence, done);
}

#[cfg(test)]
mod get_next_sequence_tests {
    use super::*;

    #[test]
    fn test_get_next_sequence_flat() {
        let initial_sequence: Vec<i64> = vec![3, 3, 3, 3, 3, 3];
        let expected_next_sequence: Vec<i64> = vec![0, 0, 0, 0, 0];
        assert_eq!(get_next_sequence(&initial_sequence), (expected_next_sequence, true));
    }

    #[test]
    fn test_get_next_sequence_increment_by_three() {
        let initial_sequence: Vec<i64> = vec![0, 3, 6, 9, 12, 15];
        let expected_next_sequence: Vec<i64> = vec![3, 3, 3, 3, 3];
        assert_eq!(get_next_sequence(&initial_sequence), (expected_next_sequence, false));
    }

    #[test]
    fn test_get_next_sequence_increment_increasing() {
        let initial_sequence: Vec<i64> = vec![1, 3, 6, 10, 15, 21];
        let expected_next_sequence: Vec<i64> = vec![2, 3, 4, 5, 6];
        assert_eq!(get_next_sequence(&initial_sequence), (expected_next_sequence, false));
    }

    #[test]
    fn test_get_next_sequence_increment_another_pattern() {
        let initial_sequence: Vec<i64> = vec![10, 13, 16, 21, 30, 45];
        let expected_next_sequence: Vec<i64> = vec![3, 3, 5, 9, 15];
        assert_eq!(get_next_sequence(&initial_sequence), (expected_next_sequence, false));
    }
}

fn get_sequences(line: &str) -> Vec<Vec<i64>> {
    let mut sequences: Vec<Vec<i64>> = vec![];
    sequences.push(get_series_of_ints::<i64>(line));
    let mut done = false;
    while !done {
        let (next_sequence, done2) = get_next_sequence(&sequences.last().unwrap());
        done = done2;
        if !done {
            sequences.push(next_sequence);
        }
    }
    return sequences;
}

#[cfg(test)]
mod get_sequences_tests {
    use super::*;

    #[test]
    fn test_get_sequences() {
        let initial_sequence = "1 3 6 10 15 21";
        let result = get_sequences(initial_sequence);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], vec![1, 3, 6, 10, 15, 21]);
        assert_eq!(result[1], vec![2, 3, 4, 5, 6]);
        assert_eq!(result[2], vec![1, 1, 1, 1]);
    }
}

fn extrapolate_next_value(sequences: Vec<Vec<i64>>) -> i64 {
    let mut index = sequences.len();
    let mut result = 0;
    while index > 0 {
        index = index - 1;
        let last = *sequences[index].last().unwrap();
        result = last + result;
    }
    return result;
}

#[cfg(test)]
mod extrapolate_next_value_tests {
    use super::*;

    #[test]
    fn test_extrapolate_next_value() {
        let sequences: Vec<Vec<i64>> = vec![
            vec![1, 3, 6, 10, 15, 21],
            vec![2, 3, 4, 5, 6],
            vec![1, 1, 1, 1],
        ];
        assert_eq!(extrapolate_next_value(sequences), 28);
    }
}

fn solve_for_part1(filename: &str) -> i64 {
    return sum_up(filename, |a| extrapolate_next_value(get_sequences(a)), sum);
}

#[cfg(test)]
mod solve_for_part1_tests {
    use super::*;

    #[test]
    fn test_solve_for_part1() {
        assert_eq!(solve_for_part1("data/day09/test.txt"), 114);
    }
}

fn extrapolate_previous_value(sequences: Vec<Vec<i64>>) -> i64 {
    let mut index = sequences.len();
    let mut result = 0;
    while index > 0 {
        index = index - 1;
        let last = *sequences[index].first().unwrap();
        result = last - result;
    }
    return result;
}

#[cfg(test)]
mod extrapolate_previous_value_tests {
    use super::*;

    #[test]
    fn test_extrapolate_previous_value() {
        let sequences: Vec<Vec<i64>> = vec![
            vec![10, 13, 16, 21, 30, 45],
            vec![3, 3, 5, 9, 15],
            vec![0, 2, 4, 6],
            vec![2, 2, 2],
        ];
        assert_eq!(extrapolate_previous_value(sequences), 5);
    }
}

fn solve_for_part2(filename: &str) -> i64 {
    return sum_up(filename, |a| extrapolate_previous_value(get_sequences(a)), sum);
}

#[cfg(test)]
mod solve_for_part2_tests {
    use super::*;

    #[test]
    fn test_solve_for_part2() {
        assert_eq!(solve_for_part2("data/day09/test.txt"), 2);
    }
}

pub fn part1() {
    println!("Day 9 Part 1 result: {}", solve_for_part1("data/day09/input.txt"));
}

pub fn part2() {
    println!("Day 9 Part 2 result: {}", solve_for_part2("data/day09/input.txt"));
}
