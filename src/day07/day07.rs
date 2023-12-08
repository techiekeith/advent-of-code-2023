use std::cmp::Ordering;
use std::collections::HashMap;
use crate::aoc_common::lib::line_iterator;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand {
    cards: [i8; 5],
    bid: i64,
}

const JOKER: i8 = 0;
const TEN: i8 = 10;
const JACK: i8 = 11;
const QUEEN: i8 = 12;
const KING: i8 = 13;
const ACE: i8 = 14;

fn get_hand(line: &str, jokers: bool) -> Option<Hand> {
    let parts: Vec<&str> = line.split(' ').collect();
    if parts.len() != 2 {
        return None;
    }
    let cards: Vec<char> = parts[0].chars().collect();
    if cards.len() != 5 {
        return None;
    }
    let bid = parts[1].parse::<i64>();
    if bid.is_err() {
        return None;
    }
    let mut hand: Hand = Hand { cards: [0; 5], bid: bid.unwrap() };
    let mut index = 0;
    for card in cards {
        let value;
        match card {
            '2'..='9' => value = card as i8 - '0' as i8,
            'T' => value = TEN,
            'J' => value = match jokers { true => JOKER, false => JACK },
            'Q' => value = QUEEN,
            'K' => value = KING,
            'A' => value = ACE,
            _ => return None
        }
        hand.cards[index] = value;
        index += 1;
    }
    return Some(hand);
}

#[cfg(test)]
mod get_hand_tests {
    use super::*;

    #[test]
    fn test_get_hand_empty() {
        assert_eq!(get_hand("", false).is_none(), true);
    }

    #[test]
    fn test_get_hand_malformed_line_missing_bid() {
        assert_eq!(get_hand("AAAAA", false).is_none(), true);
    }

    #[test]
    fn test_get_hand_malformed_line_too_many_args() {
        assert_eq!(get_hand("AAAAA 123 x", false).is_none(), true);
    }

    #[test]
    fn test_get_hand_malformed_line_too_few_cards() {
        assert_eq!(get_hand("AAAA 123", false).is_none(), true);
    }

    #[test]
    fn test_get_hand_malformed_line_too_many_cards() {
        assert_eq!(get_hand("AAAAAA 123", false).is_none(), true);
    }

    #[test]
    fn test_get_hand_malformed_line_bid_not_a_number() {
        assert_eq!(get_hand("AAAAA 123x", false).is_none(), true);
    }

    #[test]
    fn test_get_hand_malformed_line_invalid_card() {
        assert_eq!(get_hand("12345 123", false).is_none(), true);
    }

    #[test]
    fn test_get_hand_valid_bid_number_cards() {
        assert_eq!(get_hand("24689 123", false).unwrap(), Hand {
            cards: [ 2, 4, 6, 8, 9 ],
            bid: 123,
        });
    }

    #[test]
    fn test_get_hand_valid_bid_picture_cards() {
        assert_eq!(get_hand("TJQKA 123", false).unwrap(), Hand {
            cards: [ TEN, JACK, QUEEN, KING, ACE ],
            bid: 123,
        });
    }

    #[test]
    fn test_get_hand_valid_bid_picture_cards_with_joker() {
        assert_eq!(get_hand("TJQKA 123", true).unwrap(), Hand {
            cards: [ TEN, JOKER, QUEEN, KING, ACE ],
            bid: 123,
        });
    }
}

fn count_cards(cards: [i8; 5]) -> HashMap<i8, i8> {
    let mut counts: HashMap<i8, i8> = HashMap::new();
    for index in 0..5 {
        let card = cards[index];
        let option = counts.get(&card);
        if option.is_none() {
            counts.insert(card, 1);
        } else {
            counts.insert(card, option.unwrap() + 1);
        }
    }
    return counts;
}

#[cfg(test)]
mod count_cards_tests {
    use super::*;

    #[test]
    fn test_count_cards_five_of_a_kind() {
        let map = count_cards([ ACE, ACE, ACE, ACE, ACE ]);
        assert_eq!(map.get(&ACE), Some(&5));
    }

    #[test]
    fn test_count_cards_four_of_a_kind() {
        let map = count_cards([ ACE, ACE, 8, ACE, ACE ]);
        assert_eq!(map.get(&ACE), Some(&4));
        assert_eq!(map.get(&8), Some(&1));
    }

    #[test]
    fn test_count_cards_full_house() {
        let map = count_cards([ 2, 3, 3, 3, 2 ]);
        assert_eq!(map.get(&2), Some(&2));
        assert_eq!(map.get(&3), Some(&3));
    }

    #[test]
    fn test_count_cards_three_of_a_kind() {
        let map = count_cards([ TEN, TEN, TEN, 9, 8 ]);
        assert_eq!(map.get(&10), Some(&3));
        assert_eq!(map.get(&9), Some(&1));
        assert_eq!(map.get(&8), Some(&1));
    }

    #[test]
    fn test_count_cards_two_pair() {
        let map = count_cards([ 2, 3, 4, 3, 2 ]);
        assert_eq!(map.get(&2), Some(&2));
        assert_eq!(map.get(&3), Some(&2));
        assert_eq!(map.get(&4), Some(&1));
    }

    #[test]
    fn test_count_cards_one_pair() {
        let map = count_cards([ ACE, 2, 3, ACE, 4 ]);
        assert_eq!(map.get(&ACE), Some(&2));
        assert_eq!(map.get(&2), Some(&1));
        assert_eq!(map.get(&3), Some(&1));
        assert_eq!(map.get(&4), Some(&1));
    }

    #[test]
    fn test_count_cards_high_card() {
        let map = count_cards([ 2, 3, 4, 5, 6 ]);
        assert_eq!(map.get(&2), Some(&1));
        assert_eq!(map.get(&3), Some(&1));
        assert_eq!(map.get(&4), Some(&1));
        assert_eq!(map.get(&5), Some(&1));
        assert_eq!(map.get(&6), Some(&1));
    }
}

const HAND_TYPE_HIGH_CARD: i8 = 0;
const HAND_TYPE_ONE_PAIR: i8 = 1;
const HAND_TYPE_TWO_PAIR: i8 = 2;
const HAND_TYPE_THREE_OF_A_KIND: i8 = 3;
const HAND_TYPE_FULL_HOUSE: i8 = 4;
const HAND_TYPE_FOUR_OF_A_KIND: i8 = 5;
const HAND_TYPE_FIVE_OF_A_KIND: i8 = 6;

fn get_hand_type(cards: [i8; 5]) -> i8 {
    let mut map = count_cards(cards);
    let jokers = map.remove(&JOKER).unwrap_or(0);
    match map.len() {
        0 | 1 => HAND_TYPE_FIVE_OF_A_KIND,
        2 => {
            let value = map.values().reduce(std::cmp::max).unwrap();
            match *value + jokers {
                4 => HAND_TYPE_FOUR_OF_A_KIND,
                _ => HAND_TYPE_FULL_HOUSE,
            }
        },
        3 => {
            let value = map.values().reduce(std::cmp::max).unwrap();
            match *value + jokers {
                3 => HAND_TYPE_THREE_OF_A_KIND,
                _ => HAND_TYPE_TWO_PAIR,
            }
        },
        4 => HAND_TYPE_ONE_PAIR,
        _ => HAND_TYPE_HIGH_CARD,
    }
}

#[cfg(test)]
mod get_hand_type_tests {
    use super::*;

    #[test]
    fn test_get_hand_type_five_of_a_kind() {
        assert_eq!(get_hand_type([ ACE, ACE, ACE, ACE, ACE ]), HAND_TYPE_FIVE_OF_A_KIND);
    }

    #[test]
    fn test_get_hand_type_four_of_a_kind() {
        assert_eq!(get_hand_type([ ACE, ACE, 8, ACE, ACE ]), HAND_TYPE_FOUR_OF_A_KIND);
    }

    #[test]
    fn test_get_hand_type_full_house() {
        assert_eq!(get_hand_type([ 2, 3, 3, 3, 2 ]), HAND_TYPE_FULL_HOUSE);
    }

    #[test]
    fn test_get_hand_type_three_of_a_kind() {
        assert_eq!(get_hand_type([ TEN, TEN, TEN, 9, 8 ]), HAND_TYPE_THREE_OF_A_KIND);
    }

    #[test]
    fn test_get_hand_type_two_pair() {
        assert_eq!(get_hand_type([ 2, 3, 4, 3, 2 ]), HAND_TYPE_TWO_PAIR);
    }

    #[test]
    fn test_get_hand_type_one_pair() {
        assert_eq!(get_hand_type([ ACE, 2, 3, ACE, 4 ]), HAND_TYPE_ONE_PAIR);
    }

    #[test]
    fn test_get_hand_type_high_card() {
        assert_eq!(get_hand_type([ 2, 3, 4, 5, 6 ]), HAND_TYPE_HIGH_CARD);
    }

    #[test]
    fn test_get_hand_type_five_of_a_kind_with_all_jokers() {
        assert_eq!(get_hand_type([ JOKER, JOKER, JOKER, JOKER, JOKER ]), HAND_TYPE_FIVE_OF_A_KIND);
    }

    #[test]
    fn test_get_hand_type_four_of_a_kind_with_joker() {
        assert_eq!(get_hand_type([ TEN, 5, 5, JOKER, 5 ]), HAND_TYPE_FOUR_OF_A_KIND);
        assert_eq!(get_hand_type([ KING, TEN, JOKER, JOKER, TEN ]), HAND_TYPE_FOUR_OF_A_KIND);
        assert_eq!(get_hand_type([ QUEEN, QUEEN, QUEEN, JOKER, ACE ]), HAND_TYPE_FOUR_OF_A_KIND);
    }

    #[test]
    fn test_get_hand_type_full_house_with_joker() {
        assert_eq!(get_hand_type([ TEN, 5, TEN, JOKER, 5 ]), HAND_TYPE_FULL_HOUSE);
    }

    #[test]
    fn test_get_hand_type_three_of_a_kind_with_joker() {
        assert_eq!(get_hand_type([ 9, 5, TEN, JOKER, 5 ]), HAND_TYPE_THREE_OF_A_KIND);
    }

    #[test]
    fn test_get_hand_type_one_pair_with_joker() {
        assert_eq!(get_hand_type([ 9, 8, 7, JOKER, 5 ]), HAND_TYPE_ONE_PAIR);
    }
}

fn compare_hands_of_same_type(a: [i8; 5], b: [i8; 5]) -> Ordering {
    for index in 0..5 {
        let compare_card = a[index] - b[index];
        match compare_card {
            d if d > 0 => return Ordering::Greater,
            d if d < 0 => return Ordering::Less,
            _ => {}
        }
    }
    return Ordering::Equal;
}

#[cfg(test)]
mod compare_hands_of_same_type_tests {
    use super::*;

    #[test]
    fn test_compare_hands_of_same_type_all_cards_same() {
        assert_eq!(compare_hands_of_same_type([2, 3, 4, 5, 6], [2, 3, 4, 5, 6]), Ordering::Equal);
    }

    #[test]
    fn test_compare_hands_of_same_type_first_hand_has_higher_first_card() {
        assert_eq!(compare_hands_of_same_type([3, 4, 5, 6, 7], [2, 4, 5, 6, 7]), Ordering::Greater);
    }

    #[test]
    fn test_compare_hands_of_same_type_second_hand_has_higher_first_card() {
        assert_eq!(compare_hands_of_same_type([2, 4, 5, 6, 7], [3, 4, 5, 6, 7]), Ordering::Less);
    }

    #[test]
    fn test_compare_hands_of_same_type_first_hand_has_higher_last_card() {
        assert_eq!(compare_hands_of_same_type([3, 4, 5, 6, 8], [2, 4, 5, 6, 7]), Ordering::Greater);
    }

    #[test]
    fn test_compare_hands_of_same_type_second_hand_has_higher_last_card() {
        assert_eq!(compare_hands_of_same_type([2, 4, 5, 6, 7], [2, 4, 5, 6, 8]), Ordering::Less);
    }

    #[test]
    fn test_compare_hands_of_same_type_jokers_are_weaker() {
        assert_eq!(compare_hands_of_same_type([JOKER, 2, 2, 2, 3], [2, 2, 2, JOKER, 3]), Ordering::Less);
    }
}

fn compare_hands(a: [i8; 5], b: [i8; 5]) -> Ordering {
    let compare_hand_types = get_hand_type(a) - get_hand_type(b);
    match compare_hand_types {
        d if d > 0 => Ordering::Greater,
        d if d < 0 => Ordering::Less,
        _ => compare_hands_of_same_type(a, b)
    }
}

#[cfg(test)]
mod compare_hands_tests {
    use super::*;

    #[test]
    fn test_compare_hands_all_cards_same() {
        assert_eq!(compare_hands([2, 3, 4, 5, 6], [2, 3, 4, 5, 6]), Ordering::Equal);
    }

    #[test]
    fn test_compare_hands_full_house_in_first_hand_beats_two_pair_in_second_hand() {
        assert_eq!(compare_hands([2, 2, 2, 7, 7], [2, 2, 5, 5, 8]), Ordering::Greater);
    }

    #[test]
    fn test_compare_hands_full_house_in_second_hand_beats_two_pair_in_first_hand() {
        assert_eq!(compare_hands([2, 2, 3, 7, 7], [2, 2, 5, 5, 5]), Ordering::Less);
    }

    #[test]
    fn test_compare_hands_second_hand_has_higher_last_card() {
        assert_eq!(compare_hands([2, 4, 5, 6, 7], [2, 4, 5, 6, 8]), Ordering::Less);
    }
}

fn get_cards_by_rank(filename: &str, jokers: bool) -> Vec<Hand> {
    let lines = line_iterator(filename);
    let mut hands: Vec<Hand> = vec![];
    for line in lines {
        hands.push(get_hand(line.as_str(), jokers).unwrap());
    }
    hands.sort_by(|a, b| compare_hands(a.cards, b.cards));
    return hands;
}

#[cfg(test)]
mod get_cards_by_rank_tests {
    use super::*;

    #[test]
    fn test_get_cards_by_rank() {
        let hands = get_cards_by_rank("data/day07/test.txt", false);
        assert_eq!(hands[0], Hand { cards: [3, 2, TEN, 3, KING], bid: 765 });
        assert_eq!(hands[1], Hand { cards: [KING, TEN, JACK, JACK, TEN], bid: 220 });
        assert_eq!(hands[2], Hand { cards: [KING, KING, 6, 7, 7], bid: 28 });
        assert_eq!(hands[3], Hand { cards: [TEN, 5, 5, JACK, 5], bid: 684 });
        assert_eq!(hands[4], Hand { cards: [QUEEN, QUEEN, QUEEN, JACK, ACE], bid: 483 });
    }

    #[test]
    fn test_get_cards_by_rank_with_jokers() {
        let hands = get_cards_by_rank("data/day07/test.txt", true);
        assert_eq!(hands[0], Hand { cards: [3, 2, TEN, 3, KING], bid: 765 });
        assert_eq!(hands[1], Hand { cards: [KING, KING, 6, 7, 7], bid: 28 });
        assert_eq!(hands[2], Hand { cards: [TEN, 5, 5, JOKER, 5], bid: 684 });
        assert_eq!(hands[3], Hand { cards: [QUEEN, QUEEN, QUEEN, JOKER, ACE], bid: 483 });
        assert_eq!(hands[4], Hand { cards: [KING, TEN, JOKER, JOKER, TEN], bid: 220 });
    }
}

fn get_total_winnings(ranked_hands: Vec<Hand>) -> i64 {
    let mut index = 1;
    let mut total = 0;
    for hand in ranked_hands {
        total += index * hand.bid;
        index += 1;
    }
    return total;
}

#[cfg(test)]
mod get_total_winnings_tests {
    use super::*;

    #[test]
    fn test_get_total_winnings() {
        let hands = vec![
            Hand { cards: [3, 2, TEN, 3, KING], bid: 765 },
            Hand { cards: [KING, TEN, JACK, JACK, TEN], bid: 220 },
            Hand { cards: [KING, KING, 6, 7, 7], bid: 28 },
            Hand { cards: [TEN, 5, 5, JACK, 5], bid: 684 },
            Hand { cards: [QUEEN, QUEEN, QUEEN, JACK, ACE], bid: 483 },
        ];
        assert_eq!(get_total_winnings(hands), 6440);
    }

    #[test]
    fn test_get_total_winnings_with_jokers() {
        let hands = vec![
            Hand { cards: [3, 2, TEN, 3, KING], bid: 765 },
            Hand { cards: [KING, KING, 6, 7, 7], bid: 28 },
            Hand { cards: [TEN, 5, 5, JOKER, 5], bid: 684 },
            Hand { cards: [QUEEN, QUEEN, QUEEN, JOKER, ACE], bid: 483 },
            Hand { cards: [KING, TEN, JOKER, JOKER, TEN], bid: 220 },
        ];
        assert_eq!(get_total_winnings(hands), 5905);
    }
}

pub fn part1() {
    println!("Day 7 Part 1 result: {}", get_total_winnings(get_cards_by_rank("data/day07/input.txt", false)));
}

pub fn part2() {
    println!("Day 7 Part 2 result: {}", get_total_winnings(get_cards_by_rank("data/day07/input.txt", true)));
}
