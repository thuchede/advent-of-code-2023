use std::time::{Instant};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod helpers;

fn main() {
    let time_for_part1 = Instant::now();
    let part1 = day08::part_1();
    println!("{}", part1);
    println!("Done in {}ms", time_for_part1.elapsed().as_millis());
    let time_for_part2 = Instant::now();
    let part2 = day08::part_2();
    println!("{}", part2);
    println!("Done in {}ms", time_for_part2.elapsed().as_millis());
}
