use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::FilterMap;
use num::Num;

pub fn get_series_of_ints<T: std::str::FromStr>(source: &str) -> Vec<T> {
    return source.split(" ").filter_map(|n| n.parse::<T>().ok()).collect();
}

#[cfg(test)]
mod get_series_of_ints_tests {
    use super::*;

    #[test]
    fn test_get_series_of_ints_empty() {
        let numbers: Vec<i32> = get_series_of_ints("");
        assert_eq!(numbers.is_empty(), true);
    }

    #[test]
    fn test_get_series_of_ints() {
        let numbers: Vec<i32> = get_series_of_ints("41 48 83 86 17");
        assert_eq!(numbers.len(), 5);
    }
}

pub fn sum<T: Num>(a: T, b: T) -> T {
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

#[cfg(test)]
mod line_iterator_tests {
    use super::*;

    #[test]
    fn test_line_iterator() {
        let lines = line_iterator("data/aoc_common/test_data.txt");
        let line: Vec<String> = lines.collect();
        assert_eq!(line.len(), 5);
        assert_eq!(line[0].as_str(), "1");
        assert_eq!(line[4].as_str(), "5");
    }
}

pub fn sum_up<T: Num>(filename: &str, map_function: fn(&str) -> T, reduce_function: fn(T, T) -> T) -> T {
    return line_iterator(filename)
        .map(|line| map_function(line.as_str()))
        .reduce(reduce_function)
        .unwrap_or(num::zero::<T>());
}

#[cfg(test)]
mod sum_up_tests {
    use super::*;

    fn int_value(line: &str) -> i32 {
        return line.parse::<i32>().unwrap_or(0);
    }

    #[test]
    fn test_sum_up() {
        assert_eq!(sum_up("data/aoc_common/test_data.txt", int_value, sum), 15);
    }
}

pub fn sum_up_with_rule<T: Num>(filename: &str, map_function: fn(&str, &str) -> T, reduce_function: fn(T, T) -> T, rule: &str) -> T {
    return line_iterator(filename)
        .map(|line| map_function(line.as_str(), rule))
        .reduce(reduce_function)
        .unwrap_or(num::zero::<T>());
}

#[cfg(test)]
mod sum_up_with_rule_tests {
    use super::*;

    fn int_value_with_rule(line: &str, rule: &str) -> i32 {
        if rule == "count" {
            return line.parse::<i32>().map(|_a| 1).unwrap_or(0);
        }
        return line.parse::<i32>().unwrap_or(0);
    }

    #[test]
    fn test_sum_up_with_rule_empty() {
        assert_eq!(sum_up_with_rule("data/aoc_common/test_data.txt", int_value_with_rule, sum, ""), 15);
    }

    #[test]
    fn test_sum_up_with_rule_count() {
        assert_eq!(sum_up_with_rule("data/aoc_common/test_data.txt", int_value_with_rule, sum, "count"), 5);
    }
}
