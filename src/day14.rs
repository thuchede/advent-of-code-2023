use itertools::Itertools;
use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day14.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day14.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();

    let flipped: Vec<String> = flip(&sample);

    let new_layout: Vec<String> = flipped.iter().map(|line| slide_rock(line)).collect();

    let flipped_back: Vec<String> = flip(&new_layout);

    let res = count_load(&flipped_back);

    res as i64
}

fn slide_rock(source: &String) -> String {
    let sections = source.split("#");
    let moved_rocks: Vec<String> = sections.map(|v| v.chars().sorted().rev().join("")).collect();
    moved_rocks.join("#")
}

fn flip(source: &Vec<String>) -> Vec<String> {
    let source_chars: Vec<Vec<char>> = source.iter().map(|line| line.chars().collect::<Vec<char>>()).collect();
    let size = source_chars.len();

    let flipped: Vec<String> = (0..size).map(|i| source_chars.clone()
        .into_iter()
        .map(|c| c.get(i).unwrap().clone())
        .collect()
    ).map(|v: Vec<char>| v.iter().collect::<String>()).collect();

    flipped
}


fn count_load(beams: &Vec<String>) -> usize {
    beams.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, line)| {
            acc + line.matches("O").count() * (index + 1)
        })
}


fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();

    let mut previous_layouts: Vec<Vec<String>> = vec![];
    let mut iterations =  0;

    // N
    let mut flipped: Vec<String> = flip(&sample);
    let mut new_layout: Vec<String> = flipped.iter().map(|line| slide_rock(line)).collect();
    // while previous_layouts.find(|e|)


    // W
    let mut flipped: Vec<String> = flip(&sample);
    let mut new_layout: Vec<String> = flipped.iter().map(|line| slide_rock(line)).collect();

    // flip back for west
    // TODO: implement a flip_and_reverse
    // flip back to north and reverse for SOUTH
    // flip back to west and reverse for WEST


    let flipped_back: Vec<String> = flip(&new_layout);

    let res = count_load(&flipped_back);

    res as i64
}


#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 105982);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample14.txt");
        assert_eq!(res, 136);
    }

    #[test]
    fn test_slide_rocks() {
        let res = read_from("src/input/sample14.txt");

        let s = ".O.#.OO".to_string();
        let t = s.split("#");
        let v: Vec<String> = t.map(|v| v.chars().sorted().rev().join("")).collect();
        let u = v.join("#");
        assert_eq!(u, "O..#OO.");
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample14.txt");
        assert_eq!(res, 0);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 0);
    }
}