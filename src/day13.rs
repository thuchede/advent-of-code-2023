use std::cmp;
use find_all::FindAll;

use itertools::Itertools;

use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day13.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day13.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();

    let splitted = sample.into_iter()
        .group_by(|k| k != "")
        .into_iter()
        .filter(|(key, group)| *key)
        .map(|(_, group)| group.collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();


    let res = splitted.into_iter().map(|v| {
        match get_summary_h_mirror(v.clone()) {
            0 => get_summary_v_mirror(v) as i64,
            i => i as i64
        }
    }).sum();

    res
}

fn get_mirror_index(notes: Vec<String>) -> Vec<usize> {
    let index = notes.into_iter()
        .enumerate()
        .tuple_windows()
        .find_all(|(first, second)| {
            first.1 == second.1
        });
    index.unwrap_or(vec![])
}

fn get_mirror_index_v2(notes: Vec<String>) -> Vec<usize> {
    let index = notes.into_iter()
        .enumerate()
        .tuple_windows()
        .find_all(|((_, first), (_, second))| {
            let differ_by_one = first.chars().zip(second.chars()).filter(|(c1, c2)| c1 != c2).nth(1).is_none();
            *first == *second || differ_by_one
        });
    index.unwrap_or(vec![])
}

fn is_mirror(notes: Vec<String>, index: usize) -> bool {
    let max_checks = cmp::min(index + 1, notes.len() - index - 1);
    if index > notes.len() {
        false
    } else {
        let test = (0..max_checks).into_iter().all(|i| {
            let l1 = notes.get(index - i).unwrap();
            let l2 = notes.get(index + i + 1).unwrap();
            let ch = l1 == l2;
            ch
        });
        test
    }
}

fn is_mirror_v2(notes: Vec<String>, index: usize) -> bool {
    let max_checks = cmp::min(index + 1, notes.len() - index - 1);
    let mut nb_of_fixed_smudge = 0;
    if index > notes.len() {
        false
    } else {
        let test = (0..max_checks).into_iter().all(|i| {
            let first = notes.get(index - i).unwrap();
            let second = notes.get(index + i + 1).unwrap();
            let differ_by_one = first.chars().zip(second.chars()).filter(|(c1, c2)| c1 != c2).count() == 1;
            if differ_by_one {
                nb_of_fixed_smudge += 1;
                nb_of_fixed_smudge == 1
            } else {
                first == second
            }
        });
        test && nb_of_fixed_smudge == 1
    }
}

fn get_summary_h_mirror(notes: Vec<String>) -> usize {
    // FIXME: change to & instead of clone
    let indexes = get_mirror_index(notes.clone());
    let n = indexes.into_iter().find_map(|index| if is_mirror(notes.clone(), index) { Some(index) } else { None });

    if let Some(i) = n {
        (i + 1) * 100
    } else {
        0
    }
}

fn get_summary_v_mirror(notes: Vec<String>) -> usize {
    // flip matrix
    let notes_chars = notes.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let size = notes_chars.first().unwrap().len();
    let flipped: Vec<Vec<char>> = (0..size).map(|i| notes_chars.clone()
        .into_iter()
        .map(|c| c.get(i).unwrap().clone())
        .collect()
    ).collect();
    let collapsed = flipped.iter().map(|v| v.into_iter().collect::<String>()).collect::<Vec<String>>();

    // FIXME: change to & instead of clone
    let indexes = get_mirror_index(collapsed.clone());
    let n = indexes.into_iter().find_map(|index| if is_mirror(collapsed.clone(), index) { Some(index) } else { None });

    match n {
        Some(i) => i + 1,
        None => 0
    }
}

fn get_summary_h_mirror_v2(notes: Vec<String>) -> usize {
    // FIXME: change to & instead of clone
    let indexes = get_mirror_index_v2(notes.clone());
    let n = indexes.into_iter().find_map(|index| if is_mirror_v2(notes.clone(), index) { Some(index) } else { None });

    if let Some(i) = n {
        (i + 1) * 100
    } else {
        0
    }
}

fn get_summary_v_mirror_v2(notes: Vec<String>) -> usize {
    // flip matrix
    let notes_chars = notes.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let size = notes_chars.first().unwrap().len();
    let flipped: Vec<Vec<char>> = (0..size).map(|i| notes_chars.clone()
        .into_iter()
        .map(|c| c.get(i).unwrap().clone())
        .collect()
    ).collect();
    let collapsed = flipped.iter().map(|v| v.into_iter().collect::<String>()).collect::<Vec<String>>();

    // FIXME: change to & instead of clone
    let indexes = get_mirror_index_v2(collapsed.clone());
    let n = indexes.into_iter().find_map(|index| if is_mirror_v2(collapsed.clone(), index) { Some(index) } else { None });

    match n {
        Some(i) => {
            i + 1
        }
        None => 0
    }
}


fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();

    let splitted = sample.into_iter()
        .group_by(|k| k != "")
        .into_iter()
        .filter(|(key, group)| *key)
        .map(|(_, group)| group.collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();


    let res = splitted.into_iter().map(|v| {
        match get_summary_h_mirror_v2(v.clone()) {
            0 => get_summary_v_mirror_v2(v) as i64,
            i => i as i64
        }
    }).sum();

    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 37381);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample13.txt");
        assert_eq!(res, 405);
        let res = read_from("src/input/sample13b.txt");
        assert_eq!(res, 100);
        let res = read_from("src/input/sample13c.txt");
        assert_eq!(res, 3);
    }

    #[test]
    fn test_get_mirror_index() {
        let res = get_mirror_index(vec!["aabb".to_string(), "bbaa".to_string(), "abcd".to_string(), "abcd".to_string(), "bbaa".to_string()]);
        assert_eq!(res, vec![2]);
        let res = get_mirror_index(vec!["aabb".to_string(), "bbaa".to_string(), "abcd".to_string(), "abdd".to_string(), "bbaa".to_string()]);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn test_get_mirror_index_v2() {
        let res = get_mirror_index_v2(vec!["bbab".to_string(), "bbaa".to_string(), "abcd".to_string(), "abcd".to_string(), "bbaa".to_string()]);
        assert_eq!(res, vec![0, 2]);
        let res = get_mirror_index_v2(vec!["aabb".to_string(), "bbaa".to_string(), "abcd".to_string(), "abdd".to_string(), "bbaa".to_string()]);
        assert_eq!(res, vec![2]);
        let res = get_mirror_index_v2(vec!["aabb".to_string(), "bbaa".to_string(), "abcd".to_string(), "abdh".to_string(), "bbaa".to_string()]);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn test_is_mirror() {
        let res = is_mirror(vec!["aabb".to_string(), "bbaa".to_string(), "abcd".to_string(), "abcd".to_string(), "bbaa".to_string()], 2);
        assert!(res);
        let res = is_mirror(vec!["aabb".to_string(), "xxxx".to_string(), "abcd".to_string(), "abcd".to_string(), "bbaa".to_string()], 2);
        assert!(!res);
        let res = is_mirror(vec!["#..#.##.#..".to_string(), "#..#.##.#..".to_string(), "#..#.#.##..".to_string()], 0);
        assert!(res);
    }

    #[test]
    fn test_is_mirror_v2() {
        let res = is_mirror(vec!["####".to_string(), "####".to_string(), "....".to_string(), "...#".to_string(), "####".to_string()], 0);
        assert!(res);
        let res = is_mirror_v2(vec!["####".to_string(), "####".to_string(), "....".to_string(), "...#".to_string(), "####".to_string()], 0);
        assert!(!res);
        let res = is_mirror_v2(vec!["####".to_string(), "####".to_string(), "....".to_string(), "...#".to_string(), "####".to_string()], 2);
        assert!(res);
    }

    #[test]
    fn test_get_summary_h_mirror() {
        let res = get_summary_h_mirror(vec!["aabb".to_string(), "bbaa".to_string(), "abcd".to_string(), "abcd".to_string(), "bbaa".to_string()]);
        assert_eq!(res, 300);
    }

    #[test]
    fn test_get_summary_v_mirror() {
        let res = get_summary_v_mirror(vec!["cabba".to_string(), "dbbbb".to_string(), "cadda".to_string(), "aabba".to_string(), "bbaab".to_string()]);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample13.txt");
        assert_eq!(res, 400);
        let res = read_from_v2("src/input/sample13c.txt");
        assert_eq!(res, 10);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 28210);
    }
}