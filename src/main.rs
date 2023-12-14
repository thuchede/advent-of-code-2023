use std::time::{Instant};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod helpers;

fn main() {
    let time_for_part1 = Instant::now();
    let part1 = day12::part_1();
    println!("{}", part1);
    println!("Done in {}ms", time_for_part1.elapsed().as_millis());
    let time_for_part2 = Instant::now();
    // let part2 = day12::part_2();
    println!("Done in {}ms", time_for_part2.elapsed().as_millis());
}
