use crate::utils::divide_iterator_on_predicate;

fn get_totals(file_contents: &str) -> Vec<u64> {
    let entries: Vec<Vec<&str>> = divide_iterator_on_predicate(file_contents.lines(), |&s| s == "");
    let numbers = entries.iter().map(|v| v.iter().map(|s| s.parse::<u64>().expect("invalid number")));
    return numbers.map(|v| v.sum::<u64>()).collect();
}

pub fn part_1(file_contents: &str) -> String {
    return get_totals(file_contents).iter().max().unwrap().to_string();
}

pub fn part_2(file_contents: &str) -> String {
    let mut vec = get_totals(file_contents);
    vec.sort();
    vec.reverse();
    return vec.iter().take(3).sum::<u64>().to_string();
}
