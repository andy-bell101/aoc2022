use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CMD_RE: Regex = Regex::new(r"\$").expect("invalid regex");
    static ref CD_RE: Regex = Regex::new(r"\$ cd (\S+)").expect("invalid regex");
    static ref LS_RE: Regex = Regex::new(r"\$ ls").expect("invalid regex");
    static ref DIR_RE: Regex = Regex::new(r"dir (\S+)").expect("invalid regex");
    static ref FILE_RE: Regex = Regex::new(r"(\d+) (\S+)").expect("invalid regex");
}

#[derive(Debug, PartialEq, Eq)]
struct File<'a> {
    name: &'a str,
    size: u32,
}

impl<'a> File<'a> {
    fn new(line: &'a str) -> Self {
        let caps = FILE_RE.captures(line).unwrap();
        return Self {
            name: caps.get(2).unwrap().as_str(),
            size: caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Directory<'a> {
    name: &'a str,
    sub_dirs: Vec<Directory<'a>>,
    files: Vec<File<'a>>,
    size: u32,
}

impl<'a> Directory<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            sub_dirs: vec![],
            files: vec![],
            size: 0,
        }
    }

    fn get_all_sub_dirs(&self) -> Vec<&Directory> {
        return self.sub_dirs.iter().fold(vec![self], |mut acc, d| {
            acc.extend(d.get_all_sub_dirs());
            acc
        });
    }
}

fn process_cmds<'a>(lines: &Vec<&'a str>, start: usize) -> (Directory<'a>, usize) {
    let first_line = lines[start];
    let mut dir: Directory =
        Directory::new(CD_RE.captures(first_line).unwrap().get(1).unwrap().as_str());

    let mut i: usize = start + 1;
    let len = lines.len();
    while i < len {
        let line = lines[i];
        let cd_caps = CD_RE.captures(line);
        if cd_caps.is_some() {
            if cd_caps.unwrap().get(1).unwrap().as_str() != ".." {
                let (d, tmp) = process_cmds(lines, i);
                i = tmp + 1;
                dir.sub_dirs.push(d);
                dir.size += dir.sub_dirs.last().unwrap().size;
                continue;
            } else {
                return (dir, i);
            }
        }
        if FILE_RE.is_match(line) {
            dir.files.push(File::new(line));
            dir.size += dir.files.last().unwrap().size;
            i += 1;
            continue;
        }
        i += 1;
    }
    return (dir, i);
}

pub fn part_1(file_contents: &str) -> String {
    let lines: Vec<&str> = file_contents.lines().collect();
    let (root_dir, _i) = process_cmds(&lines, 0);
    return root_dir
        .get_all_sub_dirs()
        .iter()
        .map(|d| d.size)
        .filter(|&s| s <= 100000)
        .sum::<u32>()
        .to_string();
}

pub fn part_2(file_contents: &str) -> String {
    let lines: Vec<&str> = file_contents.lines().collect();
    let (root_dir, _i) = process_cmds(&lines, 0);
    let target_to_free = root_dir.size - 40000000;
    return root_dir
        .get_all_sub_dirs()
        .iter()
        .map(|d| d.size)
        .filter(|&s| s > target_to_free)
        .min()
        .unwrap()
        .to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_dir_structure() -> Directory<'static> {
        // files
        let i = File {
            name: "i",
            size: 584,
        };
        let f = File {
            name: "f",
            size: 29116,
        };
        let g = File {
            name: "g",
            size: 2557,
        };
        let h_lst = File {
            name: "h.lst",
            size: 62596,
        };
        let b_txt = File {
            name: "b.txt",
            size: 14848514,
        };
        let c_dat = File {
            name: "c.dat",
            size: 8504156,
        };
        let j = File {
            name: "j",
            size: 4060174,
        };
        let d_log = File {
            name: "d.log",
            size: 8033020,
        };
        let d_ext = File {
            name: "d.ext",
            size: 5626152,
        };
        let k = File {
            name: "k",
            size: 7214296,
        };
        // dirs
        let e_dir = Directory {
            name: "e",
            sub_dirs: vec![],
            files: vec![i],
            size: 584,
        };
        let a_dir = Directory {
            name: "a",
            sub_dirs: vec![e_dir],
            files: vec![f, g, h_lst],
            size: 94853,
        };
        let d_dir = Directory {
            name: "d",
            sub_dirs: vec![],
            files: vec![j, d_log, d_ext, k],
            size: 24933642,
        };
        let root = Directory {
            name: "/",
            sub_dirs: vec![a_dir, d_dir],
            files: vec![b_txt, c_dat],
            size: 48381165,
        };
        return root;
    }

    #[test]
    fn constructs_directory_tree() {
        let file_contents = include_str!("../tests/example_files/day7.txt");
        let lines: Vec<&str> = file_contents.lines().collect();
        let (root_dir, _i) = process_cmds(&lines, 0);
        assert_eq!(root_dir, example_dir_structure());
    }
}
