use crate::aoc_common::main::{line_iterator, sum, sum_up};

fn get_space_separated_numbers(source: &str) -> Vec<i32> {
    return source.split(" ").filter_map(|n| n.parse::<i32>().ok()).collect();
}

#[cfg(test)]
mod get_space_separated_numbers_tests {
    use super::*;

    #[test]
    fn test_get_space_separated_numbers_empty() {
        let numbers = get_space_separated_numbers("");
        assert_eq!(numbers.is_empty(), true);
    }

    #[test]
    fn test_get_space_separated_numbers() {
        let numbers = get_space_separated_numbers("41 48 83 86 17");
        assert_eq!(numbers.len(), 5);
    }
}

fn get_scratchcard_numbers(line: &str) -> Option<(i32, Vec<i32>, Vec<i32>)> {
    let colon = line.find(':');
    let pipe = line.find('|');
    if colon.is_none() || pipe.is_none() {
        return None;
    }
    let first = colon.unwrap();
    let second = pipe.unwrap();
    let card_numbers = get_space_separated_numbers(&line[..first]);
    if card_numbers.len() != 1 {
        return None;
    }
    let winners = get_space_separated_numbers(&line[first..second]);
    let selected = get_space_separated_numbers(&line[second..]);
    return Some((card_numbers[0], winners, selected));
}

#[cfg(test)]
mod get_scratchcard_numbers_tests {
    use super::*;

    #[test]
    fn test_get_scratchcard_numbers_empty() {
        let result = get_scratchcard_numbers("");
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_scratchcard_numbers() {
        let result = get_scratchcard_numbers("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(result.is_some(), true);
        let (card_number, winners, selected) = result.unwrap();
        assert_eq!(card_number, 1);
        assert_eq!(winners.len(), 5);
        assert_eq!(selected.len(), 8);
    }
}

fn find(n: &i32, v: &Vec<i32>) -> Option<usize> {
    return v.iter().position(|i| i == n);
}

#[cfg(test)]
mod find_tests {
    use super::*;

    #[test]
    fn test_find_empty() {
        assert_eq!(find(&10, &vec![]), None);
    }

    #[test]
    fn test_find_none_match() {
        assert_eq!(find(&10, &vec![1, 2, 3, 4, 5]), None);
    }

    #[test]
    fn test_find_one_match() {
        assert_eq!(find(&10, &vec![1, 3, 6, 10, 15]), Some(3));
    }
}

fn count_all_matches(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let mut matches = 0;
    for number in a {
        if find(number, b).is_some() {
            matches = matches + 1;
        }
    }
    return matches;
}

#[cfg(test)]
mod count_all_matches_tests {
    use super::*;

    #[test]
    fn test_count_all_matches_lhs_empty() {
        assert_eq!(count_all_matches(&vec![], &vec![1, 2, 3, 4, 5]), 0);
    }

    #[test]
    fn test_count_all_matches_rhs_empty() {
        assert_eq!(count_all_matches(&vec![1, 2, 3, 4, 5], &vec![]), 0);
    }

    #[test]
    fn test_count_all_matches_none_match() {
        assert_eq!(count_all_matches(&vec![1, 2, 3, 4, 5], &vec![6, 7, 8, 9, 10]), 0);
    }

    #[test]
    fn test_count_all_matches_some_match() {
        assert_eq!(count_all_matches(&vec![1, 2, 3, 4, 5], &vec![2, 3, 8, 9, 10]), 2);
    }
}

fn matches_for_line(line: &str) -> i32 {
    let result = get_scratchcard_numbers(line);
    if result.is_none() {
        return 0;
    }
    let (_, winners, selected) = result.unwrap();
    return count_all_matches(&winners, &selected);
}

#[cfg(test)]
mod matches_for_line_tests {
    use super::*;

    #[test]
    fn test_matches_for_line_empty() {
        assert_eq!(matches_for_line(""), 0);
    }

    #[test]
    fn test_matches_for_line_none_match() {
        assert_eq!(matches_for_line("Card 1: 1 2 3 4 5 | 6 7 8 9 10 11 12"), 0);
    }

    #[test]
    fn test_matches_for_line_one_match() {
        assert_eq!(matches_for_line("Card 1: 1 2 3 4 5 | 5 6 7 8 9 10 11 12"), 1);
    }

    #[test]
    fn test_matches_for_line_four_match() {
        assert_eq!(matches_for_line("Card 1: 1 2 3 4 5 6 7 8 | 5 6 7 8 9 10 11 12"), 4);
    }
}

fn score_for_line(line: &str) -> i32 {
    let matches = matches_for_line(line);
    if matches == 0 {
        return 0;
    }
    return 1 << (matches - 1);
}

#[cfg(test)]
mod score_for_line_tests {
    use super::*;

    #[test]
    fn test_score_for_line_empty() {
        assert_eq!(score_for_line(""), 0);
    }

    #[test]
    fn test_score_for_line_none_match() {
        assert_eq!(score_for_line("Card 1: 1 2 3 4 5 | 6 7 8 9 10 11 12"), 0);
    }

    #[test]
    fn test_score_for_line_one_match() {
        assert_eq!(score_for_line("Card 1: 1 2 3 4 5 | 5 6 7 8 9 10 11 12"), 1);
    }

    #[test]
    fn test_score_for_line_four_match() {
        assert_eq!(score_for_line("Card 1: 1 2 3 4 5 6 7 8 | 5 6 7 8 9 10 11 12"), 8);
    }
}

fn score_for_file(filename: &str) -> i32 {
    return sum_up(filename, score_for_line, sum);
}

#[cfg(test)]
mod score_for_file_tests {
    use super::*;

    #[test]
    fn test_score_for_file() {
        assert_eq!(score_for_file("data/day04/test.txt"), 13);
    }
}

fn count_cards_for_file(filename: &str) -> i32 {
    let lines = line_iterator(filename);
    let matches: Vec<i32> = lines.map(|line| matches_for_line(line.as_str())).collect();
    let size = matches.len();
    let mut totals: Vec<i32> = vec![0; size];
    for i in 0..size {
        totals[i] += 1;
        for j in 0..matches[i] {
            totals[i+j as usize+1] += totals[i];
        }
    }
    return totals.iter().fold(0, |a, b| a + b);
}

#[cfg(test)]
mod count_cards_for_file_tests {
    use super::*;

    #[test]
    fn test_count_cards_for_file() {
        assert_eq!(count_cards_for_file("data/day04/test.txt"), 30);
    }
}

pub fn part1() {
    println!("Day 4 Part 1 result: {}", score_for_file("data/day04/input.txt"));
}

pub fn part2() {
    println!("Day 4 Part 2 result: {}", count_cards_for_file("data/day04/input.txt"));
}
