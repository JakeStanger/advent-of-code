extern crate regex;

mod year2019;
mod year2020;

use std::env;

fn main() {
    let year: u32 = env::var("YEAR").unwrap().parse().unwrap();
    let day: u32 = env::var("DAY").unwrap().parse().unwrap();

    match (year, day) {
        // 2019
        (2019, 1) => year2019::day01::run(),
        (2019, 2) => year2019::day02::run(),
        (2019, 3) => year2019::day03::run(),
        (2019, 4) => year2019::day04::run(),
        (2019, 5) => year2019::day05::run(),
        (2019, 6) => year2019::day06::run(),
        (2019, 7) => year2019::day07::run(),
        (2019, 8) => year2019::day08::run(),

        // 2020
        (2020, 1) => year2020::day01::run(),
        (2020, 2) => year2020::day02::run(),
        (2020, 3) => year2020::day03::run(),
        (2020, 4) => year2020::day04::run(),

        (_, _) => panic!("invalid year/day"),
    }
}
