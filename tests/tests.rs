use aoc2022;

#[test]
fn day_1_part_1integration_test() {
    let file_name = "tests/example_files/day1.txt";
    let result = aoc2022::day1::part_1(file_name);
    assert_eq!(result, "24000");
}

#[test]
fn day_1_part_2_integration_test() {
    let file_name = "tests/example_files/day1.txt";
    let result = aoc2022::day1::part_2(file_name);
    assert_eq!(result, "45000");
}

