use crate::helpers;

// ____________________
// Part 1
// ____________________

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day01.txt")
}

fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath);
    sample.unwrap().iter().map(|line| get_coordinates(line)).reduce(|a, b| a + b).unwrap()
}

fn get_coordinates(amended_coordinates: &str) -> i64 {
    let first = amended_coordinates.chars().find(|c| { c.is_numeric() });
    let last = amended_coordinates.chars().rev().find(|c| { c.is_numeric() });
    if let (Some(f), Some(l)) = (first, last) {
        (f.to_string() + l.to_string().as_str()).parse().unwrap()
    } else {
        0
    }
}

// ____________________
// Part 2
// ____________________

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day01.txt")
}

fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath);
    sample.unwrap().iter().map(|line| get_coordinates_v2(line)).reduce(|a, b| a + b).unwrap()
}

fn get_coordinates_v2(amended_coordinates_with_txt: &str) -> i64 {
    // replace to easily match numbers
    // dup text-based version so we don't strip a letter
    // e.g. eightwo => e8t2o
    let amended_coordinates = amended_coordinates_with_txt
        .replace("one", "o1e")
        .replace("two", "t2t")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    return get_coordinates(amended_coordinates.as_str());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_1_part_1_sample() {
        let res = read_from("src/input/sample01.txt");
        assert_eq!(res, 142);
    }

    #[test]
    fn test_day1_1_get_coordinates() {
        let res1 = get_coordinates("abc12ds");
        assert_eq!(res1, 12);
        let res2 = get_coordinates("abc7ds");
        assert_eq!(res2, 77);
        let res3 = get_coordinates("ab1c23d4s");
        assert_eq!(res3, 14);
    }

    #[test]
    fn test_day1_2_get_coordinates_v2() {
        let res1 = get_coordinates_v2("two1nine");
        assert_eq!(res1, 29);
        let res2 = get_coordinates_v2("eightwo");
        assert_eq!(res2, 82);
        let res3 = get_coordinates_v2("abcone2threexyz");
        assert_eq!(res3, 13);
    }

    #[test]
    fn test_day1_2() {
        let res = part_2();
        assert_eq!(res, 53221);
    }
}