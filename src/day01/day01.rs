use regex::Regex;
use crate::aoc_common::lib::{sum, sum_up_with_rule};

const FIRST_DIGIT_ONLY_STR: &str = "\\d";
const FIRST_DIGIT_OR_WORD_STR: &str = "\\d|one|two|three|four|five|six|seven|eight|nine";
const LAST_DIGIT_ONLY_STR: &str = ".*(?<digit>\\d)";
const LAST_DIGIT_OR_WORD_STR: &str = ".*(?<digit>\\d|one|two|three|four|five|six|seven|eight|nine)";

const NUMBER_WORDS: [&str;9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn digit_value(word: &str) -> i32 {
    let index = NUMBER_WORDS.iter().position(|&elem| elem == word);
    if index == None {
        return word.parse::<i32>().unwrap();
    }
    return 1 + index.unwrap() as i32;
}

#[cfg(test)]
mod digit_value_tests {
    use super::*;

    #[test]
    fn test_digit_value_one() {
        assert_eq!(digit_value("one"), 1);
    }

    #[test]
    fn test_digit_value_nine() {
        assert_eq!(digit_value("nine"), 9);
    }

    #[test]
    fn test_digit_value_1() {
        assert_eq!(digit_value("1"), 1);
    }

    #[test]
    fn test_digit_value_9() {
        assert_eq!(digit_value("9"), 9);
    }
}

fn first_digit(line: &str, rule: &str) -> i32 {
    let pattern_str: &str;
    if rule == "words" {
        pattern_str = FIRST_DIGIT_OR_WORD_STR;
    } else {
        pattern_str = FIRST_DIGIT_ONLY_STR;
    }
    let pattern = Regex::new(pattern_str).unwrap();
    let found_match = pattern.find(line).unwrap();
    return digit_value(found_match.as_str());
}

#[cfg(test)]
mod first_digit_tests {
    use super::*;

    #[test]
    fn test_first_digit_no_words() {
        assert_eq!(first_digit("one2three4five6seveneight9ten11twelve", "digits"), 2);
    }

    #[test]
    fn test_first_digit_with_words() {
        assert_eq!(first_digit("one2three4five6seveneight9ten11twelve", "words"), 1);
    }
}

fn last_digit(line: &str, rule: &str) -> i32 {
    let pattern_str: &str;
    if rule == "words" {
        pattern_str = LAST_DIGIT_OR_WORD_STR;
    } else {
        pattern_str = LAST_DIGIT_ONLY_STR;
    }
    let pattern = Regex::new(pattern_str).unwrap();
    let captures = pattern.captures(line).unwrap();
    return digit_value(&captures["digit"]);
}

#[cfg(test)]
mod last_digit_tests {
    use super::*;

    #[test]
    fn test_last_digit_no_words() {
        assert_eq!(last_digit("one2three4five6seveneight", "digits"), 6);
    }

    #[test]
    fn test_last_digit_with_words() {
        assert_eq!(last_digit("one2three4five6seveneight", "words"), 8);
    }
}

fn calibration_value(line: &str, rule: &str) -> i32 {
    return 10 * first_digit(line, rule) + last_digit(line, rule);
}

#[cfg(test)]
mod calibration_value_tests {
    use super::*;

    #[test]
    fn test_calibration_value() {
        assert_eq!(calibration_value("one2three4five6seveneight", "digits"), 26);
    }

    #[test]
    fn test_last_digit_with_words() {
        assert_eq!(calibration_value("one2three4five6seveneight", "words"), 18);
    }
}

fn sum_calibration_values(filename: &str, rule: &str) -> i32 {
    return sum_up_with_rule(filename, calibration_value, sum, rule);
}

#[cfg(test)]
mod sum_calibration_values_tests {
    use super::*;

    #[test]
    fn test_sum_calibration_values_part1() {
        assert_eq!(sum_calibration_values("data/day01/part1_test.txt", "digits"), 142);
    }

    #[test]
    fn test_sum_calibration_values_part2() {
        assert_eq!(sum_calibration_values("data/day01/part2_test.txt", "words"), 281);
    }
}

pub fn part1() {
    println!("Day 1 Part 1 result: {}", sum_calibration_values("data/day01/input.txt", "digits"));
}

pub fn part2() {
    println!("Day 1 Part 2 result: {}", sum_calibration_values("data/day01/input.txt", "words"));
}
