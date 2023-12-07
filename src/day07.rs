use std::cmp::Ordering;
use itertools::Itertools;
use nom::character::complete;
use nom::character::complete::{alphanumeric1, space1};
use nom::{Compare, Parser};
use nom::error::Error;
use nom::sequence::{pair, terminated};
use crate::helpers;

pub fn part_1() -> i64 {
    read_from("src/input/day07.txt")
}

pub fn part_2() -> i64 {
    read_from_v2("src/input/day07.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let ordered: Vec<(&str, i64, Strength)> = sample.iter().map(|line| parse_hand(line)).map(|h| get_strength(h)).sorted_by(|a: &(&str, i64, Strength), b: &(&str, i64, Strength)| {
        let ordering = a.2.cmp(&b.2);
        if ordering == Ordering::Equal {
            a.0.to_string().chars().enumerate().find_map(|(idx, card)| {
                let binding = b.0.chars().nth(idx).unwrap().to_string();
                let c2: &str = binding.as_str();
                let c2o = ORDER.iter().find_position(|&&o| o == c2).map(|(a, _)| a).unwrap();
                let card2o = ORDER.iter().find_position(|&&o| o == card.to_string().as_str()).map(|(a, _)| a).unwrap();
                if c2o == card2o {
                    None
                } else {
                    let order = c2o.cmp(&card2o);
                    Some(order)
                }
            }).unwrap_or(Ordering::Equal)
        } else {
            ordering
        }
    }).collect();

    let init = 0i64;
    let res = ordered.iter().enumerate().fold(init, |a, (idx, (_, bid, _))| a + bid * (idx as i64 + 1));
    res
}

fn parse_hand(input: &str) -> (&str, i64) {
    let (_, res): (&str, (&str, i64)) = pair(terminated(alphanumeric1::<&str, Error<&str>>, space1), complete::i64).parse(input).unwrap();
    res
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum Strength {
    High = 1,
    Pair = 2,
    TwoPair = 3,
    Three = 4,
    Full = 5,
    Four = 6,
    Five = 7,
}


fn get_strength((hand, bid): (&str, i64)) -> (&str, i64, Strength) {
    let cards: Vec<(char, i64)> = hand.to_string().chars().into_group_map_by(|&card| card).into_iter().map(|(key, group)| (key, group.len() as i64)).collect();
    let cards_value: Vec<(char, i64)> = cards.iter().sorted_by(|(_, count_a), (_, count_b)| count_b.cmp(count_a)).copied().collect();

    let mut strength = match cards_value.get(0).unwrap().1 {
        5 => Strength::Five,
        4 => Strength::Four,
        3 => Strength::Three,
        2 => Strength::Pair,
        _ => Strength::High,
    };

    if strength == Strength::Three && cards_value.get(1).unwrap().1 == 2 {
        strength = Strength::Full;
    } else if strength == Strength::Pair && cards_value.get(1).unwrap().1 == 2 {
        strength = Strength::TwoPair;
    }
    (hand, bid, strength)
}

fn get_strength_v2((hand, bid): (&str, i64)) -> (&str, i64, Strength) {
    let joker: i64 = (*hand).to_string().chars().filter(|&c| c == 'J').count() as i64;
    let cards: Vec<(char, i64)> = hand.to_string().chars().into_group_map_by(|&card| card).into_iter().map(|(key, group)| (key, group.len() as i64)).collect();
    let cards_value: Vec<(char, i64)> = cards.iter().sorted_by(|(_, count_a), (_, count_b)| count_b.cmp(count_a)).copied().collect();

    let (card, base_strength) = cards_value.get(0).unwrap();
    let strength = match base_strength {
        5 => Strength::Five,
        4 => Strength::Four,
        3 => Strength::Three,
        2 => Strength::Pair,
        _ => Strength::High,
    };

    let with_joker = match strength {
        Strength::Five => Strength::Five,
        Strength::Four => {
            if *card == 'J' || joker == 1 {
                Strength::Five
            } else {
                Strength::Four
            }
        }
        Strength::Three => {
            if *card == 'J' {
                if cards_value.get(1).unwrap().1 == 2 {
                    Strength::Five
                } else {
                    Strength::Four
                }
            } else if joker == 2 {
                Strength::Five
            } else if joker == 1 {
                Strength::Four
            } else if cards_value.get(1).unwrap().1 == 2 {
                Strength::Full
            } else {
                Strength::Three
            }
        }
        Strength::Pair => {
            if *card == 'J' {
                if cards_value.get(1).unwrap().1 == 2 {
                    Strength::Four
                } else {
                    Strength::Three
                }
            } else if joker == 2 {
                Strength::Four
            } else if joker == 1 {
                if cards_value.get(1).unwrap().1 == 2 {
                    Strength::Full
                } else {
                    Strength::Three
                }
            } else if cards_value.get(1).unwrap().1 == 2 {
                Strength::TwoPair
            } else {
                Strength::Pair
            }
        }
        Strength::High => if joker == 1 { Strength::Pair } else { Strength::High }
        e => e
    };
    (hand, bid, with_joker)
}

const ORDER: [&str; 13] = ["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"];
const ORDER_2: [&str; 13] = ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J"];

// FIXME: unused
type Hand<'a> = (&'a str, i64, Strength);
// FIXME: unused
trait ByStrength {
    fn sort_by_strength(&self, comp: Hand) -> Ordering;
}

// FIXME: unused
impl ByStrength for Hand<'_> {
    fn sort_by_strength(&self, hand: Hand<'_>) -> Ordering {
        let ordering = self.2.cmp(&hand.2);
        if ordering == Ordering::Equal {
            self.0.to_string().chars().enumerate().find_map(|(idx, card)| {
                let binding = hand.0.chars().nth(idx).unwrap().to_string();
                let c2: &str = binding.as_str();
                let c2o = ORDER.iter().find_position(|&&o| o == c2).map(|(a, _)| a).unwrap();
                let card2o = ORDER.iter().find_position(|&&o| o == card.to_string().as_str()).map(|(a, _)| a).unwrap();
                if c2o == card2o {
                    None
                } else {
                    let order = c2o.cmp(&card2o);
                    Some(order)
                }
            }).unwrap_or(Ordering::Equal)
        } else {
            ordering
        }
    }
}


fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let ordered: Vec<(&str, i64, Strength)> = sample.iter().map(|line| parse_hand(line)).map(|h| get_strength_v2(h)).sorted_by(|a: &(&str, i64, Strength), b: &(&str, i64, Strength)| {
        let ordering = a.2.cmp(&b.2);
        if ordering == Ordering::Equal {
            a.0.to_string().chars().enumerate().find_map(|(idx, card)| {
                let binding = b.0.chars().nth(idx).unwrap().to_string();
                let c2: &str = binding.as_str();
                let c2o = ORDER_2.iter().find_position(|&&o| o == c2).map(|(a, _)| a).unwrap();
                let card2o = ORDER_2.iter().find_position(|&&o| o == card.to_string().as_str()).map(|(a, _)| a).unwrap();
                if c2o == card2o {
                    None
                } else {
                    let order = c2o.cmp(&card2o);
                    Some(order)
                }
            }).unwrap_or(Ordering::Equal)
        } else {
            ordering
        }
    }).collect();

    let res = ordered.iter().enumerate().fold(0i64, |a, (idx, (_, bid, _))| {
        let deref_bid = *bid;
        let total_bid = deref_bid * (idx as i64 + 1);
        a + total_bid
    });
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 251106089);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample07.txt");
        assert_eq!(res, 6440);
    }

    #[test]
    fn test_read_fromd() {
        let res = read_from("src/input/sample07d.txt");
        assert_eq!(res, 3542);
    }

    #[test]
    fn test_get_strength() {
        let res = get_strength(("KK677", 28));
        assert_eq!(res.2, Strength::TwoPair);
        assert_eq!(get_strength(("KKK77", 28)).2, Strength::Full);
        assert_eq!(get_strength(("KKKK7", 28)).2, Strength::Four);
        assert_eq!(get_strength(("32T3K", 28)).2, Strength::Pair);
        assert_eq!(get_strength(("T55J5", 28)).2, Strength::Three);
        assert_eq!(get_strength(("KTJJT", 28)).2, Strength::TwoPair);
    }

    #[test]
    fn test_get_strength_v2() {
        let res = get_strength_v2(("32T3K", 28));
        assert_eq!(res.2, Strength::Pair);
        assert_eq!(get_strength_v2(("KK677", 28)).2, Strength::TwoPair);
        assert_eq!(get_strength_v2(("T55J5", 28)).2, Strength::Four);
        assert_eq!(get_strength_v2(("KTJJT", 28)).2, Strength::Four);
        // ====
        assert_eq!(get_strength_v2(("AKJTT", 28)).2, Strength::Three);
        assert_eq!(get_strength_v2(("AJJTT", 28)).2, Strength::Four);
        assert_eq!(get_strength_v2(("JJJTT", 28)).2, Strength::Five);
        // ====
        assert_eq!(get_strength_v2(("JJJJK", 28)).2, Strength::Five);
        assert_eq!(get_strength_v2(("JJJAK", 28)).2, Strength::Four);
        assert_eq!(get_strength_v2(("JJJKK", 28)).2, Strength::Five);
        assert_eq!(get_strength_v2(("JJAAK", 28)).2, Strength::Four);
        assert_eq!(get_strength_v2(("JAAAK", 28)).2, Strength::Four);
        assert_eq!(get_strength_v2(("JJAQK", 28)).2, Strength::Three);
        // ====
        assert_eq!(get_strength_v2(("J345A", 3)).2, Strength::Pair);
        assert_eq!(get_strength_v2(("2345J", 5)).2, Strength::Pair);
        assert_eq!(get_strength_v2(("JJJJ2", 3)).2, Strength::Five);
        assert_eq!(get_strength_v2(("2JJJJ", 3)).2, Strength::Five);
    }

    #[test]
    fn test_sort_by_strength() {
        let hand: Hand = ("KK677", 28, Strength::TwoPair);
        let res = hand.sort_by_strength(("KK677", 28, Strength::TwoPair));
        assert_eq!(res, Ordering::Equal);
        assert_eq!(hand.sort_by_strength(("KK577", 28, Strength::TwoPair)), Ordering::Greater);
        assert_eq!(hand.sort_by_strength(("KKA77", 28, Strength::TwoPair)), Ordering::Less);
    }

    #[test]
    fn test_parse_hand() {
        let res = parse_hand("KK677 28");
        assert_eq!(res, ("KK677", 28));
        assert_eq!(parse_hand("TT3KK 1000"), ("TT3KK", 1000));
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample07.txt");
        assert_eq!(res, 5905);
    }

    #[test]
    fn test_read_from_v2_b() {
        let res = read_from_v2("src/input/sample07b.txt");
        assert_eq!(res, 1369);
    }

    #[test]
    fn test_read_from_v2_c() {
        let res = read_from_v2("src/input/sample07c.txt");
        assert_eq!(res, 1387);
    }

    #[test]
    fn test_read_from_v2_d() {
        let res = read_from_v2("src/input/sample07d.txt");
        assert_eq!(res, 3667);
    }

    #[test]
    fn test_read_from_v2_e() {
        let res = read_from_v2("src/input/sample07e.txt");
        // assert_eq!(res, 12); // added one line > result change
        assert_eq!(res, 19);
    }

    #[test]
    fn test_enum_order() {
        assert!(Strength::High < Strength::TwoPair);
        assert!(Strength::TwoPair < Strength::Three);
        assert!(Strength::Three < Strength::Full);
        assert!(Strength::Full < Strength::Four);
        assert!(Strength::Four < Strength::Five);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 249620106);
    }
}