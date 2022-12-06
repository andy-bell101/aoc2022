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
