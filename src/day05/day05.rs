use crate::aoc_common::lib::{get_series_of_ints, line_iterator};

#[derive(Copy, Clone)]
struct Mapping {
    target: i64,
    source: i64,
    len: i64,
}

fn get_seed_numbers(line: &str) -> Vec<i64> {
    if line.starts_with("seeds: ") {
        return get_series_of_ints(line);
    }
    return vec![];
}

#[cfg(test)]
mod get_seed_numbers_tests {
    use super::*;

    #[test]
    fn test_get_seed_numbers_empty() {
        assert_eq!(get_seed_numbers("").is_empty(), true);
    }

    #[test]
    fn test_get_seed_numbers_not_seed_line() {
        assert_eq!(get_seed_numbers("1 2 3 4 5").is_empty(), true);
    }

    #[test]
    fn test_get_seed_numbers_correct() {
        assert_eq!(get_seed_numbers("seeds: 79 14 55 13").len(), 4);
    }
}

fn get_mapping(line: &str) -> Option<Mapping> {
    let ints = get_series_of_ints(line);
    if ints.len() != 3 {
        return None;
    }
    return Some(Mapping { target: ints[0], source: ints[1], len: ints[2] });
}

#[cfg(test)]
mod get_mapping_tests {
    use super::*;

    #[test]
    fn test_get_mapping_empty_line() {
        assert_eq!(get_mapping("").is_none(), true);
    }

    #[test]
    fn test_get_mapping_seed_line() {
        assert_eq!(get_mapping("seeds: 79 14 55 13").is_none(), true);
    }

    #[test]
    fn test_get_mapping_map_header_line() {
        assert_eq!(get_mapping("seed-to-soil map:").is_none(), true);
    }

    #[test]
    fn test_get_mapping_map_correct_line() {
        let result = get_mapping("50 98 2");
        assert_eq!(result.is_some(), true);
        let mapping = result.unwrap();
        assert_eq!(mapping.target, 50);
        assert_eq!(mapping.source, 98);
        assert_eq!(mapping.len, 2);
    }
}

fn get_target(source: i64, mappings: &Vec<Mapping>) -> i64 {
    for mapping in mappings {
        if source >= mapping.source && source < mapping.source + mapping.len {
            return source + mapping.target - mapping.source;
        }
    }
    return source;
}

#[cfg(test)]
const FIRST_MAPPING: Mapping = Mapping {
    target: 50,
    source: 98,
    len: 2,
};

#[cfg(test)]
const SECOND_MAPPING: Mapping = Mapping {
    target: 52,
    source: 50,
    len: 48,
};

#[cfg(test)]
mod get_target_tests {
    use super::*;

    #[test]
    fn test_get_target_98() {
        let test_mappings = vec![FIRST_MAPPING, SECOND_MAPPING];
        assert_eq!(get_target(98, &test_mappings), 50);
    }

    #[test]
    fn test_get_target_99() {
        let test_mappings = vec![FIRST_MAPPING, SECOND_MAPPING];
        assert_eq!(get_target(99, &test_mappings), 51);
    }

    #[test]
    fn test_get_target_53() {
        let test_mappings = vec![FIRST_MAPPING, SECOND_MAPPING];
        assert_eq!(get_target(53, &test_mappings), 55);
    }

    #[test]
    fn test_get_target_10() {
        let test_mappings = vec![FIRST_MAPPING, SECOND_MAPPING];
        assert_eq!(get_target(10, &test_mappings), 10);
    }
}

fn get_targets(sources: Vec<i64>, mappings: &Vec<Mapping>) -> Vec<i64> {
    return sources.iter().map(|source| get_target(*source, mappings)).collect();
}

#[cfg(test)]
mod get_targets_tests {
    use super::*;

    #[test]
    fn test_get_targets() {
        let test_sources = vec![79, 14, 55, 13];
        let test_mappings = vec![FIRST_MAPPING, SECOND_MAPPING];
        let targets = get_targets(test_sources, &test_mappings);
        assert_eq!(targets.len(), 4);
        assert_eq!(targets[0], 81);
        assert_eq!(targets[1], 14);
        assert_eq!(targets[2], 57);
        assert_eq!(targets[3], 13);
    }
}

struct TargetRange {
    start: i64,
    len: i64,
    prev_start: i64,
    prev_len: i64,
    next_start: i64,
    next_len: i64,
}

fn get_target_range_for_single_mapping(source: i64, source_len: i64, mapping: Mapping) -> TargetRange {
    let mut mapping_start = source - mapping.source;
    let mut mapping_end = mapping_start + source_len;
    let mut items_before = 0;
    let mut items_after = 0;
    if mapping_start <= mapping.len && mapping_end >= 0 {
        if mapping_start < 0 {
            items_before = -mapping_start;
            mapping_start = 0;
        }
        if mapping_end > mapping.len {
            items_after = mapping_end - mapping.len;
            mapping_end = mapping.len;
        }
    } else {
        mapping_start = 0;
        mapping_end = 0;
        items_after = source_len;
    }
    return TargetRange {
        start: mapping_start + mapping.target,
        len: mapping_end - mapping_start,
        prev_start: source - mapping_start,
        prev_len: items_before,
        next_start: source + items_before + mapping_end - mapping_start,
        next_len: items_after,
    }
}

#[cfg(test)]
mod get_target_range_for_single_mapping_tests {
    use super::*;

    #[test]
    fn test_get_target_range_for_single_mapping_no_match() {
        let result = get_target_range_for_single_mapping(20, 10, Mapping { target: 40, source: 40, len: 10 });
        assert_eq!(result.len, 0);
        assert_eq!(result.prev_len, 0);
        assert_eq!(result.next_start, 20);
        assert_eq!(result.next_len, 10);
    }

    #[test]
    fn test_get_target_range_for_single_mapping_exact_match() {
        let result = get_target_range_for_single_mapping(20, 10, Mapping { target: 40, source: 20, len: 10 });
        assert_eq!(result.start, 40);
        assert_eq!(result.len, 10);
        assert_eq!(result.prev_len, 0);
        assert_eq!(result.next_len, 0);
    }

    #[test]
    fn test_get_target_range_for_single_mapping_source_enclosed() {
        let result = get_target_range_for_single_mapping(22, 8, Mapping { target: 40, source: 20, len: 10 });
        assert_eq!(result.start, 42);
        assert_eq!(result.len, 8);
        assert_eq!(result.prev_len, 0);
        assert_eq!(result.next_len, 0);
    }

    #[test]
    fn test_get_target_range_for_single_mapping_some_preceding_numbers() {
        let result = get_target_range_for_single_mapping(18, 8, Mapping { target: 40, source: 20, len: 10 });
        assert_eq!(result.start, 40);
        assert_eq!(result.len, 6);
        assert_eq!(result.prev_start, 18);
        assert_eq!(result.prev_len, 2);
        assert_eq!(result.next_len, 0);
    }

    #[test]
    fn test_get_target_range_for_single_mapping_some_following_numbers() {
        let result = get_target_range_for_single_mapping(24, 8, Mapping { target: 40, source: 20, len: 10 });
        assert_eq!(result.start, 44);
        assert_eq!(result.len, 6);
        assert_eq!(result.prev_len, 0);
        assert_eq!(result.next_start, 30);
        assert_eq!(result.next_len, 2);
    }

    #[test]
    fn test_get_target_range_for_single_mapping_some_preceding_and_following_numbers() {
        let result = get_target_range_for_single_mapping(18, 14, Mapping { target: 40, source: 20, len: 10 });
        assert_eq!(result.start, 40);
        assert_eq!(result.len, 10);
        assert_eq!(result.prev_start, 18);
        assert_eq!(result.prev_len, 2);
        assert_eq!(result.next_start, 30);
        assert_eq!(result.next_len, 2);
    }
}

fn partial_vector_copy(mappings: &Vec<Mapping>, start: usize) -> Vec<Mapping> {
    let mut mappings_copy: Vec<Mapping> = vec![];
    let mut index = start;
    let len = mappings.len();
    while index < len {
        mappings_copy.push(mappings[index]);
        index = index + 1;
    }
    return mappings_copy;
}

#[cfg(test)]
mod partial_vector_copy_tests {
    use super::*;

    #[test]
    fn test_partial_vector_copy() {
        let source = vec![FIRST_MAPPING, SECOND_MAPPING];
        let result = partial_vector_copy(&source, 1);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].source, source[1].source);
    }
}

fn get_targets_for_map(source: i64, source_len: i64, mappings: &Vec<Mapping>) -> Vec<(i64, i64)> {
    if mappings.len() == 0 {
        return vec![(source, source_len)];
    }
    let target_range = get_target_range_for_single_mapping(source, source_len, mappings[0]);
    let mappings_copy = partial_vector_copy(&mappings, 1);
    let mut targets: Vec<(i64, i64)> = vec![];
    if target_range.prev_len != 0 {
        targets.append(&mut get_targets_for_map(target_range.prev_start, target_range.prev_len, &mappings_copy));
    }
    if target_range.len != 0 {
        targets.push((target_range.start, target_range.len));
    }
    if target_range.next_len != 0 {
        targets.append(&mut get_targets_for_map(target_range.next_start, target_range.next_len, &mappings_copy));
    }
    return targets;
}

#[cfg(test)]
mod get_targets_for_map_tests {
    use super::*;

    #[test]
    fn test_get_targets_for_map() {
        let test_mappings = vec![FIRST_MAPPING, SECOND_MAPPING];
        let targets = get_targets_for_map(79, 14, &test_mappings);
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0], (81, 14));
    }

    #[test]
    fn test_get_targets_for_map_two_mappings() {
        let test_mappings = vec![FIRST_MAPPING, SECOND_MAPPING];
        let targets = get_targets_for_map(79, 20, &test_mappings);
        assert_eq!(targets.len(), 2);
        assert_eq!(targets[0], (81, 19));
        assert_eq!(targets[1], (50, 1));
    }

    #[test]
    fn test_get_targets_for_map_two_mappings_and_unmatched() {
        let test_mappings = vec![FIRST_MAPPING, SECOND_MAPPING];
        let targets = get_targets_for_map(79, 22, &test_mappings);
        assert_eq!(targets.len(), 3);
        assert_eq!(targets[0], (81, 19));
        assert_eq!(targets[1], (50, 2));
        assert_eq!(targets[2], (100, 1));
    }
}

fn condense_ranges(ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut condensed: Vec<(i64, i64)> = vec![];
    let mut sorted_ranges: Vec<(i64, i64)> = ranges.iter().copied().collect();
    sorted_ranges.sort();
    let mut index = 0;
    let mut start = 0;
    let mut len = 0;
    while index < sorted_ranges.len() {
        let (range_start, range_len) = sorted_ranges[index];
        if range_start == start + len {
            len = len + range_len;
        } else {
            if len != 0 {
                condensed.push((start, len));
            }
            start = range_start;
            len = range_len;
        }
        index = index + 1;
    }
    if len != 0 {
        condensed.push((start, len));
    }
    return condensed;
}

#[cfg(test)]
mod condense_ranges_tests {
    use super::*;

    #[test]
    fn test_condense_ranges() {
        let sources = vec![(81, 19), (50, 2), (100, 1)];
        let result = condense_ranges(sources);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], (50, 2));
        assert_eq!(result[1], (81, 20));
    }
}

fn sources_to_ranges(sources: Vec<i64>) -> Vec<(i64, i64)> {
    let mut ranges: Vec<(i64, i64)> = vec![];
    let mut index = 0;
    while index < sources.len() {
        let source_start = sources[index];
        let source_len = sources[index + 1];
        ranges.push((source_start, source_len));
        index += 2;
    }
    return ranges;
}

#[cfg(test)]
mod sources_to_ranges_tests {
    use super::*;

    #[test]
    fn test_sources_to_ranges() {
        let sources = vec![79, 14, 55, 13];
        let result = sources_to_ranges(sources);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], (79, 14));
        assert_eq!(result[1], (55, 13));
    }
}

fn get_targets_for_sources(sources: Vec<(i64, i64)>, mappings: &Vec<Mapping>) -> Vec<(i64, i64)> {
    let mut target_ranges: Vec<(i64, i64)> = vec![];
    for source in sources {
        let (source_start, source_len) = source;
        target_ranges.append(&mut get_targets_for_map(source_start, source_len, mappings));
    }
    return condense_ranges(target_ranges);
}

#[cfg(test)]
mod get_targets_for_sources_tests {
    use super::*;

    #[test]
    fn test_get_targets_for_sources() {
        let test_mappings = vec![FIRST_MAPPING, SECOND_MAPPING];
        let sources = vec![(79, 14), (55, 13)];
        let result = get_targets_for_sources(sources, &test_mappings);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], (57, 13));
        assert_eq!(result[1], (81, 14));
    }
}

fn get_seeds_and_maps(filename: &str) -> (Vec<i64>, Vec<Vec<Mapping>>) {
    let mut seeds: Vec<i64> = vec![];
    let mut maps: Vec<Vec<Mapping>> = vec![];
    let mut current_map: Vec<Mapping> = vec![];
    let lines = line_iterator(filename);
    for line_string in lines {
        let line = line_string.as_str();
        if line.starts_with("seeds: ") {
            seeds.append(&mut get_seed_numbers(line));
        } else if line.ends_with("map:") {
            if !current_map.is_empty() {
                maps.push(current_map);
            }
            current_map = vec![];
        } else if line != "" {
            current_map.push(get_mapping(line).unwrap());
        }
    }
    maps.push(current_map);
    return (seeds, maps);
}

#[cfg(test)]
mod get_seeds_and_maps_tests {
    use super::*;

    fn mapping_eq(a: &Mapping, b: &Mapping) -> bool {
        a.target == b.target && a.source == b.source && a.len == b.len
    }

    fn vec_mapping_eq(a: &Vec<Mapping>, b: &Vec<Mapping>) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut index = 0;
        while index < a.len() {
            if !mapping_eq(&a[index], &b[index]) {
                return false;
            }
            index = index + 1;
        }
        return true;
    }

    #[test]
    fn test_get_seeds_and_maps() {
        let (seeds, maps) = get_seeds_and_maps("data/day05/test.txt");
        assert_eq!(seeds.len(), 4);
        assert_eq!(seeds[0], 79);
        assert_eq!(seeds[1], 14);
        assert_eq!(seeds[2], 55);
        assert_eq!(seeds[3], 13);
        assert_eq!(maps.len(), 7);
        assert_eq!(vec_mapping_eq(&maps[0], &vec![
            Mapping { target: 50, source: 98, len: 2 },
            Mapping { target: 52, source: 50, len: 48 },
        ]), true);
        assert_eq!(vec_mapping_eq(&maps[1], &vec![
            Mapping { target: 0, source: 15, len: 37 },
            Mapping { target: 37, source: 52, len: 2 },
            Mapping { target: 39, source: 0, len: 15 },
        ]), true);
        assert_eq!(vec_mapping_eq(&maps[2], &vec![
            Mapping { target: 49, source: 53, len: 8 },
            Mapping { target: 0, source: 11, len: 42 },
            Mapping { target: 42, source: 0, len: 7 },
            Mapping { target: 57, source: 7, len: 4 },
        ]), true);
        assert_eq!(vec_mapping_eq(&maps[3], &vec![
            Mapping { target: 88, source: 18, len: 7 },
            Mapping { target: 18, source: 25, len: 70 },
        ]), true);
        assert_eq!(vec_mapping_eq(&maps[4], &vec![
            Mapping { target: 45, source: 77, len: 23 },
            Mapping { target: 81, source: 45, len: 19 },
            Mapping { target: 68, source: 64, len: 13 },
        ]), true);
        assert_eq!(vec_mapping_eq(&maps[5], &vec![
            Mapping { target: 0, source: 69, len: 1 },
            Mapping { target: 1, source: 0, len: 69 },
        ]), true);
        assert_eq!(vec_mapping_eq(&maps[6], &vec![
            Mapping { target: 60, source: 56, len: 37 },
            Mapping { target: 56, source: 93, len: 4 },
        ]), true);
    }
}

fn get_locations(seeds: Vec<i64>, maps: Vec<Vec<Mapping>>) -> Vec<i64> {
    let mut targets: Vec<i64> = seeds;
    for map in maps {
        targets = get_targets(targets, &map);
    }
    return targets;
}

#[cfg(test)]
mod get_locations_tests {
    use super::*;

    #[test]
    fn test_get_locations() {
        let (seeds, maps) = get_seeds_and_maps("data/day05/test.txt");
        let locations = get_locations(seeds, maps);
        assert_eq!(locations[0], 82);
        assert_eq!(locations[1], 43);
        assert_eq!(locations[2], 86);
        assert_eq!(locations[3], 35);
    }
}

fn get_locations_part2(seeds: Vec<i64>, maps: Vec<Vec<Mapping>>) -> Vec<i64> {
    let mut targets: Vec<(i64, i64)> = sources_to_ranges(seeds);
    for map in maps {
        targets = get_targets_for_sources(targets, &map);
    }
    let mut result: Vec<i64> = vec![];
    for target in targets {
        let (target_start, _) = target;
        result.push(target_start);
    }
    return result;
}

#[cfg(test)]
mod get_locations_part2_tests {
    use super::*;

    #[test]
    fn test_get_locations_part2() {
        let (seeds, maps) = get_seeds_and_maps("data/day05/test.txt");
        let locations = get_locations_part2(seeds, maps);
        assert_eq!(locations.len(), 4);
        assert_eq!(locations[0], 46);
        assert_eq!(locations[1], 82);
        assert_eq!(locations[2], 86);
        assert_eq!(locations[3], 94);
    }
}

fn min_value(values: Vec<i64>) -> i64 {
    let mut sorted_values = values;
    sorted_values.sort();
    return sorted_values[0];
}

#[cfg(test)]
mod min_value_tests {
    use super::*;

    #[test]
    fn test_min_value() {
        assert_eq!(min_value(vec![82, 43, 86, 35]), 35);
    }
}

pub fn part1() {
    let (seeds, maps) = get_seeds_and_maps("data/day05/input.txt");
    println!("Day 5 Part 1 result: {}", min_value(get_locations(seeds, maps)));
}

pub fn part2() {
    let (seeds, maps) = get_seeds_and_maps("data/day05/input.txt");
    println!("Day 5 Part 2 result: {}", min_value(get_locations_part2(seeds, maps)));
}
