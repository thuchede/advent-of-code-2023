use std::collections::HashSet;
use nom::{IResult, Parser};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::multi::separated_list0;
use nom::sequence::{preceded, tuple};

use crate::helpers;

pub fn part_1() -> i64 {
    read_from("src/input/day04.txt")
}

pub fn part_2() -> i64 {
    read_from_v2("src/input/day04.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let res = sample.iter()
        .map(|line| parse_game(line))
        .map(|game| game.unwrap().1)
        .map(|res| get_number_of_winning_num(res))
        .map(get_card_value)
        .sum();
    res
}

fn parse_nums(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, m) = separated_list0(
        space1, digit1).parse(input)?;
    Ok((input, m))
}

fn parse_game_result(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (input, m) = separated_list0(preceded(tag(" |"), space1), parse_nums).parse(input)?;
    let winning = m.get(0).unwrap().to_vec();
    let played = m.get(1).unwrap().to_vec();
    Ok((input, (winning, played)))
}

fn parse_game(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (input, m) = preceded(
        tuple((
            tag("Card"),
            space1,
            digit1,
            tag(":"),
            space1,
        )),
        parse_game_result,
    ).parse(input)?;
    Ok((input, m))
}

fn get_number_of_winning_num((winning_num, played_nums): (Vec<&str>, Vec<&str>)) -> usize {
    let hash_win: HashSet<&str> = HashSet::from_iter(winning_num.iter().cloned());
    let hash_pla: HashSet<&str> = HashSet::from_iter(played_nums.iter().cloned());
    let res = hash_win.intersection(&hash_pla).collect::<Vec<&&str>>().len();
    res
}


fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let num_games = sample.len();
    let mut nb_card: Vec<i64> = vec![1; num_games];

    let res: Vec<i64> = sample.iter()
        .map(|line| parse_game(line))
        .map(|game| game.unwrap().1)
        .map(|res| get_number_of_winning_num(res) as i64)
        .collect();

    for (idx, range) in res.iter().enumerate() {
        let value = nb_card[idx];
        let rangeusize = *range as usize;
        for (i, unit) in nb_card[(idx + 1)..=(idx + rangeusize)].iter_mut().enumerate() {
            *unit += value;
        }
    }
    let res = nb_card.iter().sum();
    res
}

fn get_card_value(num_of_wins: usize) -> i64 {
    match num_of_wins {
        0 => 0,
        n => 2i64.pow((n - 1) as u32)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_1() {
        let res = part_1();
        assert_eq!(res, 24175);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample04.txt");
        assert_eq!(res, 13);
    }

    #[test]
    fn test_parse_nums() {
        let res = parse_nums("12 23 45").unwrap();
        assert_eq!(res.1, vec!["12", "23", "45"]);
        let res_with_double_space = parse_nums("12 23  5").unwrap();
        assert_eq!(res_with_double_space.1, vec!["12", "23", "5"]);
    }

    #[test]
    fn test_parse_game_result() {
        let res = parse_game_result("12 23 45 | 12 45 78").unwrap();
        assert_eq!(res.1, (vec!["12", "23", "45"], vec!["12", "45", "78"]));
        let res_space = parse_game_result("2 23 45 |  2 45 78").unwrap();
        assert_eq!(res_space.1, (vec!["2", "23", "45"], vec!["2", "45", "78"]));
    }

    #[test]
    fn test_parse_game() {
        let res = parse_game("Card  71: 12 23 45 | 12 45 78").unwrap();
        assert_eq!(res.1, (vec!["12", "23", "45"], vec!["12", "45", "78"]));
        let res_space = parse_game("Card  72:  2 23 45 | 12 45 78").unwrap();
        assert_eq!(res_space.1, (vec!["2", "23", "45"], vec!["12", "45", "78"]));
    }

    #[test]
    fn test_get_number_of_winning_num() {
        let res = get_number_of_winning_num((vec!["12", "23", "45"], vec!["12", "45", "78"]));
        assert_eq!(res, 2);
    }

    #[test]
    fn test_get_card_value() {
        assert_eq!(get_card_value(0), 0);
        assert_eq!(get_card_value(1), 1);
        assert_eq!(get_card_value(2), 2);
        assert_eq!(get_card_value(3), 4);
        assert_eq!(get_card_value(4), 8);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample04.txt");
        assert_eq!(res, 30);
    }


    #[test]
    fn test_day4_2() {
        let res = part_2();
        assert_eq!(res, 18846301);
    }
}