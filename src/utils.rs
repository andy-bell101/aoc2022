use std::io::Read;

pub fn get_file_contents(file_name: &str) -> String {
    let mut file = std::fs::File::open(file_name).expect("failed to open input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn divide_iterator_on_predicate<'a, I, P, T>(input: I, pred: P) -> Vec<Vec<T>>
where
    I: IntoIterator<Item = T>,
    P: Fn(&T) -> bool,
{
    let vec: Vec<Vec<T>> = vec![];
    let entries = input.into_iter().fold(vec, |mut acc, s| {
        if pred(&s) {
            acc.push(vec![]);
            acc
        } else {
            let last = match acc.last_mut() {
                Some(x) => x,
                None => {
                    acc.push(vec![]);
                    acc.last_mut().unwrap()
                }
            };
            last.push(s);
            acc
        }
    });
    return entries;
}
