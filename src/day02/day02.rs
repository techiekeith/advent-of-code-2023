use std::collections::HashMap;
use crate::aoc_common::lib::{sum, sum_up, sum_up_with_rule};

fn get_rgb_for_phrase(round: &str) -> HashMap<&str, i32> {
    let mut colour_map = HashMap::new();
    let words: Vec<&str> = round.split(" ").collect();
    if words.len() == 2 {
        let number = words[0].parse::<i32>().unwrap();
        colour_map.insert(words[1], number);
    }
    return colour_map;
}

#[cfg(test)]
mod get_rgb_for_phrase_tests {
    use super::*;

    #[test]
    fn test_get_rgb_for_phrase_empty() {
        let colour_map = get_rgb_for_phrase("");
        assert_eq!(colour_map.is_empty(), true);
    }

    #[test]
    fn test_get_rgb_for_phrase_1_red() {
        let colour_map = get_rgb_for_phrase("1 red");
        assert_eq!(colour_map.is_empty(), false);
        assert_eq!(colour_map["red"], 1);
    }
}

fn sum_maps<'a>(a: HashMap<&'a str, i32>, b: HashMap<&'a str, i32>) -> HashMap<&'a str, i32> {
    let mut sum: HashMap<&str, i32> = HashMap::new();
    for key in a.keys() {
        sum.insert(key, a[key] + b.get(key).unwrap_or(&0));
    }
    for key in b.keys() {
        if !a.contains_key(key) {
            sum.insert(key, b[key]);
        }
    }
    return sum;
}

#[cfg(test)]
mod sum_maps_tests {
    use super::*;

    #[test]
    fn test_sum_maps_empty() {
        let a = HashMap::new();
        let b = HashMap::new();
        let result = sum_maps(a, b);
        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn test_sum_maps_some() {
        let a = HashMap::from([
            ("red", 1),
            ("green", 2),
        ]);
        let b = HashMap::from([
            ("red", 7),
            ("blue", 4),
        ]);
        let result = sum_maps(a, b);
        assert_eq!(result.is_empty(), false);
        assert_eq!(result["red"], 8);
        assert_eq!(result["green"], 2);
        assert_eq!(result["blue"], 4);
    }
}

fn get_rgb_for_round(round: &str) -> HashMap<&str, i32> {
    let phrases: Vec<&str> = round.split(", ").collect();
    let mut colour_map = HashMap::new();
    for phrase in phrases {
        colour_map = sum_maps(colour_map, get_rgb_for_phrase(phrase));
    }
    return colour_map;
}

#[cfg(test)]
mod get_rgb_for_round_tests {
    use super::*;

    #[test]
    fn test_get_rgb_for_round_empty() {
        let colour_map = get_rgb_for_round("");
        assert_eq!(colour_map.is_empty(), true);
    }

    #[test]
    fn test_get_rgb_for_round_1_red_2_green() {
        let colour_map = get_rgb_for_round("1 red, 2 green");
        assert_eq!(colour_map.is_empty(), false);
        assert_eq!(colour_map["red"], 1);
        assert_eq!(colour_map["green"], 2);
    }
}

fn is_round_possible(round: HashMap<&str, i32>, max: HashMap<&str, i32>) -> bool {
    for key in round.keys() {
        if round[key] > *max.get(key).unwrap_or(&0) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod is_round_possible_tests {
    use super::*;

    #[test]
    fn test_is_round_possible_empty_round() {
        let round = get_rgb_for_round("");
        let max = get_rgb_for_round("12 red, 13 green, 14 blue");
        assert_eq!(is_round_possible(round, max), true);
    }

    #[test]
    fn test_is_round_possible_empty_max() {
        let round = get_rgb_for_round("12 red, 13 green, 14 blue");
        let max = get_rgb_for_round("");
        assert_eq!(is_round_possible(round, max), false);
    }

    #[test]
    fn test_is_round_possible_some_values_have_no_max() {
        let round = get_rgb_for_round("12 red, 13 green, 14 blue");
        let max = get_rgb_for_round("12 red, 14 blue");
        assert_eq!(is_round_possible(round, max), false);
    }

    #[test]
    fn test_is_round_possible_some_values_are_not_drawn() {
        let round = get_rgb_for_round("12 red, 14 blue");
        let max = get_rgb_for_round("12 red, 13 green, 14 blue");
        assert_eq!(is_round_possible(round, max), true);
    }

    #[test]
    fn test_is_round_possible_some_values_are_too_many() {
        let round = get_rgb_for_round("12 red, 14 green, 14 blue");
        let max = get_rgb_for_round("12 red, 13 green, 14 blue");
        assert_eq!(is_round_possible(round, max), false);
    }

    #[test]
    fn test_is_round_possible_some_values_are_fewer() {
        let round = get_rgb_for_round("12 red, 12 green, 14 blue");
        let max = get_rgb_for_round("12 red, 13 green, 14 blue");
        assert_eq!(is_round_possible(round, max), true);
    }

    #[test]
    fn test_is_round_possible_all_values_are_equal() {
        let round = get_rgb_for_round("12 red, 13 green, 14 blue");
        let max = get_rgb_for_round("12 red, 13 green, 14 blue");
        assert_eq!(is_round_possible(round, max), true);
    }
}

fn is_game_possible(game: &str, max: &str) -> bool {
    let max_map = get_rgb_for_round(max);
    let rounds: Vec<&str> = game.split("; ").collect();
    for round in rounds {
        if !is_round_possible(get_rgb_for_round(round), max_map.clone()) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod is_game_possible_tests {
    use super::*;

    #[test]
    fn test_is_game_possible_empty_game() {
        let game = "";
        let max = "12 red, 13 green, 14 blue";
        assert_eq!(is_game_possible(game, max), true);
    }

    #[test]
    fn test_is_game_possible_one_game_fails() {
        let game = "5 red, 6 green; 12 green; 13 red, 4 blue";
        let max = "12 red, 13 green, 14 blue";
        assert_eq!(is_game_possible(game, max), false);
    }

    #[test]
    fn test_is_game_possible_all_games_pass() {
        let game = "5 red, 6 green; 12 green; 12 red, 4 blue";
        let max = "12 red, 13 green, 14 blue";
        assert_eq!(is_game_possible(game, max), true);
    }
}

fn game_number(game_number_str: &str) -> i32 {
    let game_number_vec: Vec<&str> = game_number_str.split(" ", ).collect();
    if game_number_vec.len() < 2 || game_number_vec[0] != "Game" {
        return 0;
    }
    return game_number_vec[1].parse::<i32>().unwrap();
}

#[cfg(test)]
mod game_number_tests {
    use super::*;

    #[test]
    fn test_game_number_empty_string() {
        assert_eq!(game_number(""), 0);
    }

    #[test]
    fn test_game_number_not_game() {
        assert_eq!(game_number("4"), 0);
    }

    #[test]
    fn test_game_number_valid_string() {
        assert_eq!(game_number("Game 4"), 4);
    }
}

fn possible_game_id(game: &str, max: &str) -> i32 {
    let game_vec: Vec<&str> = game.split(": ", ).collect();
    if game_vec.len() < 2 {
        return 0;
    }
    let game_number = game_number(game_vec[0]);
    let possible = is_game_possible(game_vec[1], max);
    if possible {
        return game_number;
    }
    return 0;
}

#[cfg(test)]
mod possible_game_id_tests {
    use super::*;

    #[test]
    fn test_possible_game_id_empty_string() {
        let game = "";
        let max = "12 red, 13 green, 14 blue";
        assert_eq!(possible_game_id(game, max), 0);
    }

    #[test]
    fn test_possible_game_id_empty_game() {
        let game = "Game 4: ";
        let max = "12 red, 13 green, 14 blue";
        assert_eq!(possible_game_id(game, max), 4);
    }

    #[test]
    fn test_possible_game_id_one_game_fails() {
        let game = "Game 4: 5 red, 6 green; 12 green; 13 red, 4 blue";
        let max = "12 red, 13 green, 14 blue";
        assert_eq!(possible_game_id(game, max), 0);
    }

    #[test]
    fn test_possible_game_id_missing_game_number() {
        let game = "5 red, 6 green; 12 green; 12 red, 4 blue";
        let max = "12 red, 13 green, 14 blue";
        assert_eq!(possible_game_id(game, max), 0);
    }

    #[test]
    fn test_possible_game_id_all_games_pass() {
        let game = "Game 4: 5 red, 6 green; 12 green; 12 red, 4 blue";
        let max = "12 red, 13 green, 14 blue";
        assert_eq!(possible_game_id(game, max), 4);
    }
}

fn update_min<'a>(round: HashMap<&'a str, i32>, current_min: HashMap<&'a str, i32>) -> HashMap<&'a str, i32> {
    let mut new_min = current_min.clone();
    for key in round.keys() {
        let mut min_value = new_min.get(key).unwrap_or(&0);
        if min_value < &round[key] {
            min_value = &round[key];
        }
        new_min.insert(*key, *min_value);
    }
    return new_min;
}

#[cfg(test)]
mod update_min_tests {
    use super::*;

    #[test]
    fn test_update_min() {
        let round = get_rgb_for_round("3 blue, 5 green, 6 yellow");
        let current_min = get_rgb_for_round("4 red, 3 green, 7 yellow");
        let new_min = update_min(round, current_min);
        assert_eq!(new_min["red"], 4);
        assert_eq!(new_min["green"], 5);
        assert_eq!(new_min["blue"], 3);
        assert_eq!(new_min["yellow"], 7);
    }
}

fn get_min_for_game(rounds: &str) -> HashMap<&str, i32> {
    let mut min = HashMap::new();
    let round_vec: Vec<&str> = rounds.split("; ", ).collect();
    for round in round_vec {
        min = update_min(get_rgb_for_round(round), min);
    }
    return min;
}

#[cfg(test)]
mod get_min_for_game_tests {
    use super::*;

    #[test]
    fn test_get_min_for_game() {
        let min = get_min_for_game("3 blue, 5 green, 6 yellow; 4 red, 3 green, 7 yellow");
        assert_eq!(min["red"], 4);
        assert_eq!(min["green"], 5);
        assert_eq!(min["blue"], 3);
        assert_eq!(min["yellow"], 7);
    }
}

fn get_power_for_min(min: HashMap<&str, i32>) -> i32 {
    let mut power = 1;
    for key in min.keys() {
        power *= min[key];
    }
    return power;
}

#[cfg(test)]
mod get_power_for_min_tests {
    use super::*;

    #[test]
    fn test_get_power_for_min() {
        let min = HashMap::from([
            ("red", 4),
            ("green", 5),
            ("blue", 3),
        ]);
        assert_eq!(get_power_for_min(min), 60);
    }
}

fn game_power(game: &str) -> i32 {
    let game_vec: Vec<&str> = game.split(": ", ).collect();
    if game_vec.len() < 2 {
        return 0;
    }
    let min = get_min_for_game(game_vec[1]);
    return get_power_for_min(min);
}

#[cfg(test)]
mod game_power_tests {
    use super::*;

    #[test]
    fn test_game_power() {
        assert_eq!(game_power("Game 1: 3 blue, 5 green, 6 yellow; 4 red, 3 green, 7 yellow"), 420);
    }
}

fn sum_possible_game_ids(filename: &str, max: &str) -> i32 {
    return sum_up_with_rule(filename, possible_game_id, sum, max);
}

#[cfg(test)]
mod sum_possible_game_ids_tests {
    use super::*;

    #[test]
    fn test_sum_possible_game_ids_part1() {
        assert_eq!(sum_possible_game_ids("data/day02/test.txt", "12 red, 13 green, 14 blue"), 8);
    }
}

fn sum_game_powers(filename: &str) -> i32 {
    return sum_up(filename, game_power, sum);
}

#[cfg(test)]
mod sum_game_powers_tests {
    use super::*;

    #[test]
    fn test_sum_game_powers_part2() {
        assert_eq!(sum_game_powers("data/day02/test.txt"), 2286);
    }
}

pub fn part1() {
    println!("Day 2 Part 1 result: {}", sum_possible_game_ids("data/day02/input.txt", "12 red, 13 green, 14 blue"));
}

pub fn part2() {
    println!("Day 2 Part 2 result: {}", sum_game_powers("data/day02/input.txt"));
}
