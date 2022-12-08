use aoc2022;
use std::include_str;

#[test]
fn day1_part_1_integration_test() {
    let contents = include_str!("example_files/day1.txt");
    let result = aoc2022::day1::part_1(contents);
    assert_eq!(result, "24000");
}

#[test]
fn day1_part_2_integration_test() {
    let contents = include_str!("example_files/day1.txt");
    let result = aoc2022::day1::part_2(contents);
    assert_eq!(result, "45000");
}

#[test]
fn day2_part_1_integration_test() {
    let contents = include_str!("example_files/day2.txt");
    let result = aoc2022::day2::part_1(contents);
    assert_eq!(result, "15");
}

#[test]
fn day2_part_2_integration_test() {
    let contents = include_str!("example_files/day2.txt");
    let result = aoc2022::day2::part_2(contents);
    assert_eq!(result, "12");
}

#[test]
fn day3_part_1_integration_test() {
    let contents = include_str!("example_files/day3.txt");
    let result = aoc2022::day3::part_1(contents);
    assert_eq!(result, "157");
}

#[test]
fn day3_part_2_integration_test() {
    let contents = include_str!("example_files/day3.txt");
    let result = aoc2022::day3::part_2(contents);
    assert_eq!(result, "70");
}

#[test]
fn day4_part_1_integration_test() {
    let contents = include_str!("example_files/day4.txt");
    let result = aoc2022::day4::part_1(contents);
    assert_eq!(result, "2");
}

#[test]
fn day4_part_2_integration_test() {
    let contents = include_str!("example_files/day4.txt");
    let result = aoc2022::day4::part_2(contents);
    assert_eq!(result, "4");
}

#[test]
fn day5_part_1_integration_test() {
    let contents = include_str!("example_files/day5.txt");
    let result = aoc2022::day5::part_1(contents);
    assert_eq!(result, "CMZ");
}

#[test]
fn day5_part_2_integration_test() {
    let contents = include_str!("example_files/day5.txt");
    let result = aoc2022::day5::part_2(contents);
    assert_eq!(result, "MCD");
}

#[test]
fn day6_part_1_integration_test() {
    let contents = include_str!("example_files/day6.txt");
    let result: Vec<String> = contents
        .lines()
        .map(|line| aoc2022::day6::part_1(line))
        .collect();
    assert_eq!(result, vec!["7", "5", "6", "10", "11"]);
}

#[test]
fn day6_part_2_integration_test() {
    let contents = include_str!("example_files/day6.txt");
    let result: Vec<String> = contents
        .lines()
        .map(|line| aoc2022::day6::part_2(line))
        .collect();
    assert_eq!(result, vec!["19", "23", "23", "29", "26"]);
}

#[test]
fn day7_part_1_integration_test() {
    let contents = include_str!("example_files/day7.txt");
    let result = aoc2022::day7::part_1(contents);
    assert_eq!(result, "95437");
}

#[test]
fn day7_part_2_integration_test() {
    let contents = include_str!("example_files/day7.txt");
    let result = aoc2022::day7::part_2(contents);
    assert_eq!(result, "24933642");
}
