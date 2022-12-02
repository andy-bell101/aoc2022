use crate::utils::{get_file_contents, divide_iterator_on_predicate};

pub fn part_1(file_name: &str) -> String {
    let contents = get_file_contents(file_name);
    let entries: Vec<Vec<&str>> = divide_iterator_on_predicate(contents.lines(), |&s| s == "");
    let numbers = entries.iter().map(|v| v.iter().map(|s| s.parse::<u64>().expect("invalid number")));
    return numbers.map(|v| v.sum::<u64>()).max().unwrap().to_string();
}
