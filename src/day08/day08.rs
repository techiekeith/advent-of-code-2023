use std::collections::HashMap;
use crate::aoc_common::lib::line_iterator;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Node {
    id: i16,
    left: i16,
    right: i16,
}

const AAA_NODE: i16 = 0;
const ZZZ_NODE: i16 = 17575; // (26 * 26 * 26) - 1

fn node_id(from_string: &str) -> Option<i16> {
    if from_string.len() != 3 {
        return None;
    }
    let chars = from_string.chars();
    let mut id: i16 = 0;
    for chr in chars {
        if chr < 'A' || chr > 'Z' {
            return None;
        }
        id = id * 26 + chr as i16 - 'A' as i16;
    }
    return Some(id);
}

#[cfg(test)]
mod node_id_tests {
    use super::*;

    #[test]
    fn test_node_id_aaa() {
        assert_eq!(node_id("AAA"), Some(AAA_NODE));
    }

    #[test]
    fn test_node_id_aab() {
        assert_eq!(node_id("AAB"), Some(1));
    }

    #[test]
    fn test_node_id_baa() {
        assert_eq!(node_id("BAA"), Some(26 * 26));
    }

    #[test]
    fn test_node_id_zzz() {
        assert_eq!(node_id("ZZZ"), Some(ZZZ_NODE));
    }
}

fn parse_node(line: &str) -> Option<Node> {
    if line.len() != 16 {
        return None;
    }
    let id = node_id(&line[0..3]);
    let left = node_id(&line[7..10]);
    let right = node_id(&line[12..15]);
    return Some(Node { id: id.unwrap(), left: left.unwrap(), right: right.unwrap() })
}

#[cfg(test)]
mod parse_node_tests {
    use super::*;

    #[test]
    fn parse_aaa_bbb_ccc() {
        assert_eq!(parse_node("AAA = (BBB, CCC)"), Some(Node {
            id: AAA_NODE,
            left: node_id("BBB").unwrap(),
            right: node_id("CCC").unwrap(),
        }))
    }
}

const Z: i16 = 25;

fn is_unfinished(node_id: i16, any_z: bool) -> bool {
    match any_z {
        false => node_id != ZZZ_NODE,
        true => node_id % 26 != Z,
    }
}

#[cfg(test)]
mod is_unfinished_tests {
    use super::*;

    #[test]
    fn test_is_unfinished_only_zzz() {
        assert_eq!(is_unfinished(node_id("ATQ").unwrap(), false), true);
        assert_eq!(is_unfinished(node_id("AAZ").unwrap(), false), true);
        assert_eq!(is_unfinished(ZZZ_NODE, false), false);
    }

    #[test]
    fn test_is_unfinished_any_z() {
        assert_eq!(is_unfinished(node_id("ATQ").unwrap(), true), true);
        assert_eq!(is_unfinished(node_id("AAZ").unwrap(), true), false);
        assert_eq!(is_unfinished(ZZZ_NODE, false), false);
    }
}

fn count_steps(path: &str, nodes: &HashMap<i16, Node>, path_start: usize, start_node: i16, any_z: bool) -> Option<i64> {
    let path_chars: Vec<char> = path.chars().collect();
    let mut path_index = path_start;
    let mut current_node = start_node;
    let mut step_count = 0;
    while is_unfinished(current_node, any_z) || step_count == 0 {
        let node = nodes.get(&current_node);
        if node.is_none() {
            return None;
        }
        let dir = path_chars[path_index];
        match dir {
            'L' => current_node = node.unwrap().left,
            'R' => current_node = node.unwrap().right,
            _ => return None,
        }
        step_count += 1;
        path_index += 1;
        if path_index == path_chars.len() {
            path_index = 0;
        }
    }
    return Some(step_count);
}

#[cfg(test)]
mod count_steps_tests {
    use super::*;

    #[test]
    fn test_count_steps_test1() {
        let path = "RL";
        let mut nodes: HashMap<i16, Node> = HashMap::new();
        let bbb_node = node_id("BBB").unwrap();
        let ccc_node = node_id("CCC").unwrap();
        let ggg_node = node_id("GGG").unwrap();
        nodes.insert(AAA_NODE, Node {
            id: AAA_NODE,
            left: bbb_node,
            right: ccc_node,
        });
        nodes.insert(ccc_node, Node {
            id: ccc_node,
            left: ZZZ_NODE,
            right: ggg_node,
        });
        assert_eq!(count_steps(path, &nodes, 0, AAA_NODE, false), Some(2));
    }

    #[test]
    fn test_count_steps_test2() {
        let path = "LLR";
        let mut nodes: HashMap<i16, Node> = HashMap::new();
        let bbb_node = node_id("BBB").unwrap();
        nodes.insert(AAA_NODE, Node {
            id: AAA_NODE,
            left: bbb_node,
            right: bbb_node,
        });
        nodes.insert(bbb_node, Node {
            id: bbb_node,
            left: AAA_NODE,
            right: ZZZ_NODE,
        });
        assert_eq!(count_steps(path, &nodes, 0, AAA_NODE, false), Some(6));
    }
}

fn read_file(filename: &str) -> (String, HashMap<i16, Node>) {
    let mut lines = line_iterator(filename);
    let path = lines.next().unwrap();
    let mut nodes: HashMap<i16, Node> = HashMap::new();
    for line in lines {
        let maybe_node = parse_node(line.as_str());
        if maybe_node.is_some() {
            let node = maybe_node.unwrap();
            nodes.insert(node.id, node);
        }
    }
    return (path, nodes);
}

#[cfg(test)]
mod read_file_tests {
    use super::*;

    #[test]
    fn test_read_file_test1() {
        let (path, nodes) = read_file("data/day08/test1.txt");
        assert_eq!(path.as_str(), "RL");
        assert_eq!(nodes.len(), 7);
        assert_eq!(nodes.get(&AAA_NODE), Some(&Node {
            id: AAA_NODE,
            left: node_id("BBB").unwrap(),
            right: node_id("CCC").unwrap(),
        }));
        assert_eq!(nodes.get(&ZZZ_NODE), Some(&Node {
            id: ZZZ_NODE,
            left: ZZZ_NODE,
            right: ZZZ_NODE,
        }));
    }

    #[test]
    fn test_read_file_test2() {
        let (path, nodes) = read_file("data/day08/test2.txt");
        assert_eq!(path.as_str(), "LLR");
        assert_eq!(nodes.len(), 3);
        assert_eq!(nodes.get(&AAA_NODE), Some(&Node {
            id: AAA_NODE,
            left: node_id("BBB").unwrap(),
            right: node_id("BBB").unwrap(),
        }));
        assert_eq!(nodes.get(&node_id("BBB").unwrap()), Some(&Node {
            id: node_id("BBB").unwrap(),
            left: AAA_NODE,
            right: ZZZ_NODE,
        }));
        assert_eq!(nodes.get(&ZZZ_NODE), Some(&Node {
            id: ZZZ_NODE,
            left: ZZZ_NODE,
            right: ZZZ_NODE,
        }));
    }
}

fn count_steps_for_part1(filename: &str) -> i64 {
    let (path, nodes) = read_file(filename);
    return count_steps(path.as_str(), &nodes, 0, AAA_NODE, false).unwrap();
}

#[cfg(test)]
mod count_steps_for_part1_tests {
    use super::*;

    #[test]
    fn test_count_steps_for_part1_using_test1() {
        assert_eq!(count_steps_for_part1("data/day08/test1.txt"), 2);
    }

    #[test]
    fn test_count_steps_for_part1_using_test2() {
        assert_eq!(count_steps_for_part1("data/day08/test2.txt"), 6);
    }
}

fn find_starting_nodes(nodes: &HashMap<i16, Node>) -> Vec<i16> {
    return nodes.keys().filter_map(|a| match a % 26 { 0 => Some(a), _ => None }).map(|a| *a).collect();
}

#[cfg(test)]
mod find_starting_nodes_tests {
    use super::*;

    #[test]
    fn test_find_starting_nodes() {
        let nna = node_id("NNA").unwrap();
        let tta = node_id("TTA").unwrap();
        let (_path, nodes) = read_file("data/day08/test3.txt");
        let mut starting_nodes = find_starting_nodes(&nodes);
        starting_nodes.sort();
        assert_eq!(starting_nodes.len(), 2);
        assert_eq!(starting_nodes[0], nna);
        assert_eq!(starting_nodes[1], tta);
    }
}

fn factors(initial_number: i64) -> HashMap<i64, i64> {
    let mut factors: HashMap<i64, i64> = HashMap::new();
    let root = (initial_number as f64).sqrt() as i64;
    let mut number = initial_number;
    let mut index = 2;
    while index <= root {
        let mut count = 0;
        while number % index == 0 {
            number = number / index;
            count = count + 1;
        }
        if count != 0 {
            factors.insert(index, count);
        }
        index = index + 1;
    }
    if number > 1 {
        factors.insert(number, 1);
    }
    return factors;
}

#[cfg(test)]
mod factors_tests {
    use super::*;

    #[test]
    fn test_factors() {
        let fac = factors(140);
        assert_eq!(fac.len(), 3);
        assert_eq!(fac.get(&2).unwrap(), &2);
        assert_eq!(fac.get(&5).unwrap(), &1);
        assert_eq!(fac.get(&7).unwrap(), &1);
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    let mut common_factors: HashMap<i64, i64> = HashMap::new();
    let factors_in_a = factors(a);
    let factors_in_b = factors(b);
    for (a_factor, a_factor_count) in factors_in_a {
        let b_factor_value_maybe = factors_in_b.get(&a_factor);
        if b_factor_value_maybe.is_none() {
            common_factors.insert(a_factor, a_factor_count);
        } else {
            let b_factor_count = *b_factor_value_maybe.unwrap();
            if a_factor_count > b_factor_count {
                common_factors.insert(a_factor, a_factor_count);
            } else {
                common_factors.insert(a_factor, b_factor_count);
            }
        }
    }
    for (b_factor, b_factor_count) in factors_in_b {
        let existing_value = common_factors.get(&b_factor);
        if existing_value.is_none() {
            common_factors.insert(b_factor, b_factor_count);
        }
    }
    let mut result = 1;
    for (factor, count) in common_factors {
        result = result * factor * count;
    }
    return result;
}

#[cfg(test)]
mod lcm_tests {
    use super::*;

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(6, 4), 12);
    }
}

fn count_steps_for_part2(filename: &str) -> i64 {
    let (path, nodes) = read_file(filename);
    let mut starting_nodes = find_starting_nodes(&nodes);
    starting_nodes.sort();
    let mut steps: Vec<i64> = vec![];
    for starting_node in find_starting_nodes(&nodes) {
        steps.push(count_steps(path.as_str(), &nodes, 0, starting_node, true).unwrap());
    }
    let mut min_steps = 1;
    for count in steps {
        min_steps = lcm(min_steps, count);
    }
    return min_steps;
}

#[cfg(test)]
mod count_steps_for_part2_tests {
    use super::*;

    #[test]
    fn test_count_steps_for_part2_test3() {
        assert_eq!(count_steps_for_part2("data/day08/test3.txt"), 6);
    }
}

pub fn part1() {
    println!("Day 8 Part 1 result: {}", count_steps_for_part1("data/day08/input.txt"));
}

pub fn part2() {
    println!("Day 8 Part 2 result: {}", count_steps_for_part2("data/day08/input.txt"));
}
