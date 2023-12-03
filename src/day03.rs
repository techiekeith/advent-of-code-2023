use crate::aoc_common::line_iterator;

struct Number {
    value: i32,
    line: i32,
    column_start: i32,
    column_end: i32,
}

struct Symbol {
    value: char,
    line: i32,
    column: i32,
}

fn find_symbols(source: &str, line: i32) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = vec![];
    let mut column = 0;
    for value in source.chars() {
        if value != '.' && (value < '0' || value > '9') {
            symbols.push(Symbol { value, line, column });
        }
        column = column + 1;
    }
    return symbols;
}

#[cfg(test)]
mod find_symbols_tests {
    use super::*;

    #[test]
    fn test_find_symbols_empty() {
        let symbols = find_symbols("", 4);
        assert_eq!(symbols.is_empty(), true);
    }

    #[test]
    fn test_find_symbols_none_match() {
        let symbols = find_symbols(".....", 4);
        assert_eq!(symbols.is_empty(), true);
    }

    #[test]
    fn test_find_symbols_one_match() {
        let symbols = find_symbols("..$..", 4);
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].value, '$');
        assert_eq!(symbols[0].line, 4);
        assert_eq!(symbols[0].column, 2);
    }

    #[test]
    fn test_find_symbols_many_match() {
        let symbols = find_symbols("..$..*.", 4);
        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].value, '$');
        assert_eq!(symbols[0].line, 4);
        assert_eq!(symbols[0].column, 2);
        assert_eq!(symbols[1].value, '*');
        assert_eq!(symbols[1].line, 4);
        assert_eq!(symbols[1].column, 5);
    }
}

fn find_numbers(source: &str, line: i32) -> Vec<Number> {
    let mut numbers: Vec<Number> = vec![];
    let mut column = 0;
    let mut column_start = -1;
    let mut number = 0;
    for value in source.chars() {
        if value >= '0' && value <= '9' {
            let digit = value as i32 - 48;
            if column_start < 0 {
                column_start = column;
                number = digit;
            } else {
                number = number * 10 + digit;
            }
        } else if column_start >= 0 {
            numbers.push(Number { value: number, line, column_start, column_end: column - 1 });
            column_start = -1;
        }
        column = column + 1;
    }
    if column_start >= 0 {
        numbers.push(Number { value: number, line, column_start, column_end: column - 1 });
    }
    return numbers;
}

#[cfg(test)]
mod find_numbers_tests {
    use super::*;

    #[test]
    fn test_find_numbers_empty() {
        let numbers = find_numbers("", 4);
        assert_eq!(numbers.is_empty(), true);
    }

    #[test]
    fn test_find_numbers_none_match() {
        let numbers = find_numbers(".#...", 4);
        assert_eq!(numbers.is_empty(), true);
    }

    #[test]
    fn test_find_numbers_one_match() {
        let numbers = find_numbers(".#.234..", 4);
        assert_eq!(numbers.len(), 1);
        assert_eq!(numbers[0].value, 234);
        assert_eq!(numbers[0].line, 4);
        assert_eq!(numbers[0].column_start, 3);
        assert_eq!(numbers[0].column_end, 5);
    }

    #[test]
    fn test_find_numbers_many_match() {
        let numbers = find_numbers(".#.234..567", 4);
        assert_eq!(numbers.len(), 2);
        assert_eq!(numbers[0].value, 234);
        assert_eq!(numbers[0].line, 4);
        assert_eq!(numbers[0].column_start, 3);
        assert_eq!(numbers[0].column_end, 5);
        assert_eq!(numbers[1].value, 567);
        assert_eq!(numbers[1].line, 4);
        assert_eq!(numbers[1].column_start, 8);
        assert_eq!(numbers[1].column_end, 10);
    }
}

fn is_part_number(number: &Number, symbols: &Vec<Symbol>) -> bool {
    for symbol in symbols {
        if symbol.column >= number.column_start - 1
            && symbol.column <= number.column_end + 1
            && symbol.line >= number.line - 1
            && symbol.line <= number.line + 1 {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod is_part_number_tests {
    use super::*;

    #[test]
    fn test_is_part_number_empty() {
        let number = Number {
            value: 123,
            line: 4,
            column_start: 1,
            column_end: 3,
        };
        let symbols: Vec<Symbol> = vec![];
        assert_eq!(is_part_number(&number, &symbols), false);
    }

    #[test]
    fn test_is_part_number_out_of_range_vertical() {
        let number = Number {
            value: 123,
            line: 4,
            column_start: 1,
            column_end: 3,
        };
        let symbol = Symbol {
            value: '*',
            line: 6,
            column: 2,
        };
        let symbols: Vec<Symbol> = vec![symbol];
        assert_eq!(is_part_number(&number, &symbols), false);
    }

    #[test]
    fn test_is_part_number_out_of_range_horizontal() {
        let number = Number {
            value: 123,
            line: 4,
            column_start: 1,
            column_end: 3,
        };
        let symbol = Symbol {
            value: '*',
            line: 4,
            column: 5,
        };
        let symbols: Vec<Symbol> = vec![symbol];
        assert_eq!(is_part_number(&number, &symbols), false);
    }

    #[test]
    fn test_is_part_number_in_range_northeast() {
        let number = Number {
            value: 123,
            line: 4,
            column_start: 1,
            column_end: 3,
        };
        let symbol = Symbol {
            value: '*',
            line: 3,
            column: 4,
        };
        let symbols: Vec<Symbol> = vec![symbol];
        assert_eq!(is_part_number(&number, &symbols), true);
    }

    #[test]
    fn test_is_part_number_in_range_southeast() {
        let number = Number {
            value: 123,
            line: 4,
            column_start: 1,
            column_end: 3,
        };
        let symbol = Symbol {
            value: '*',
            line: 5,
            column: 4,
        };
        let symbols: Vec<Symbol> = vec![symbol];
        assert_eq!(is_part_number(&number, &symbols), true);
    }

    #[test]
    fn test_is_part_number_in_range_southwest() {
        let number = Number {
            value: 123,
            line: 4,
            column_start: 1,
            column_end: 3,
        };
        let symbol = Symbol {
            value: '*',
            line: 5,
            column: 0,
        };
        let symbols: Vec<Symbol> = vec![symbol];
        assert_eq!(is_part_number(&number, &symbols), true);
    }

    #[test]
    fn test_is_part_number_in_range_northwest() {
        let number = Number {
            value: 123,
            line: 4,
            column_start: 1,
            column_end: 3,
        };
        let symbol = Symbol {
            value: '*',
            line: 3,
            column: 0,
        };
        let symbols: Vec<Symbol> = vec![symbol];
        assert_eq!(is_part_number(&number, &symbols), true);
    }
}

fn read_schematic_part1(filename: &str) -> i32 {
    let mut symbols: Vec<Symbol> = vec![];
    let mut numbers: Vec<Number> = vec![];
    let lines = line_iterator(filename);
    let mut count = 0;
    for line in lines {
        symbols.append(&mut find_symbols(line.as_str(), count));
        numbers.append(&mut find_numbers(line.as_str(), count));
        count = count + 1;
    }
    let mut sum = 0;
    for number in numbers {
        if is_part_number(&number, &symbols) {
            sum += number.value;
        }
    }
    return sum;
}

#[cfg(test)]
mod read_schematic_part1_tests {
    use super::*;

    #[test]
    fn test_read_schematic() {
        assert_eq!(read_schematic_part1("data/day03-test.txt"), 4361);
    }
}

pub fn part1() {
    println!("Day 3 Part 1 result: {}", read_schematic_part1("data/day03-input.txt"));
}

pub fn part2() {
    println!("Day 3 Part 2 result: {}", 0);
}
