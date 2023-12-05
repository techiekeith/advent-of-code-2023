use crate::aoc_common::main::line_iterator;

struct Number {
    value: i32,
    line_number: i32,
    column_number_start: i32,
    column_number_end: i32,
}

struct Symbol {
    value: char,
    line_number: i32,
    column_number: i32,
}

fn find_symbols_in_line(source: &str, line_number: i32) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = vec![];
    let mut column_number = 0;
    for value in source.chars() {
        if value != '.' && (value < '0' || value > '9') {
            symbols.push(Symbol { value, line_number, column_number });
        }
        column_number = column_number + 1;
    }
    return symbols;
}

#[cfg(test)]
mod find_symbols_in_line_tests {
    use super::*;

    #[test]
    fn test_find_symbols_in_line_empty() {
        let symbols = find_symbols_in_line("", 4);
        assert_eq!(symbols.is_empty(), true);
    }

    #[test]
    fn test_find_symbols_in_line_none_match() {
        let symbols = find_symbols_in_line(".....", 4);
        assert_eq!(symbols.is_empty(), true);
    }

    #[test]
    fn test_find_symbols_in_line_one_match() {
        let symbols = find_symbols_in_line("..$..", 4);
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].value, '$');
        assert_eq!(symbols[0].line_number, 4);
        assert_eq!(symbols[0].column_number, 2);
    }

    #[test]
    fn test_find_symbols_in_line_many_match() {
        let symbols = find_symbols_in_line("..$..*.", 4);
        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].value, '$');
        assert_eq!(symbols[0].line_number, 4);
        assert_eq!(symbols[0].column_number, 2);
        assert_eq!(symbols[1].value, '*');
        assert_eq!(symbols[1].line_number, 4);
        assert_eq!(symbols[1].column_number, 5);
    }
}

fn find_numbers_in_line(source: &str, line: i32) -> Vec<Number> {
    let mut numbers: Vec<Number> = vec![];
    let mut number = 0;
    let mut column_number = 0;
    let mut column_number_start = -1;
    for value in source.chars() {
        if value >= '0' && value <= '9' {
            let digit = value as i32 - 48;
            if column_number_start < 0 {
                column_number_start = column_number;
                number = digit;
            } else {
                number = number * 10 + digit;
            }
        } else if column_number_start >= 0 {
            numbers.push(Number { value: number, line_number: line, column_number_start, column_number_end: column_number - 1 });
            column_number_start = -1;
        }
        column_number = column_number + 1;
    }
    if column_number_start >= 0 {
        numbers.push(Number { value: number, line_number: line, column_number_start, column_number_end: column_number - 1 });
    }
    return numbers;
}

#[cfg(test)]
mod find_numbers_in_line_tests {
    use super::*;

    #[test]
    fn test_find_numbers_in_line_empty() {
        let numbers = find_numbers_in_line("", 4);
        assert_eq!(numbers.is_empty(), true);
    }

    #[test]
    fn test_find_numbers_in_line_none_match() {
        let numbers = find_numbers_in_line(".#...", 4);
        assert_eq!(numbers.is_empty(), true);
    }

    #[test]
    fn test_find_numbers_in_line_one_match() {
        let numbers = find_numbers_in_line(".#.234..", 4);
        assert_eq!(numbers.len(), 1);
        assert_eq!(numbers[0].value, 234);
        assert_eq!(numbers[0].line_number, 4);
        assert_eq!(numbers[0].column_number_start, 3);
        assert_eq!(numbers[0].column_number_end, 5);
    }

    #[test]
    fn test_find_numbers_in_line_many_match() {
        let numbers = find_numbers_in_line(".#.234..567", 4);
        assert_eq!(numbers.len(), 2);
        assert_eq!(numbers[0].value, 234);
        assert_eq!(numbers[0].line_number, 4);
        assert_eq!(numbers[0].column_number_start, 3);
        assert_eq!(numbers[0].column_number_end, 5);
        assert_eq!(numbers[1].value, 567);
        assert_eq!(numbers[1].line_number, 4);
        assert_eq!(numbers[1].column_number_start, 8);
        assert_eq!(numbers[1].column_number_end, 10);
    }
}

fn is_adjacent(number: &Number, symbol: &Symbol) -> bool {
    return symbol.column_number >= number.column_number_start - 1
        && symbol.column_number <= number.column_number_end + 1
        && symbol.line_number >= number.line_number - 1
        && symbol.line_number <= number.line_number + 1;
}

#[cfg(test)]
mod is_adjacent_tests {
    use super::*;

    #[test]
    fn test_is_adjacent_out_of_range_vertical() {
        let number = Number {
            value: 123,
            line_number: 4,
            column_number_start: 1,
            column_number_end: 3,
        };
        let symbol = Symbol {
            value: '*',
            line_number: 6,
            column_number: 2,
        };
        assert_eq!(is_adjacent(&number, &symbol), false);
    }

    #[test]
    fn test_is_adjacent_out_of_range_horizontal() {
        let number = Number {
            value: 123,
            line_number: 4,
            column_number_start: 1,
            column_number_end: 3,
        };
        let symbol = Symbol {
            value: '*',
            line_number: 4,
            column_number: 5,
        };
        assert_eq!(is_adjacent(&number, &symbol), false);
    }

    #[test]
    fn test_is_adjacent_in_range_northeast() {
        let number = Number {
            value: 123,
            line_number: 4,
            column_number_start: 1,
            column_number_end: 3,
        };
        let symbol = Symbol {
            value: '*',
            line_number: 3,
            column_number: 4,
        };
        assert_eq!(is_adjacent(&number, &symbol), true);
    }

    #[test]
    fn test_is_adjacent_in_range_southeast() {
        let number = Number {
            value: 123,
            line_number: 4,
            column_number_start: 1,
            column_number_end: 3,
        };
        let symbol = Symbol {
            value: '*',
            line_number: 5,
            column_number: 4,
        };
        assert_eq!(is_adjacent(&number, &symbol), true);
    }

    #[test]
    fn test_is_adjacent_in_range_southwest() {
        let number = Number {
            value: 123,
            line_number: 4,
            column_number_start: 1,
            column_number_end: 3,
        };
        let symbol = Symbol {
            value: '*',
            line_number: 5,
            column_number: 0,
        };
        assert_eq!(is_adjacent(&number, &symbol), true);
    }

    #[test]
    fn test_is_adjacent_in_range_northwest() {
        let number = Number {
            value: 123,
            line_number: 4,
            column_number_start: 1,
            column_number_end: 3,
        };
        let symbol = Symbol {
            value: '*',
            line_number: 3,
            column_number: 0,
        };
        assert_eq!(is_adjacent(&number, &symbol), true);
    }
}

fn is_part_number(number: &Number, symbols: &Vec<Symbol>) -> bool {
    for symbol in symbols {
        if is_adjacent(number, symbol) {
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
            line_number: 4,
            column_number_start: 1,
            column_number_end: 3,
        };
        let symbols: Vec<Symbol> = vec![];
        assert_eq!(is_part_number(&number, &symbols), false);
    }

    #[test]
    fn test_is_part_number_out_of_range() {
        let number = Number {
            value: 123,
            line_number: 4,
            column_number_start: 1,
            column_number_end: 3,
        };
        let symbol = Symbol {
            value: '*',
            line_number: 6,
            column_number: 2,
        };
        let symbols: Vec<Symbol> = vec![symbol];
        assert_eq!(is_part_number(&number, &symbols), false);
    }

    #[test]
    fn test_is_part_number_in_range() {
        let number = Number {
            value: 123,
            line_number: 4,
            column_number_start: 1,
            column_number_end: 3,
        };
        let symbol = Symbol {
            value: '*',
            line_number: 5,
            column_number: 4,
        };
        let symbols: Vec<Symbol> = vec![symbol];
        assert_eq!(is_part_number(&number, &symbols), true);
    }
}

fn part_numbers_in_range(symbol: &Symbol, numbers: &Vec<Number>) -> Vec<i32> {
    let mut matches: Vec<i32> = vec![];
    for number in numbers {
        if is_adjacent(number, symbol) {
            matches.push(number.value);
        }
    }
    return matches;
}

#[cfg(test)]
mod part_numbers_in_range_tests {
    use super::*;

    #[test]
    fn test_part_numbers_in_range_empty() {
        let symbol = Symbol {
            value: '$',
            line_number: 4,
            column_number: 5,
        };
        let numbers: Vec<Number> = vec![];
        assert_eq!(part_numbers_in_range(&symbol, &numbers).is_empty(), true);
    }

    #[test]
    fn test_part_numbers_in_range_none_match() {
        let symbol = Symbol {
            value: '$',
            line_number: 4,
            column_number: 5,
        };
        let number = Number {
            value: 123,
            line_number: 4,
            column_number_start: 1,
            column_number_end: 3,
        };
        let numbers: Vec<Number> = vec![number];
        assert_eq!(part_numbers_in_range(&symbol, &numbers).is_empty(), true);
    }

    #[test]
    fn test_part_numbers_in_range_one_match() {
        let symbol = Symbol {
            value: '$',
            line_number: 4,
            column_number: 4,
        };
        let number = Number {
            value: 123,
            line_number: 4,
            column_number_start: 1,
            column_number_end: 3,
        };
        let numbers: Vec<Number> = vec![number];
        let matches = part_numbers_in_range(&symbol, &numbers);
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_part_numbers_in_range_two_match() {
        let symbol = Symbol {
            value: '$',
            line_number: 4,
            column_number: 4,
        };
        let number_one = Number {
            value: 123,
            line_number: 4,
            column_number_start: 1,
            column_number_end: 3,
        };
        let number_two = Number {
            value: 456,
            line_number: 5,
            column_number_start: 5,
            column_number_end: 7,
        };
        let numbers: Vec<Number> = vec![number_one, number_two];
        let matches = part_numbers_in_range(&symbol, &numbers);
        assert_eq!(matches.len(), 2);
    }
}

fn gear_ratio(symbol: &Symbol, numbers: &Vec<Number>) -> i32 {
    if symbol.value != '*' {
        return 0;
    }
    let matches = part_numbers_in_range(symbol, numbers);
    if matches.len() != 2 {
        return 0;
    }
    return matches[0] * matches[1];
}

#[cfg(test)]
mod gear_ratio_tests {
    use super::*;

    #[test]
    fn test_gear_ratio_empty() {
        let symbol = Symbol {
            value: '*',
            line_number: 4,
            column_number: 5,
        };
        let numbers: Vec<Number> = vec![];
        assert_eq!(gear_ratio(&symbol, &numbers), 0);
    }

    #[test]
    fn test_gear_ratio_none_match() {
        let symbol = Symbol {
            value: '*',
            line_number: 4,
            column_number: 5,
        };
        let number = Number {
            value: 123,
            line_number: 4,
            column_number_start: 1,
            column_number_end: 3,
        };
        let numbers: Vec<Number> = vec![number];
        assert_eq!(gear_ratio(&symbol, &numbers), 0);
    }

    #[test]
    fn test_gear_ratio_two_match_but_symbol_is_not_a_star() {
        let symbol = Symbol {
            value: '$',
            line_number: 4,
            column_number: 4,
        };
        let number_one = Number {
            value: 123,
            line_number: 4,
            column_number_start: 1,
            column_number_end: 3,
        };
        let number_two = Number {
            value: 456,
            line_number: 5,
            column_number_start: 5,
            column_number_end: 7,
        };
        let numbers: Vec<Number> = vec![number_one, number_two];
        assert_eq!(gear_ratio(&symbol, &numbers), 0);
    }

    #[test]
    fn test_gear_ratio_one_match() {
        let symbol = Symbol {
            value: '*',
            line_number: 4,
            column_number: 4,
        };
        let number = Number {
            value: 123,
            line_number: 4,
            column_number_start: 1,
            column_number_end: 3,
        };
        let numbers: Vec<Number> = vec![number];
        assert_eq!(gear_ratio(&symbol, &numbers), 0);
    }

    #[test]
    fn test_gear_ratio_two_match() {
        let symbol = Symbol {
            value: '*',
            line_number: 4,
            column_number: 4,
        };
        let number_one = Number {
            value: 123,
            line_number: 4,
            column_number_start: 1,
            column_number_end: 3,
        };
        let number_two = Number {
            value: 456,
            line_number: 5,
            column_number_start: 5,
            column_number_end: 7,
        };
        let numbers: Vec<Number> = vec![number_one, number_two];
        assert_eq!(gear_ratio(&symbol, &numbers), 56088);
    }

    #[test]
    fn test_gear_ratio_three_match() {
        let symbol = Symbol {
            value: '*',
            line_number: 4,
            column_number: 4,
        };
        let number_one = Number {
            value: 123,
            line_number: 4,
            column_number_start: 1,
            column_number_end: 3,
        };
        let number_two = Number {
            value: 456,
            line_number: 5,
            column_number_start: 5,
            column_number_end: 7,
        };
        let number_three = Number {
            value: 789,
            line_number: 5,
            column_number_start: 5,
            column_number_end: 7,
        };
        let numbers: Vec<Number> = vec![number_one, number_two, number_three];
        assert_eq!(gear_ratio(&symbol, &numbers), 0);
    }
}

fn read_schematic(filename: &str) -> (Vec<Symbol>, Vec<Number>) {
    let mut symbols: Vec<Symbol> = vec![];
    let mut numbers: Vec<Number> = vec![];
    let lines = line_iterator(filename);
    let mut line_number = 0;
    for line in lines {
        symbols.append(&mut find_symbols_in_line(line.as_str(), line_number));
        numbers.append(&mut find_numbers_in_line(line.as_str(), line_number));
        line_number = line_number + 1;
    }
    return (symbols, numbers);
}

#[cfg(test)]
mod read_schematic_tests {
    use super::*;

    #[test]
    fn test_read_schematic() {
        let (symbols, numbers) = read_schematic("data/day03/test.txt");
        assert_eq!(numbers.len(), 10);
        assert_eq!(symbols.len(), 6);
    }
}

fn read_schematic_part1(filename: &str) -> i32 {
    let (symbols, numbers) = read_schematic(filename);
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
    fn test_read_schematic_part1() {
        assert_eq!(read_schematic_part1("data/day03/test.txt"), 4361);
    }
}

fn read_schematic_part2(filename: &str) -> i32 {
    let (symbols, numbers) = read_schematic(filename);
    let mut sum = 0;
    for symbol in symbols {
        sum += gear_ratio(&symbol, &numbers);
    }
    return sum;
}

#[cfg(test)]
mod read_schematic_part2_tests {
    use super::*;

    #[test]
    fn test_read_schematic_part2() {
        assert_eq!(read_schematic_part2("data/day03/test.txt"), 467835);
    }
}

pub fn part1() {
    println!("Day 3 Part 1 result: {}", read_schematic_part1("data/day03/input.txt"));
}

pub fn part2() {
    println!("Day 3 Part 2 result: {}", read_schematic_part2("data/day03/input.txt"));
}
