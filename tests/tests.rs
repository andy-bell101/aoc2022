use aoc2022;

#[test]
fn day1_part_1_integration_test() {
    let file_name = "tests/example_files/day1.txt";
    let result = aoc2022::day1::part_1(file_name);
    assert_eq!(result, "24000");
}

#[test]
fn day1_part_2_integration_test() {
    let file_name = "tests/example_files/day1.txt";
    let result = aoc2022::day1::part_2(file_name);
    assert_eq!(result, "45000");
}

#[test]
fn day2_part_1_integration_test() {
    let file_name = "tests/example_files/day2.txt";
    let result = aoc2022::day2::part_1(file_name);
    assert_eq!(result, "15");
}

#[test]
fn day2_part_2_integration_test() {
    let file_name = "tests/example_files/day2.txt";
    let result = aoc2022::day2::part_2(file_name);
    assert_eq!(result, "12");
}
