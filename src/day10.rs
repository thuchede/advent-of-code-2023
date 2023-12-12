use itertools::Itertools;

use crate::helpers;


#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day10.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day10.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let pipe_map: Vec<Vec<Option<PipeShape>>> = sample.iter()
        .map(|line| parse_line(line).iter().map(|&c| char_to_pipe_shape(c)).collect())
        .collect();

    let start = get_starting_position(&pipe_map);
    let mut path: Vec<(i64, i64)> = vec![];
    // try North
    let loop_starting_north = follow_path_until_loop(start, &pipe_map, Direction::North);
    if let Some(north) = loop_starting_north {
        path = north;
    } else {
        // try East
        let loop_starting_east = follow_path_until_loop(start, &pipe_map, Direction::East);
        if let Some(east) = loop_starting_east {
            path = east;
        } else {
            let loop_starting_south = follow_path_until_loop(start, &pipe_map, Direction::South);
            if let Some(south) = loop_starting_south {
                path = south;
            }
        }
    }
    // No need to try West as we cannot fail for 3/4, we need at least two entrypoint

    (path.len() / 2) as i64
}


fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let pipe_map: Vec<Vec<Option<PipeShape>>> = sample.iter()
        .map(|line| parse_line(line).iter().map(|&c| char_to_pipe_shape(c)).collect())
        .collect();

    let start = get_starting_position(&pipe_map);
    let mut path: Vec<(i64, i64)> = vec![];
    // try North
    let loop_starting_north = follow_path_until_loop(start, &pipe_map, Direction::North);
    if let Some(north) = loop_starting_north {
        path = north;
    } else {
        // try East
        let loop_starting_east = follow_path_until_loop(start, &pipe_map, Direction::East);
        if let Some(east) = loop_starting_east {
            path = east;
        } else {
            let loop_starting_south = follow_path_until_loop(start, &pipe_map, Direction::South);
            if let Some(south) = loop_starting_south {
                path = south;
            }
        }
    }
    // No need to try West as we cannot fail for 3/4, we need at least two entrypoint
    let mut path_loop = path.clone();
    path_loop.push(*path_loop.first().unwrap());
    // Using Gauss area theorem to get the area of the path
    let area: i64 = path_loop.clone().iter().tuple_windows().map(|(&u, &v)| shoelace_multiplication(u, v)).sum::<i64>() / 2;
    let nb_nodes = path.len() as i64;

    // Using Pick theorem to get the number of nodes contained within the area
    let inner_nodes = area.abs() - (nb_nodes / 2) + 1;
    inner_nodes
}


// Option<PipeShape> => None for '.'
#[derive(Debug, PartialEq, Eq)]
enum PipeShape {
    SouthNorth,
    SouthEast,
    SouthWest,
    EastWest,
    NorthEast,
    NorthWest,
    Start,
}

fn parse_line(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn char_to_pipe_shape(pipe: char) -> Option<PipeShape> {
    match pipe {
        '|' => Some(PipeShape::SouthNorth),
        'F' => Some(PipeShape::SouthEast),
        '7' => Some(PipeShape::SouthWest),
        '-' => Some(PipeShape::EastWest),
        'L' => Some(PipeShape::NorthEast),
        'J' => Some(PipeShape::NorthWest),
        'S' => Some(PipeShape::Start),
        _ => None
    }
}

fn get_starting_position(pipes: &[Vec<Option<PipeShape>>]) -> (i64, i64) {
    let res = pipes.iter().enumerate().find_map(|(y_idx, line)| {
        let find_pos: Option<(usize, &Option<PipeShape>)> = line.iter().find_position(|&pipe| matches!(pipe, Some(PipeShape::Start)));
        match find_pos {
            Some((x_idx, _)) => Some((y_idx as i64, x_idx as i64)),
            _ => None
        }
    });
    let (y_pos, x_pos) = res.unwrap();

    (y_pos, x_pos)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn follow_path_until_loop((initial_y, initial_x): (i64, i64), pipes: &Vec<Vec<Option<PipeShape>>>, original_direction: Direction) -> Option<Vec<(i64, i64)>> {
    let max_x = pipes.first().unwrap().len() as i64;
    let max_y = pipes.len() as i64;
    let mut acc = vec![(initial_y, initial_x)];

    let mut new_pos_x = initial_x;
    let mut new_pos_y = initial_y;
    let mut next_item_at_position: &Option<PipeShape> = &None; // we dont know at start what the next node is gonna be
    let mut direction = original_direction;

    while *next_item_at_position != Some(PipeShape::Start) {
        match direction {
            Direction::North => {
                new_pos_y = new_pos_y - 1;
                if new_pos_y >= 0 {
                    let next = (new_pos_y, new_pos_x);
                    let next_item = pipes.get(new_pos_y as usize).unwrap().get(new_pos_x as usize).unwrap();
                    next_item_at_position = next_item;
                    match next_item {
                        Some(PipeShape::SouthNorth) => {
                            acc.push(next);
                            direction = Direction::North;
                        }
                        Some(PipeShape::SouthWest) => {
                            acc.push(next);
                            direction = Direction::West;
                        }
                        Some(PipeShape::SouthEast) => {
                            acc.push(next);
                            direction = Direction::East;
                        }
                        Some(PipeShape::Start) => { return Some(acc.clone()); }
                        _ => { return None; }
                    }
                } else {
                    return None;
                }
            }
            Direction::South => {
                new_pos_y = new_pos_y + 1;
                if new_pos_y < max_y {
                    let next = (new_pos_y, new_pos_x);
                    let next_item = pipes.get(new_pos_y as usize).unwrap().get(new_pos_x as usize).unwrap();
                    next_item_at_position = next_item;
                    match next_item {
                        Some(PipeShape::SouthNorth) => {
                            acc.push(next);
                            direction = Direction::South;
                        }
                        Some(PipeShape::NorthEast) => {
                            acc.push(next);
                            direction = Direction::East;
                        }
                        Some(PipeShape::NorthWest) => {
                            acc.push(next);
                            direction = Direction::West;
                        }
                        Some(PipeShape::Start) => { return Some(acc.clone()); }
                        _ => { return None; }
                    }
                } else {
                    return None;
                }
            }
            Direction::West => {
                new_pos_x = new_pos_x - 1;
                if new_pos_x >= 0 {
                    let next = (new_pos_y, new_pos_x);
                    let next_item = pipes.get(new_pos_y as usize).unwrap().get(new_pos_x as usize).unwrap();
                    next_item_at_position = next_item;
                    match next_item {
                        Some(PipeShape::EastWest) => {
                            acc.push(next);
                            direction = Direction::West;
                        }
                        Some(PipeShape::NorthEast) => {
                            acc.push(next);
                            direction = Direction::North;
                        }
                        Some(PipeShape::SouthEast) => {
                            acc.push(next);
                            direction = Direction::South;
                        }
                        Some(PipeShape::Start) => { return Some(acc.clone()); }
                        _ => { return None; }
                    }
                } else {
                    return None;
                }
            }
            Direction::East => {
                new_pos_x = new_pos_x + 1;
                if new_pos_x < max_x {
                    let next = (new_pos_y, new_pos_x);
                    let next_item = pipes.get(new_pos_y as usize).unwrap().get(new_pos_x as usize).unwrap();
                    next_item_at_position = next_item;
                    match next_item {
                        Some(PipeShape::EastWest) => {
                            acc.push(next);
                            direction = Direction::East;
                        }
                        Some(PipeShape::NorthWest) => {
                            acc.push(next);
                            direction = Direction::North;
                        }
                        Some(PipeShape::SouthWest) => {
                            acc.push(next);
                            direction = Direction::South;
                        }
                        Some(PipeShape::Start) => { return Some(acc.clone()); }
                        _ => { return None; }
                    }
                } else {
                    return None;
                }
            }
        };
    }
    None
}

fn shoelace_multiplication((vy, vx): (i64, i64), (uy, ux): (i64, i64)) -> i64 {
    vx * uy - vy * ux
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 7066);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample10.txt");
        assert_eq!(res, 8);
        let res = read_from("src/input/sample10b.txt");
        assert_eq!(res, 4);
    }

    #[test]
    fn test_parse_line() {
        let res = parse_line(".L-F.");
        assert_eq!(res, vec!['.', 'L', '-', 'F', '.']);
    }

    #[test]
    fn test_char_to_direction() {
        assert_eq!(char_to_pipe_shape('|'), Some(PipeShape::SouthNorth));
        assert_eq!(char_to_pipe_shape('F'), Some(PipeShape::SouthEast));
        assert_eq!(char_to_pipe_shape('7'), Some(PipeShape::SouthWest));
        assert_eq!(char_to_pipe_shape('-'), Some(PipeShape::EastWest));
        assert_eq!(char_to_pipe_shape('L'), Some(PipeShape::NorthEast));
        assert_eq!(char_to_pipe_shape('J'), Some(PipeShape::NorthWest));
        assert_eq!(char_to_pipe_shape('S'), Some(PipeShape::Start));
        assert_eq!(char_to_pipe_shape('.'), None);
    }

    #[test]
    fn test_get_starting_position() {
        let sample = helpers::read("src/input/sample10.txt").unwrap();
        let pipe_map: Vec<Vec<Option<PipeShape>>> = sample.iter()
            .map(|line| parse_line(line).iter().map(|&c| char_to_pipe_shape(c)).collect())
            .collect();
        let res = get_starting_position(&pipe_map);
        assert_eq!(res, (2, 0));
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample10c.txt");
        assert_eq!(res, 4);
    }

    #[test]
    fn test_read_from_v2_b() {
        let res = read_from_v2("src/input/sample10d.txt");
        assert_eq!(res, 28);
    }

    #[test]
    fn test_read_from_v2_c() {
        let res = read_from_v2("src/input/sample10e.txt");
        assert_eq!(res, 9);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 401);
    }
}