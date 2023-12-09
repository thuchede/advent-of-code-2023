use std::collections::HashMap;

use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::error::Error;
use nom::sequence::{delimited, pair, preceded, terminated};
use num::integer::lcm;

use crate::helpers;

pub fn part_1() -> i64 {
    read_from("src/input/day08.txt")
}

pub fn part_2() -> i64 {
    read_from_v2("src/input/day08.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let instructions = parse_instructions(sample.first().unwrap().as_str());
    let maps: HashMap<&str, (&str, &str)> = sample.iter().skip(2).map(|line| parse_map(line.as_str())).collect();
    process_instructions(instructions, "AAA", "ZZZ", maps)
}

#[derive(Debug, PartialEq, Eq)]
struct DesertMap {
    from: &'static str,
    to_left: &'static str,
    to_right: &'static str,
}

fn parse_instructions(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn parse_map(input: &str) -> (&str, (&str, &str)) {
    let (input, node): (&str, &str) = terminated(alphanumeric1::<&str, Error<&str>>, tag(" = ")).parse(input).unwrap();
    let (_, left_or_right): (&str, (&str, &str)) = delimited(tag("("), pair(alphanumeric1::<&str, Error<&str>>, preceded(tag(", "), alphanumeric1)), tag(")")).parse(input).unwrap();
    (node, left_or_right)
}

fn is_last((position, (right, left)): (&str, (&str, &str))) -> bool {
    position == right && position == left
}

fn process_instructions(instructions: Vec<char>, starting_node: &str, last_node: &str, maps: HashMap<&str, (&str, &str)>) -> i64 {
    let mut steps: usize = 0;
    let mut current_node = starting_node;
    let instructions_len = instructions.len();
    while current_node != last_node {
        let step = instructions.get(steps % instructions_len).unwrap();
        current_node = if *step == 'L' {
            maps.get(current_node).unwrap().0
        } else {
            maps.get(current_node).unwrap().1
        };
        steps += 1;
    }
    steps as i64
}


fn process_instructions_v2(instructions: &Vec<char>, starting_node: &str, maps: &HashMap<&str, (&str, &str)>) -> i64 {
    let mut steps: usize = 0;
    let mut current_node = starting_node;
    let instructions_len = instructions.len();
    while current_node.chars().nth(2).unwrap() != 'Z' {
        let step = instructions.get(steps % instructions_len).unwrap();
        current_node = if *step == 'L' {
            maps.get(current_node).unwrap().0
        } else {
            maps.get(current_node).unwrap().1
        };
        steps += 1;
    }
    steps as i64
}

fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();

    let instructions = &parse_instructions(sample.first().unwrap().as_str());
    let rows: Vec<(&str, (&str, &str))> = sample.iter().skip(2).map(|line| parse_map(line.as_str())).collect();
    let maps: &HashMap<&str, (&str, &str)> = &rows.clone().into_iter().collect();
    let starting_nodes: Vec<&str> = rows.into_iter().filter_map(|(node, _)| if node.ends_with('A') { Some(node) } else { None }).collect();

    let res: i64 = starting_nodes.into_iter().map(|node| {
        process_instructions_v2(instructions, node, maps)
    }).fold(1, lcm);
    res
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 13207);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample08.txt");
        assert_eq!(res, 2);
    }

    #[test]
    fn test_read_from_b() {
        let res = read_from("src/input/sample08b.txt");
        assert_eq!(res, 6);
    }

    #[test]
    fn test_parse_map() {
        let res = parse_map("AAA = (BBB, CCC)");
        assert_eq!(res.0, "AAA");
        assert_eq!(res.1.0, "BBB");
        assert_eq!(res.1.1, "CCC");
    }

    #[test]
    fn test_parse_instructions() {
        let res = parse_instructions("RLLRR");
        assert_eq!(*res.get(0).unwrap(), 'R');
        assert_eq!(*res.get(1).unwrap(), 'L');
        assert_eq!(*res.get(2).unwrap(), 'L');
        assert_eq!(*res.get(3).unwrap(), 'R');
        assert_eq!(*res.get(4).unwrap(), 'R');
    }

    #[test]
    fn test_is_last() { // FIXME: unused as the last node is always expected to be ZZZ for p1
        assert!(is_last(("AAA", ("AAA", "AAA"))));
        assert!(!is_last(("AAA", ("BBB", "BBB"))));
        assert!(!is_last(("AAA", ("AAA", "BBB"))));
        assert!(!is_last(("AAA", ("BBB", "AAA"))));
        assert!(!is_last(("AAA", ("BBB", "CCC"))));
    }

    #[test]
    fn test_process_instruction() {
        let res = process_instructions(vec!['R'], "AAA", "AAA", HashMap::from([("AAA", ("AAA", "AAA"))]));
        assert_eq!(res, 0);
        let res = process_instructions(vec!['R'], "BBB", "AAA", HashMap::from([("BBB", ("BBB", "AAA")), ("AAA", ("AAA", "AAA"))]));
        assert_eq!(res, 1);
        let res = process_instructions(vec!['L', 'R'], "BBB", "AAA", HashMap::from([("BBB", ("BBB", "AAA")), ("AAA", ("AAA", "AAA"))]));
        assert_eq!(res, 2);
        let res = process_instructions(vec!['L', 'R'], "AAA", "AAA", HashMap::from([("BBB", ("BBB", "AAA")), ("AAA", ("AAA", "AAA"))]));
        assert_eq!(res, 0);
    }

    #[test]
    fn test_process_instruction_v2() {
        let res = process_instructions_v2(&vec!['L', 'R'], "11A", &HashMap::from([
            ("11A", ("11B", "XXX")),
            ("11B", ("XXX", "11Z")),
            ("11Z", ("11B", "XXX"))
        ]));
        assert_eq!(res, 2);
        let res = process_instructions_v2(&vec!['L', 'R'], "22A", &HashMap::from([
            ("22A", ("22B", "XXX")),
            ("22B", ("22C", "22C")),
            ("22C", ("22Z", "22Z")),
            ("22Z", ("22B", "22B"))
        ]));
        assert_eq!(res, 3);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample08c.txt");
        assert_eq!(res, 6);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 12324145107121);
    }
}