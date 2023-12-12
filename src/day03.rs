use std::cmp;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::{Match, Regex};

use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day03.txt")
}


fn read_from(filepath: &str) -> i64 {
    let mut sample = helpers::read(filepath).unwrap();
    // generate first and last empty lines;
    let line_length = sample.get(0).unwrap().len();
    let empty_line = ".".repeat(line_length);

    sample.insert(0, empty_line.clone());
    sample.push(empty_line);
    let parts_sum: i64 = sample
        .iter()
        .tuple_windows::<(_, _, _)>()
        .flat_map(|(prev, line, next)| get_parts(prev, line, next))
        .filter_map(|part_num: &str| {
            match part_num.parse::<i64>() {
                Ok(num) => Some(num),
                Err(_) => None
            }
        })
        .sum();
    parts_sum
}


lazy_static! {
    static ref NUM_REGEX: Regex = Regex::new(r"\d+").unwrap();
}
fn parse_line(line: &str) -> (&str, Vec<Match>) {
    let all_match_for_line: Vec<Match> = NUM_REGEX.captures_iter(line).map(|captures| captures.get(0).unwrap()).collect();
    (line, all_match_for_line)
}


lazy_static! {
        static ref SPECIAL_CHAR: Regex = Regex::new(r"[^0-9.]").unwrap();
}
fn get_parts<'a>(previous: &str, line: &'a str, next: &str) -> Vec<&'a str> {
    let parsed_line = parse_line(line);
    let res: Vec<&'a str> = parsed_line.1.iter().filter_map(|part_match| {
        let start = cmp::max(part_match.start() as i64 - 1, 0) as usize;
        let end = cmp::min(part_match.end() + 1, line.len());
        let adjacent_char_in_previous_line = SPECIAL_CHAR.is_match(&previous[start..end]);
        let adjacent_char_in_line = SPECIAL_CHAR.is_match(&line[start..end]);
        let adjacent_char_in_next_line = SPECIAL_CHAR.is_match(&next[start..end]);
        if adjacent_char_in_previous_line || adjacent_char_in_line || adjacent_char_in_next_line {
            Some(part_match.as_str())
        } else {
            None
        }
    }).collect();
    res
}

lazy_static! {
        static ref GEAR: Regex = Regex::new(r"[*]").unwrap();
}

fn parse_line_v2(line: &str) -> (&str, Vec<Match>) {
    let all_match_for_line: Vec<Match> = GEAR.captures_iter(line).map(|captures| captures.get(0).unwrap()).collect();
    (line, all_match_for_line)
}

fn get_gears<'a>(previous: &'a str, line: &'a str, next: &'a str) -> Vec<(&'a str, &'a str)> {
    let parsed_line = parse_line_v2(line);
    let gears_previous = parse_line(previous).1;
    let gears_line = parse_line(line).1;
    let gears_next = parse_line(next).1;
    let res: Vec<(&'a str, &'a str)> = parsed_line.1.iter().filter_map(|part_match| {
        let start = cmp::max(part_match.start() as i64 - 1, 0) as usize;
        let end = cmp::min(part_match.end(), line.len());
        let mut gears_p: Vec<&'a str> = gears_previous.iter().filter_map(|&num| {
            if num.end() > start && num.start() <= end {
                Some(num.as_str())
            } else {
                None
            }
        }).collect();
        let mut gears_l: Vec<&'a str> = gears_line.iter().filter_map(|&num| {
            if num.end() > start && num.start() <= end {
                Some(num.as_str())
            } else {
                None
            }
        }).collect();
        let mut gears_n: Vec<&'a str> = gears_next.iter().filter_map(|&num| {
            if num.end() > start && num.start() <= end {
                Some(num.as_str())
            } else {
                None
            }
        }).collect();
        let mut v: Vec<&str> = vec![];
        v.append(&mut gears_p);
        v.append(&mut gears_l);
        v.append(&mut gears_n);
        if v.len() == 2 {
            Some((v[0], v[1]))
        } else {
            None
        }
    }).collect();
    res
}

fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let parts_sum: i64 = sample
        .iter()
        .tuple_windows::<(_, _, _)>()
        .flat_map(|(prev, line, next)| get_gears(prev, line, next))
        .map(|(gear_1, gear_2): (&str, &str)| {
            gear_1.parse::<i64>().unwrap() * gear_2.parse::<i64>().unwrap()
        })
        .sum();
    parts_sum
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day03.txt")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_1() {
        let res = part_1();
        assert_eq!(res, 530495);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample03.txt");
        assert_eq!(res, 4361);
    }

    #[test]
    fn test_parse_line() {
        let s = parse_line("467..114..");
        assert_eq!(s.0, "467..114..");
        assert_eq!(s.1.get(0).unwrap().start(), 0);
        assert_eq!(s.1.get(0).unwrap().end(), 3);
        assert_eq!(s.1.get(0).unwrap().as_str(), "467");
        assert_eq!(s.1.get(1).unwrap().start(), 5);
        assert_eq!(s.1.get(1).unwrap().end(), 8);
        assert_eq!(s.1.get(1).unwrap().as_str(), "114");
    }

    #[test]
    fn test_get_parts() {
        let s = get_parts(
            "...*......",
            "..35..633.",
            "......#...",
        );
        assert_eq!(s, vec!["35", "633"]);
        let s2 = get_parts(
            "..........",
            ".#35..633.",
            "......#...",
        );
        assert_eq!(s2, vec!["35", "633"]);
    }

    #[test]
    fn test_parse_line_v2() {
        let s = parse_line_v2("..*..*..");
        assert_eq!(s.1.get(0).unwrap().start(), 2);
        assert_eq!(s.1.get(0).unwrap().end(), 3);
        assert_eq!(s.1.get(0).unwrap().as_str(), "*");
        assert_eq!(s.1.get(1).unwrap().start(), 5);
        assert_eq!(s.1.get(1).unwrap().end(), 6);
        assert_eq!(s.1.get(1).unwrap().as_str(), "*");
        let s1 = parse_line_v2("...*......");
        assert_eq!(s1.1.get(0).unwrap().start(), 3);
        assert_eq!(s1.1.get(0).unwrap().end(), 4);
        assert_eq!(s1.1.get(0).unwrap().as_str(), "*");
    }

    #[test]
    fn test_get_gears() {
        let s = get_gears(
            "467..114..",
            "...*......",
            "..35..633.",
        );
        assert_eq!(s, vec![("467", "35")]);
        let s1 = get_gears(
            "......",
            "..*...",
            ".1.292",
        );
        assert_eq!(s1, vec![("1", "292")]);
        let s2 = get_gears(
            ".837..",
            "..*...",
            ".1.292",
        );
        assert_eq!(s2, vec![]);
        let s3 = get_gears(
            "......755.",
            "...$.*....",
            ".664.598..",
        );
        assert_eq!(s3, vec![("755", "598")]);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample03.txt");
        assert_eq!(res, 467835);
    }


    #[test]
    fn test_day3_2() {
        let res = part_2();
        assert_eq!(res, 80253814);
    }
}