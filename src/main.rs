mod aoc_common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

const DAY_FUNCTIONS: [[fn();2];5] = [
    [day01::main::part1, day01::main::part2],
    [day02::main::part1, day02::main::part2],
    [day03::main::part1, day03::main::part2],
    [day04::main::part1, day04::main::part2],
    [day05::main::part1, day05::main::part2],
];

fn main() {
    let arg = std::env::args().nth(1).expect("no pattern given");
    let strings: Vec<&str> = arg.split('.').collect();
    if strings.len() < 2 {
        eprintln!("Argument should be <day#>.<part#>");
        std::process::exit(1);
    }

    let day = strings[0].parse::<i32>().unwrap();
    let part = strings[1].parse::<i32>().unwrap();

    let max_days = DAY_FUNCTIONS.len() as i32;
    if day <= 0 || day > max_days {
        eprintln!("Day must be between 1 and {}", max_days);
        std::process::exit(1);
    }
    if part != 1 && part != 2 {
        eprintln!("Part must be 1 or 2");
        std::process::exit(1);
    }

    println!("Day: {}, part: {}", day, part);

    DAY_FUNCTIONS[(day - 1) as usize][(part - 1) as usize]();
}
