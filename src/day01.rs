use regex::Regex;
use std::io::{BufRead, BufReader};
use std::fs::File;

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

fn first_digit(line: &str, allow_words: bool) -> i32 {
    let pattern_str: &str;
    if allow_words {
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
        assert_eq!(first_digit("one2three4five6seveneight9ten11twelve", false), 2);
    }

    #[test]
    fn test_first_digit_with_words() {
        assert_eq!(first_digit("one2three4five6seveneight9ten11twelve", true), 1);
    }
}

fn last_digit(line: &str, allow_words: bool) -> i32 {
    let pattern_str: &str;
    if allow_words {
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
        assert_eq!(last_digit("one2three4five6seveneight", false), 6);
    }

    #[test]
    fn test_last_digit_with_words() {
        assert_eq!(last_digit("one2three4five6seveneight", true), 8);
    }
}

fn calibration_value(line: &str, allow_words: bool) -> i32 {
    return 10 * first_digit(line, allow_words) + last_digit(line, allow_words);
}

#[cfg(test)]
mod calibration_value_tests {
    use super::*;

    #[test]
    fn test_calibration_value() {
        assert_eq!(calibration_value("one2three4five6seveneight", false), 26);
    }

    #[test]
    fn test_last_digit_with_words() {
        assert_eq!(calibration_value("one2three4five6seveneight", true), 18);
    }
}

fn sum_calibration_values(filename: &str, allow_words: bool) -> i32 {
    let mut sum = 0;
    let file = File::open(filename).expect(&*format!("Failed to open {}", filename));
    let reader = BufReader::new(file);
    for line in reader.lines().filter_map(|result| result.ok()) {
        sum += calibration_value(line.as_str(), allow_words);
    }
    return sum;
}

#[cfg(test)]
mod sum_calibration_values_tests {
    use super::*;

    #[test]
    fn test_sum_calibration_values_part1() {
        assert_eq!(sum_calibration_values("data/day01-part1-test.txt", false), 142);
    }

    #[test]
    fn test_sum_calibration_values_part2() {
        assert_eq!(sum_calibration_values("data/day01-part2-test.txt", true), 281);
    }
}

pub fn part1() {
    println!("Day 1 Part 1 result: {}", sum_calibration_values("data/day01-input.txt", false));
}

pub fn part2() {
    println!("Day 1 Part 2 result: {}", sum_calibration_values("data/day01-input.txt", true));
}
