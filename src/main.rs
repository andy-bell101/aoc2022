#[macro_use]
extern crate lazy_static;

mod utils;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

macro_rules! runner {
    ($($module:tt),*) => (
        let mut i = 0;
        $(
            i += 1;
            println!("Day {}, Part 1: {}", i, $module::part_1(format!("input_files/day{}.txt", i).as_str()));
            println!("Day {}, Part 2: {}", i, $module::part_2(format!("input_files/day{}.txt", i).as_str()));
        )*
    )
}

fn main() {
    runner!(
        day1, 
        day2,
        day3,
        day4,
        day5,
        day6,
        day7,
        day8,
        day9,
        day10,
        day11,
        day12,
        day13,
        day14,
        day15,
        day16,
        day17,
        day18,
        day19,
        day20,
        day21,
        day22,
        day23,
        day24,
        day25
    );
}
