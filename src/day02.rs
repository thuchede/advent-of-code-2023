use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{self};
use nom::multi::separated_list0;
use nom::sequence::{preceded, terminated};

use crate::day02::Balls::{Blue, Green, Red};
use crate::helpers;


#[allow(dead_code)]
pub(crate) fn part_1() -> i64 {
    read_from("src/input/day02.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath);
    let res: i64 = sample.unwrap()
        .iter()
        .map(|game| parse_game(game.as_str()).unwrap().1)
        .filter(|(_, game_res)| is_valid_game(game_res))
        .map(|(id, _)| id)
        .sum();
    res
}

// 12 red cubes, 13 green cubes, and 14 blue cubes
fn is_valid_game(game_res: &[Vec<Balls>]) -> bool {
    game_res.iter().flatten().all(|ball| match ball {
        Red(i) => *i <= 12,
        Green(i) => *i <= 13,
        Blue(i) => *i <= 14,
    })
}

#[derive(Debug, PartialEq)]
enum Balls {
    Red(i64),
    Green(i64),
    Blue(i64),
}


fn parse_red_draw(input: &str) -> IResult<&str, Balls> {
    let (input, red_amount) = terminated(
        complete::i64,
        tag(" red"),
    ).parse(input)?;

    Ok((input, Red(red_amount)))
}

fn parse_green_draw(input: &str) -> IResult<&str, Balls> {
    let (input, green_amount) = terminated(
        complete::i64,
        tag(" green"),
    ).parse(input)?;

    Ok((input, Green(green_amount)))
}

fn parse_blue_draw(input: &str) -> IResult<&str, Balls> {
    let (input, blue_amount) = terminated(
        complete::i64,
        tag(" blue"),
    ).parse(input)?;

    Ok((input, Blue(blue_amount)))
}


fn parse_draw(input: &str) -> IResult<&str, Vec<Balls>> {
    let (input, draw) = separated_list0(tag(", "), alt((parse_red_draw, parse_green_draw, parse_blue_draw))).parse(input)?;
    Ok((input, draw))
}

fn parse_draws(input: &str) -> IResult<&str, Vec<Vec<Balls>>> {
    let (input, draws) = separated_list0(tag("; "), parse_draw).parse(input)?;
    Ok((input, draws))
}

fn parse_game(input: &str) -> IResult<&str, (i64, Vec<Vec<Balls>>)> {
    let (input, game_id): (&str, i64) = preceded(tag("Game "), complete::i64).parse(input)?;
    let (input, balls) = preceded(tag(": "), parse_draws).parse(input)?;
    Ok((input, (game_id, balls)))
}

fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath);
    let res: i64 = sample.unwrap()
        .iter()
        .map(|game| parse_game(game.as_str()).unwrap().1)
        .map(|(_, game_res)| get_min_number_of_cubes(&game_res))
        .map(|(r, g, b)| r * g * b)
        .sum();
    res
}

fn get_min_number_of_cubes(game_res: &[Vec<Balls>]) -> (i64, i64, i64) {
    game_res
        .iter()
        .flatten()
        .fold((0, 0, 0), |(r, g, b), ball| match ball {
            Red(i) => if *i > r { (*i, g, b) } else { (r, g, b) },
            Green(i) => if *i > g { (r, *i, b) } else { (r, g, b) },
            Blue(i) => if *i > b { (r, g, *i) } else { (r, g, b) },
        })
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day02.txt")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_1() {
        let res = part_1();
        assert_eq!(res, 2720);
    }

    #[test]
    fn test_day2_1_sample() {
        let res = read_from("src/input/sample02.txt");
        assert_eq!(res, 8);
    }

    #[test]
    fn test_day2_1_red_draw() {
        assert_eq!(parse_red_draw("12 red").unwrap(), ("", Red(12)));
        // assert_eq!(red_draw("12 red, ").unwrap(), (, ", Red(12),));
        // assert_eq!(red_draw(", 12 red, green").unwrap(), (, green", Red(12),));
    }

    #[test]
    fn test_day2_1_green_draw() {
        assert_eq!(parse_green_draw("12 green").unwrap(), ("", Green(12)));
        assert_eq!(parse_green_draw("12 green, ").unwrap(), (", ", Green(12)));
    }

    #[test]
    fn test_day2_1_blue_draw() {
        assert_eq!(parse_blue_draw("12 blue").unwrap(), ("", Blue(12)));
        assert_eq!(parse_blue_draw("12 blue, ").unwrap(), (", ", Blue(12)));
    }

    #[test]
    fn test_day2_1_draw() {
        assert_eq!(parse_draw("12 red").unwrap(), ("", vec![Red(12)]));
        assert_eq!(parse_draw("13 green").unwrap(), ("", vec![Green(13)]));
        assert_eq!(parse_draw("14 blue").unwrap(), ("", vec![Blue(14)]));
        assert_eq!(parse_draw("14 blue, 12 red").unwrap(), ("", vec![Blue(14), Red(12)]));
        assert_eq!(parse_draw("13 green, 14 blue").unwrap(), ("", vec![Green(13), Blue(14)]));
    }

    #[test]
    fn test_day2_1_draws() {
        assert_eq!(parse_draws("12 red").unwrap(), ("", vec![vec![Red(12)]]));
        assert_eq!(parse_draws("14 blue, 12 red").unwrap(), ("", vec![vec![Blue(14), Red(12)]]));
        assert_eq!(parse_draws("14 blue, 12 red; 13 green, 14 blue").unwrap(), ("", vec![vec![Blue(14), Red(12)], vec![Green(13), Blue(14)]]));
    }

    #[test]
    fn test_day2_1_game() {
        assert_eq!(
            parse_game("Game 1: 12 red").unwrap().1,
            (1, vec![vec![Red(12)]])
        );
        assert_eq!(
            parse_game("Game 2: 14 blue, 12 red").unwrap().1,
            (2, vec![vec![Blue(14), Red(12)]])
        );
        assert_eq!(
            parse_game("Game 3: 14 blue, 12 red; 13 green, 14 blue").unwrap().1,
            (3, vec![vec![Blue(14), Red(12)], vec![Green(13), Blue(14)]])
        );
    }

    #[test]
    fn test_day2_is_valid_game() {
        assert_eq!(
            is_valid_game(&vec![vec![Red(12)]]),
            true
        );
        assert_eq!(
            is_valid_game(&vec![vec![Red(13)]]),
            false
        );
        assert_eq!(
            is_valid_game(&vec![vec![Red(12), Green(14)]]),
            false
        );
        assert_eq!(
            is_valid_game(&vec![vec![Red(13)], vec![Blue(15)]]),
            false
        );
    }

    #[test]
    fn test_day2_2_sample() {
        let res = read_from_v2("src/input/sample02.txt");
        assert_eq!(res, 2286);
    }

    #[test]
    fn test_day2_2_get_min_number_of_cubes() {
        let res = get_min_number_of_cubes(&vec![vec![Red(1)]]);
        assert_eq!(res, (1, 0, 0));
        assert_eq!(
            get_min_number_of_cubes(&vec![vec![Red(2), Green(2)], vec![Red(1), Green(4)], vec![Red(1), Blue(5)]]),
            (2, 4, 5));
    }


    #[test]
    fn test_day2_2() {
        let res = part_2();
        assert_eq!(res, 71535);
    }
}