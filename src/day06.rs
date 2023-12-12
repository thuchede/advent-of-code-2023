use nom::{IResult, Parser};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::space1;
use nom::multi::{many1, separated_list0};
use nom::sequence::{preceded, terminated};

use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day06.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day06.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let (_, durations) = parse_duration(sample.get(0).unwrap()).unwrap();
    let (_, distances) = parse_distance(sample.get(1).unwrap()).unwrap();
    let solutions: i64 = durations.iter().zip(distances.iter()).map(|(&duration, &distance)| {
        get_nb_solutions(
            (duration, distance)
        )
    }).reduce(|a, b| a * b).unwrap_or(0);
    solutions
}

fn parse_duration(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, durations): (&str, Vec<i64>) = preceded(terminated(tag("Time:"), many1(space1)), separated_list0(many1(space1), complete::i64)).parse(input)?;
    Ok((input, durations))
}

fn parse_distance(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, durations): (&str, Vec<i64>) = preceded(terminated(tag("Distance:"), many1(space1)), separated_list0(many1(space1), complete::i64)).parse(input)?;
    Ok((input, durations))
}

fn get_min_press_time((course_duration, best_distance): (i64, i64)) -> i64 {
    let min_time_pressed = (0..course_duration).find_map(|time_pressed| {
        let distance = time_pressed * (course_duration - time_pressed);
        if distance > best_distance {
            return Some(time_pressed);
        }
        None
    }).unwrap_or(0);
    min_time_pressed
}

fn get_max_press_time((course_duration, best_distance): (i64, i64)) -> i64 {
    let max_time_pressed = (0..course_duration).rev().find_map(|time_pressed| {
        let distance = time_pressed * (course_duration - time_pressed);
        if distance > best_distance {
            return Some(time_pressed);
        }
        None
    }).unwrap_or(0);
    max_time_pressed
}

fn get_nb_solutions((course_duration, best_distance): (i64, i64)) -> i64 {
    let min_time_pressed = get_min_press_time((course_duration, best_distance));
    let max_time_pressed = get_max_press_time((course_duration, best_distance));
    max_time_pressed - min_time_pressed + 1
}

fn parse_duration_v2(input: &str) -> IResult<&str, i64> {
    let (input, durations): (&str, Vec<&str>) = preceded(terminated(tag("Time:"), many1(space1)), separated_list0(many1(space1), complete::digit1)).parse(input)?;
    let duration_str = durations.iter().fold("".to_owned(), |a, &b| a.to_owned() + b);
    let duration = duration_str.parse::<i64>().unwrap();
    Ok((input, duration))
}

fn parse_distance_v2(input: &str) -> IResult<&str, i64> {
    let (input, distances): (&str, Vec<&str>) = preceded(terminated(tag("Distance:"), many1(space1)), separated_list0(many1(space1), complete::digit1)).parse(input)?;
    let distance_str = distances.iter().fold("".to_owned(), |a, &b| a.to_owned() + b);
    let distance = distance_str.parse::<i64>().unwrap();
    Ok((input, distance))
}

fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let (_, duration) = parse_duration_v2(sample.get(0).unwrap()).unwrap();
    let (_, distance) = parse_distance_v2(sample.get(1).unwrap()).unwrap();
    let solution: i64 = get_nb_solutions(
        (duration, distance)
    );
    solution
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_1() {
        let res = part_1();
        assert_eq!(res, 211904);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample06.txt");
        assert_eq!(res, 288);
    }

    #[test]
    fn test_get_min_press_time() {
        let res = get_min_press_time((7, 9));
        assert_eq!(res, 2);
    }

    #[test]
    fn test_get_max_press_time() {
        let res = get_max_press_time((7, 9));
        assert_eq!(res, 5);
    }

    #[test]
    fn test_get_nb_solutions() {
        let res = get_nb_solutions((7, 9));
        assert_eq!(res, 4);
    }

    #[test]
    fn test_parse_duration() {
        let (_, res) = parse_duration("Time:      7  15   30").unwrap();
        assert_eq!(res, vec![7, 15, 30]);
    }

    #[test]
    fn test_parse_distance() {
        let (_, res) = parse_distance("Distance:   334   1135   1350   2430").unwrap();
        assert_eq!(res, vec![334, 1135, 1350, 2430]);
    }

    #[test]
    fn test_parse_duration_v2() {
        let (_, res) = parse_duration_v2("Time:      7  15   30").unwrap();
        assert_eq!(res, 71530);
    }

    #[test]
    fn test_parse_distance_v2() {
        let (_, res) = parse_distance_v2("Distance:  9  40  200").unwrap();
        assert_eq!(res, 940200);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample06.txt");
        assert_eq!(res, 71503);
    }

    #[test]
    fn test_day6_2() {
        let res = part_2();
        assert_eq!(res, 43364472);
    }
}