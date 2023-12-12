use std::collections::HashMap;
use crate::helpers;

use itertools::Itertools;

pub fn part_1() -> i64 {
    read_from("src/input/day11.txt")
}

pub fn part_2() -> i64 {
    read_from_v2("src/input/day11.txt", 1_000_000)
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();

    let galaxies: Vec<Vec<Option<Galaxy>>> = sample.iter().map(|line| parse_line(line.as_str())).collect();

    let res = expand_universe(galaxies);

    let galaxies_coordinates: Vec<(i64, i64)> = res.into_iter().enumerate().flat_map(|(y, row)| {
        let rows: Vec<(i64, i64)> = row.into_iter().enumerate().filter(|(_, maybe_galaxy)| maybe_galaxy.is_some()).map(|(x, _)| (x as i64, y as i64)).collect();
        rows
    }).collect();

    let res = galaxies_coordinates.into_iter().combinations(2).map(|a| get_distance(*a.first().unwrap(), *a.last().unwrap())).sum();

    res
}

fn get_distance((ax, ay): (i64, i64), (bx, by): (i64, i64)) -> i64 {
    (ax - bx).abs() + (ay - by).abs()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Galaxy;

fn parse_line(input: &str) -> Vec<Option<Galaxy>> {
    let galaxies: Vec<Option<Galaxy>> = input.chars().map(|c| match c {
        '#' => Some(Galaxy),
        _ => None
    }).collect();

    galaxies
}

fn expand_universe(galaxies: Vec<Vec<Option<Galaxy>>>) -> Vec<Vec<Option<Galaxy>>> {
    let y_expanded_galaxies: Vec<Vec<Option<Galaxy>>> = galaxies.into_iter().flat_map(|line| if is_empty(&line) { vec![line.clone(), line] } else { vec![line] }).collect();
    let size = y_expanded_galaxies.first().unwrap().len();

    // flip matrix
    let flipped: Vec<Vec<Option<Galaxy>>> = (0..size).map(|i| y_expanded_galaxies.clone()
        .into_iter()
        .map(|c| c.get(i).unwrap().clone())
        .collect()
    ).collect();

    let x_expanded_galaxies: Vec<Vec<Option<Galaxy>>> = flipped.into_iter().flat_map(|line| if is_empty(&line) { vec![line.clone(), line] } else { vec![line] }).collect();
    let size = x_expanded_galaxies.first().unwrap().len();

    // flip matrix back
    let flipped_back: Vec<Vec<Option<Galaxy>>> = (0..size).map(|i| x_expanded_galaxies.clone()
        .into_iter()
        .map(|c| c.get(i).unwrap().clone())
        .collect()
    ).collect();

    flipped_back
}

fn is_empty(line: &Vec<Option<Galaxy>>) -> bool {
    line.iter().all(|c| c.is_none())
}


fn read_from_v2(filepath: &str, age: i64) -> i64 {
    let sample = helpers::read(filepath).unwrap();

    let galaxies: Vec<Vec<Option<Galaxy>>> = sample.iter().map(|line| parse_line(line.as_str())).collect();


    let original_galaxies_coordinates: Vec<(i64, i64)> = galaxies.clone().into_iter().enumerate().flat_map(|(y, row)| {
        let rows: Vec<(i64, i64)> = row.into_iter().enumerate().filter(|(_, maybe_galaxy)| maybe_galaxy.is_some()).map(|(x, _)| (x as i64, y as i64)).collect();
        rows
    }).collect();

    let res = expand_universe(galaxies);
    let galaxies_coordinates: Vec<(i64, i64)> = res.into_iter().enumerate().flat_map(|(y, row)| {
        let rows: Vec<(i64, i64)> = row.into_iter().enumerate().filter(|(_, maybe_galaxy)| maybe_galaxy.is_some()).map(|(x, _)| (x as i64, y as i64)).collect();
        rows
    }).collect();

    let tmp_res = original_galaxies_coordinates.clone().into_iter().zip(galaxies_coordinates.clone().into_iter())
        .combinations(2)
        .map(|a| {
            let ((ax_origin, ay_origin), (ax_expanded, ay_expanded)) = *a.first().unwrap();
            let ((bx_origin, by_origin), (bx_expanded, by_expanded)) = *a.last().unwrap();

            let original_x_distance = (ax_origin - bx_origin).abs();
            let expanded_x_distance = (ax_expanded - bx_expanded).abs();
            let diff_x_distance = expanded_x_distance - original_x_distance;

            let final_x = original_x_distance + diff_x_distance * (age - 1);

            let original_y_distance = (ay_origin - by_origin).abs();
            let expanded_y_distance = (ay_expanded - by_expanded).abs();
            let diff_y_distance = expanded_y_distance - original_y_distance;

            let final_y = original_y_distance + diff_y_distance * (age - 1);

            final_x + final_y
        }).sum();

    tmp_res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 9799681);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample11.txt");
        assert_eq!(res, 374);
    }

    #[test]
    fn test_get_distance() {
        let res = get_distance((4, 0), (9, 1));
        assert_eq!(res, 6);
        let res = get_distance((0, 2), (12, 7));
        assert_eq!(res, 17);
    }

    #[test]
    fn test_parse_line() {
        let res = parse_line("...#...");
        assert_eq!(res.len(), 7);
        assert_eq!(*res.get(0).unwrap(), None);
        assert_eq!(*res.get(3).unwrap(), Some(Galaxy));
    }

    #[test]
    fn test_snippet_flip() {
        let sample = vec![vec![None, Some(Galaxy)], vec![None, None]];
        let flipped: Vec<Vec<Option<Galaxy>>> = (0..sample.first().unwrap().len()).map(|i| sample.clone()
            .into_iter()
            .map(|c| c.get(i).unwrap().clone())
            .collect()
        ).collect();
        assert_eq!(flipped, vec![vec![None, None], vec![Some(Galaxy), None]]);
        let flipped_back: Vec<Vec<Option<Galaxy>>> = (0..flipped.first().unwrap().len()).map(|i| flipped.clone()
            .into_iter()
            .map(|c| c.get(i).unwrap().clone())
            .collect()
        ).collect();
        assert_eq!(flipped_back, vec![vec![None, Some(Galaxy)], vec![None, None]]);
        // ------
        let sample = vec![vec![None, Some(Galaxy), None], vec![None, None, None]];
        let flipped: Vec<Vec<Option<Galaxy>>> = (0..sample.first().unwrap().len()).map(|i| sample.clone()
            .into_iter()
            .map(|c| c.get(i).unwrap().clone())
            .collect()
        ).collect();
        assert_eq!(flipped, vec![vec![None, None], vec![Some(Galaxy), None], vec![None, None]]);
        let flipped_back: Vec<Vec<Option<Galaxy>>> = (0..flipped.first().unwrap().len()).map(|i| flipped.clone()
            .into_iter()
            .map(|c| c.get(i).unwrap().clone())
            .collect()
        ).collect();
        assert_eq!(flipped_back, vec![vec![None, Some(Galaxy), None], vec![None, None, None]]);
    }

    #[test]
    fn test_expand_universe() {
        // let res = expand_universe(vec![vec![None]]);
        // assert_eq!(res, vec![vec![None, None], vec![None, None]]);
        let res = expand_universe(vec![vec![None, Some(Galaxy)], vec![None, None]]);
        assert_eq!(res, vec![vec![None, None, Some(Galaxy)], vec![None, None, None], vec![None, None, None]]);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample11.txt", 2);
        assert_eq!(res, 374);
        let res = read_from_v2("src/input/sample11.txt", 10);
        assert_eq!(res, 1030);
        let res = read_from_v2("src/input/sample11.txt", 100);
        assert_eq!(res, 8410);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 513171773355);
    }
}