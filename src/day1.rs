use std::io::Read;

pub fn part_1(file_name: &str) -> String {
    let contents = {
        let mut file = std::fs::File::open(file_name).expect("failed to open input file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    };
    let lines = contents.lines();
    let vec: Vec<Vec<u64>>= vec![];
    let entries = lines.fold(vec, |mut acc, s| {
        if s == "" {
            acc.push(vec![]);
            acc
        }
        else{
            let last = match acc.last_mut() {
                Some(x) => x,
                None => {
                    acc.push(vec![]);
                    acc.last_mut().unwrap()
                }
            };
            last.push(s.parse().unwrap());
            acc
        }
    });
    let ans: u64 = entries.iter().map(|v| v.iter().sum()).max().unwrap();
    return ans.to_string();
}
