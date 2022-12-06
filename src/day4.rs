use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").expect("invalid regex");
}

#[derive(Debug)]
struct Pair {
    first: HashSet<u32>,
    second: HashSet<u32>,
}

fn build_pairs<'a>(file_contents: &'a str) -> impl Iterator<Item = Pair> + 'a {
    return file_contents.lines().map(|line| {
        let c = RE.captures(line).unwrap();
        let p = |s: &str| s.parse::<u32>().unwrap();
        Pair {
            first: HashSet::from_iter(p(&c[1])..=p(&c[2])),
            second: HashSet::from_iter(p(&c[3])..=p(&c[4])),
        }
    });
}

pub fn part_1(file_contents: &str) -> String {
    let pairs = build_pairs(&file_contents);
    return pairs
        .filter(
            |Pair {
                 first: a,
                 second: b,
             }| a.is_subset(b) || b.is_subset(a),
        )
        .count()
        .to_string();
}

pub fn part_2(file_contents: &str) -> String {
    let pairs = build_pairs(&file_contents);
    return pairs
        .filter(|Pair { first, second }| !(first & second).is_empty())
        .count()
        .to_string();
}
