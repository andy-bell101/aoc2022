mod utils;
pub mod day1;

fn run_day1() {
    println!("Day 1, Part 1: {}", day1::part_1("input_files/day1.txt"));
    println!("Day 1, Part 2: {}", day1::part_2("input_files/day1.txt"));
}

fn main() {
    run_day1();
}
