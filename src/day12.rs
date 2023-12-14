use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::error::Error;
use nom::multi::{separated_list0};
use nom::Parser;
use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day12.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day12.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let records: Vec<(&str, Vec<i64>)> = sample.iter().map(|line| parse_line(line)).collect();

    let res = records.into_iter().map(|(record, checks)| get_num_sol(record, checks)).sum();

    res
}

fn get_num_sol(record: &str, checks: Vec<i64>) -> i64 {
    let mut storage = vec![record.to_string()];
    for x in (0..(record.len())) {
        if record.chars().nth(x).unwrap() == '?' {
            let new_values: Vec<String> = storage.into_iter().flat_map(|item| {
                let mut dot: String = item.clone();
                dot.replace_range(x..(x + 1), ".");
// println!("dot:{:?}", dot);
                let mut hash: String = item.clone();
                hash.replace_range(x..(x + 1), "#");
// println!("hash:{:?}", hash);
                vec![dot, hash]
            }).collect();
            storage = new_values;
        }
    }

// println!("{:?}", storage);

    let res = storage.into_iter().filter(|attempts| is_valid(attempts, checks.clone())).count() as i64;
    res
}

pub fn get_num_sol_v2(record: &str, checks: Vec<i64>) -> i64 {
    let mut storage = vec![record.to_string()];
    for x in (0..(record.len())) {
        if record.chars().nth(x).unwrap() == '?' {
            let new_values: Vec<String> = storage.into_iter().flat_map(|item| {
                let mut dot: String = item.clone();
                dot.replace_range(x..(x + 1), ".");
                let mut hash: String = item.clone();
                hash.replace_range(x..(x + 1), "#");
                vec![dot, hash]
            }).collect();
            storage = new_values;
        }
    }

    let res = storage.into_iter().filter(|attempts| is_valid(attempts, checks.clone())).count() as i64;
    res
}

fn parse_line(input: &str) -> (&str, Vec<i64>) {
    let (springs_part, input) = input.split_once(" ").unwrap();
    let (_, count): (&str, Vec<i64>) = separated_list0(tag(","), complete::i64::<&str, Error<&str>>).parse(input).unwrap();

    (springs_part, count)
}

fn parse_line_v2(input: &str) -> (String, Vec<i64>) {
    let (springs_part, input) = input.split_once(" ").unwrap();
    let mut springs = springs_part.to_string();
    springs.push_str("?");
    let mut springs_complete = springs.repeat(5);
    springs_complete.replace_range((springs.len() - 1)..springs.len(), "");
    let (_, count): (&str, Vec<i64>) = separated_list0(tag(","), complete::i64::<&str, Error<&str>>).parse(input).unwrap();

    let count_five_times = count.repeat(5);
    (springs_complete, count_five_times)
}


fn is_valid(line: &str, groups: Vec<i64>) -> bool {
    let spring_groups: Vec<&str> = line.split(".").filter(|&c| c != "").collect();
    if spring_groups.len() == groups.len() {
        spring_groups.into_iter().zip(groups.into_iter()).all(|(s, g)| s.len() as i64 == g)
    } else {
        false
    }
}


fn is_not_valid(line: &str, groups: Vec<i64>) -> bool {
    let spring_groups: Vec<&str> = line.split(".").filter(|&c| c != "").collect();
    let is_false = spring_groups.into_iter()
        .zip(groups.into_iter())
        .all(|(s, g)| {
            (s == "#".repeat(g as usize) && s.len() as i64 == g) || s.len() as i64 >= g
        });


    is_false
}


fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let records: Vec<(String, Vec<i64>)> = sample.iter().map(|line| parse_line_v2(line)).collect();

    let res = records.into_iter().map(|(record, checks)| get_num_sol_v2(record.as_str(), checks)).sum();

    res
}

fn find_large_solution(line: String, records: Vec<i64>) -> (String, Vec<i64>) {
    // let largest = records.into_iter().max().unwrap() as usize;
    // let mut first_group = records.clone();
    // let mut second_group = first_group.split_off(largest);
    // let max = second_group.remove(0);
    // let groups: Vec<String> = line.chars().group_by(|&c| c == '#' || c == '?')
    //     .map(|(key, group)| group.collect()).map(|group| group.join("")).collect();

    // for ".##.?######?.##. 1,7,1"
    // find largest group of consecutive spring (7)
    // find largest string that matches largest group (?######?)
    // get combination for that group => 2
    // get combination for sub-problems ".##. 1" and ".##. 1" => 2 and 2
    // get final result 2*2*2 => 8

    ("".to_string(), vec![])
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 7792);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample12.txt");
        assert_eq!(res, 21);
    }

    #[test]
    fn test_get_num_sol() {
        assert_eq!(get_num_sol("???.###", vec![1, 1, 3]), 1);
        assert_eq!(get_num_sol(".??..??...?##.", vec![1, 1, 3]), 4);
        assert_eq!(get_num_sol("?#?#?#?#?#?#?#?", vec![1, 3, 1, 6]), 1);
        assert_eq!(get_num_sol("????.#...#...", vec![4, 1, 1]), 1);
        assert_eq!(get_num_sol("????.######..#####.", vec![1, 6, 5]), 4);
        assert_eq!(get_num_sol("?###????????", vec![3, 2, 1]), 10);
    }

    // #[test]
    // fn test_get_num_sol_v2() {
    // assert_eq!(get_num_sol_v2("???.###????.###????.###????.###????.###", vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]), 1);
    // assert_eq!(get_num_sol_v2(".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.", vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]), 16384);
    // }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample12.txt");
        assert_eq!(res, 525152);
    }

    #[test]
    fn test_parse_line() {
        let res = parse_line(".??..??...?##. 1,1,3");
        assert_eq!(res.0, ".??..??...?##.");
        assert_eq!(res.1, vec![1, 1, 3]);
    }

    #[test]
    fn test_gen() {
        let sample = ".??..??...?##.".to_string();
        let mut s = "ABCD!1234".to_string();
        s.replace_range(4..5, "_");
        let mut storage = vec![sample.clone()];
        for x in (0..(sample.len())) {
            if sample.chars().nth(x).unwrap() == '?' {
                let new_values: Vec<String> = storage.into_iter().flat_map(|item| {
                    let mut dot: String = item.clone();
                    dot.replace_range(x..(x + 1), ".");
                    let mut hash: String = item.clone();
                    hash.replace_range(x..(x + 1), "#");
                    vec![dot, hash]
                }).collect();
                storage = new_values;
            }
        }

        let res = storage.into_iter().filter(|attempts| is_valid(attempts, vec![1, 1, 3])).count();

        assert_eq!(res, 4);
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid("..###.#.##", vec![3, 1, 2]));
        assert!(!is_valid("..###.#.##", vec![3, 1, 3]));
        assert!(!is_valid("..###.#.##", vec![1, 2, 1, 3]));
        // --
        assert!(!is_valid(".###........", vec![3, 2, 1]));
        assert!(!is_valid(".###......##", vec![3, 2, 1]));
    }

    #[test]
    fn test_is_maybe_valid() {
        assert!(is_not_valid("..###.#.??", vec![3, 1, 2]));
        assert!(!is_not_valid("..###.#.?#", vec![3, 2, 1]));
        assert!(is_not_valid("..###.#.?#", vec![3, 1, 1]));
        assert!(!is_not_valid("..###.#.?#", vec![3, 1, 3]));
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 0);
    }
}