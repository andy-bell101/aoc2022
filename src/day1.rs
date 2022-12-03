use crate::utils::{get_file_contents, divide_iterator_on_predicate};

fn get_totals(file_name: &str) -> Vec<u64> {
    let contents = get_file_contents(file_name);
    let entries: Vec<Vec<&str>> = divide_iterator_on_predicate(contents.lines(), |&s| s == "");
    let numbers = entries.iter().map(|v| v.iter().map(|s| s.parse::<u64>().expect("invalid number")));
    return numbers.map(|v| v.sum::<u64>()).collect();
}

pub fn part_1(file_name: &str) -> String {
    return get_totals(file_name).iter().max().unwrap().to_string();
}

pub fn part_2(file_name: &str) -> String {
    let mut vec = get_totals(file_name);
    vec.sort();
    vec.reverse();
    return vec.iter().take(3).sum::<u64>().to_string();
}
