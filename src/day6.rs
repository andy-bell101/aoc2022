use std::collections::HashSet;

fn find_unique_marker_of_length(file_contents: &str, length: usize) -> String {
    let mut index = None;
    for (i, _c) in file_contents.chars().enumerate() {
        let set: HashSet<char> = HashSet::from_iter(file_contents[i..i + length].chars());
        if set.len() == length {
            index = Some(i + length);
            break;
        }
    }
    return index
        .expect(format!("failed to find unique string of {} chars", length).as_str())
        .to_string();
}

pub fn part_1(file_contents: &str) -> String {
    return find_unique_marker_of_length(file_contents, 4);
}

pub fn part_2(file_contents: &str) -> String {
    return find_unique_marker_of_length(file_contents, 14);
}
