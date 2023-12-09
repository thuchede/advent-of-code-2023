use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::multi::{separated_list0, separated_list1};
use nom::character::complete;
use nom::error::Error;
use nom::{IResult, Parser};
use crate::helpers;

pub fn part_1() -> i64 {
    read_from("src/input/day09.txt")
}

pub fn part_2() -> i64 {
    read_from_v2("src/input/day09.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();

    let value: i64 = sample.iter()
        .map(|line| parse_number(line).unwrap().1)
        .map(|sequence| process(sequence))
        .map(|diffs| generate_last(diffs))
        .sum();
    value
}


fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();

    let value: i64 = sample.iter()
        .map(|line| parse_number(line).unwrap().1)
        .map(|sequence| process(sequence))
        .map(|diffs| generate_first(diffs))
        .sum();
    value
}

fn parse_number(input: &str) -> IResult<&str, Vec<i64>> {
    let res = separated_list0(tag(" "), complete::i64).parse(input)?;
    Ok(res)
}

fn process(sequence: Vec<i64>) -> Vec<Vec<i64>> {
    let mut lists = vec![sequence.clone()];
    let mut res: Vec<i64> = sequence.iter().tuple_windows().map(|(first, second)| {
        second - first
    }).collect();
    lists.push(res.clone());

    while !res.iter().all(|&e| e == 0) {
        res = res.iter().tuple_windows().map(|(first, second)| {
            second - first
        }).collect();
        lists.push(res.clone());
    }

    lists
}


fn generate_last(diffs: Vec<Vec<i64>>) -> i64 {
    let res = diffs.iter().rev().fold(0, |add_to_last, list| list.last().unwrap() + add_to_last);
    res
}

fn generate_first(diffs: Vec<Vec<i64>>) -> i64 {
    let res = diffs.iter().rev().fold(0, |add_to_first, list| list.first().unwrap() - add_to_first);
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 1782868781);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample09.txt");
        assert_eq!(res, 114);
    }

    #[test]
    fn test_parse_nums() {
        let (_, res) = parse_number("0 3 6 9 12 15").unwrap();
        assert_eq!(res, vec![0, 3, 6, 9, 12, 15]);
    }

    #[test]
    fn test_process() {
        let res = process(vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(res, vec![vec![0, 3, 6, 9, 12, 15], vec![3, 3, 3, 3, 3], vec![0, 0, 0, 0]]);
        let res = process(vec![1, 3, 6, 10, 15, 21]);
        assert_eq!(res, vec![vec![1, 3, 6, 10, 15, 21], vec![2, 3, 4, 5, 6], vec![1, 1, 1, 1], vec![0, 0, 0]]);
    }

    #[test]
    fn test_generate_last() {
        let res = generate_last(vec![vec![0, 3, 6, 9, 12, 15], vec![3, 3, 3, 3, 3], vec![0, 0, 0, 0]]);
        assert_eq!(res, 18);
        let res = generate_last(vec![vec![1, 3, 6, 10, 15, 21], vec![2, 3, 4, 5, 6], vec![1, 1, 1, 1], vec![0, 0, 0]]);
        assert_eq!(res, 28);
    }

    #[test]
    fn test_generate_first() {
        let res = generate_first(vec![vec![0, 3, 6, 9, 12, 15], vec![3, 3, 3, 3, 3], vec![0, 0, 0, 0]]);
        assert_eq!(res, -3);
        let res = generate_first(vec![vec![1, 3, 6, 10, 15, 21], vec![2, 3, 4, 5, 6], vec![1, 1, 1, 1], vec![0, 0, 0]]);
        assert_eq!(res, 0);
        let res = generate_first(vec![
            vec![10, 13, 16, 21, 30, 45],
            vec![3, 3, 5, 9, 15],
            vec![0, 2, 4, 6],
            vec![2, 2, 2],
            vec![0, 0]]);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample09.txt");
        assert_eq!(res, 2);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 1057);
    }
}