use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Rucksack<'a> {
    first: &'a str,
    second: &'a str,
}

lazy_static! {
    static ref PRIORITIES: HashMap<char, u32> =
        HashMap::from_iter(('a'..='z').zip(1..27).chain(('A'..='Z').zip(27..53)));
}

fn get_sets(file_contents: &str) -> Vec<(HashSet<char>, HashSet<char>)> {
    let rucksacks = file_contents.lines().map(|line| Rucksack {
        first: &line[..line.len() / 2],
        second: &line[line.len() / 2..],
    });
    let build_set = |s: &str| HashSet::<char>::from_iter(s.chars());
    return rucksacks
        .map(|r| (build_set(r.first), (build_set(r.second))))
        .collect();
}

pub fn part_1(file_contents: &str) -> String {
    let sets = get_sets(file_contents);
    let common_items: Vec<HashSet<char>> = sets
        .iter()
        // .map(|(s1, s2)| s1.intersection(&s2).cloned().collect())
        .map(|(s1, s2)| s1 & s2)
        .collect();
    return common_items
        .iter()
        .map(|s| s.iter().map(|c| PRIORITIES[c]).sum::<u32>())
        .sum::<u32>()
        .to_string();
}

pub fn part_2(file_contents: &str) -> String {
    let sets = get_sets(file_contents);
    let united_sets: Vec<HashSet<char>> = sets.iter().map(|(s1, s2)| s1 | s2).collect();
    let badges = united_sets.chunks(3).map(|v| {
        // using reduce here would make more sense but I couldn't get it to
        // work
        v.iter().fold(HashSet::new(), |acc, s| {
            if acc.is_empty() {
                s.clone()
            } else {
                &acc & s
            }
        })
    });
    return badges
        .map(|s| s.iter().map(|c| PRIORITIES[c]).sum::<u32>())
        .sum::<u32>()
        .to_string();
}
