use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::FilterMap;

pub fn int_value(line: &str) -> i32 {
    return line.parse::<i32>().unwrap_or(0);
}

#[cfg(test)]
mod int_value_tests {
    use super::*;

    #[test]
    fn test_int_value_empty() {
        assert_eq!(int_value(""), 0);
    }

    #[test]
    fn test_int_value_invalid() {
        assert_eq!(int_value("xyzzy"), 0);
    }

    #[test]
    fn test_int_value_integer() {
        assert_eq!(int_value("4"), 4);
    }
}

pub fn int_value_with_rule(line: &str, rule: &str) -> i32 {
    if rule == "count" {
        return line.parse::<i32>().map(|_a| 1).unwrap_or(0);
    }
    return line.parse::<i32>().unwrap_or(0);
}

#[cfg(test)]
mod int_value_with_rule_tests {
    use super::*;

    #[test]
    fn test_int_value_with_rule_empty() {
        assert_eq!(int_value_with_rule("", ""), 0);
    }

    #[test]
    fn test_int_value_with_rule_empty_count() {
        assert_eq!(int_value_with_rule("", "count"), 0);
    }

    #[test]
    fn test_int_value_with_rule_invalid() {
        assert_eq!(int_value_with_rule("xyzzy", ""), 0);
    }

    #[test]
    fn test_int_value_with_rule_invalid_count() {
        assert_eq!(int_value_with_rule("xyzzy", "count"), 0);
    }

    #[test]
    fn test_int_value_with_rule_integer() {
        assert_eq!(int_value_with_rule("4", ""), 4);
    }

    #[test]
    fn test_int_value_with_rule_count() {
        assert_eq!(int_value_with_rule("4", "count"), 1);
    }
}

pub fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

#[cfg(test)]
mod sum_tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(3, 4), 7);
    }
}

pub fn line_iterator(filename: &str) -> FilterMap<Lines<BufReader<File>>, fn(std::io::Result<String>) -> Option<String>> {
    return BufReader::new(File::open(filename).expect(&*format!("Failed to open {}", filename)))
        .lines()
        .filter_map(|result| result.ok());
}

pub fn sum_up(filename: &str, map_function: fn(&str) -> i32, reduce_function: fn(i32, i32) -> i32) -> i32 {
    return line_iterator(filename)
        .map(|line| map_function(line.as_str()))
        .reduce(reduce_function)
        .unwrap_or(0);
}

#[cfg(test)]
mod sum_up_tests {
    use super::*;

    #[test]
    fn test_sum_up() {
        assert_eq!(sum_up("data/aoc_common-ints.txt", int_value, sum), 15);
    }
}

pub fn sum_up_with_rule(filename: &str, map_function: fn(&str, &str) -> i32, reduce_function: fn(i32, i32) -> i32, rule: &str) -> i32 {
    return line_iterator(filename)
        .map(|line| map_function(line.as_str(), rule))
        .reduce(reduce_function)
        .unwrap_or(0);
}

#[cfg(test)]
mod sum_up_with_rule_tests {
    use super::*;

    #[test]
    fn test_sum_up_with_rule_empty() {
        assert_eq!(sum_up_with_rule("data/aoc_common-ints.txt", int_value_with_rule, sum, ""), 15);
    }

    #[test]
    fn test_sum_up_with_rule_count() {
        assert_eq!(sum_up_with_rule("data/aoc_common-ints.txt", int_value_with_rule, sum, "count"), 5);
    }
}
