use std::convert::TryFrom;

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

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Point(pub isize, pub isize);

#[derive(PartialEq, Eq, Debug)]
pub struct Grid<T: PartialEq + Eq + Clone> {
    pub min_point: Point,
    pub max_point: Point,
    pub array: Vec<Vec<T>>,
}

impl<T: Clone + PartialEq + Eq + std::fmt::Debug + Copy> Grid<T> {
    pub fn new(min_point: Point, max_point: Point, init: T) -> Self {
        let Point(min_x, min_y) = min_point;
        let Point(max_x, max_y) = max_point;
        return Self {
            min_point,
            max_point,
            array: vec![vec![init; (max_y - min_y) as usize + 2]; (max_x - min_x) as usize + 2],
        };
    }

    pub fn get_value(&self, point: Point) -> &T {
        return &self.array[(point.0 - self.min_point.0) as usize][(point.1 - self.min_point.1) as usize];
    }

    pub fn set_value(&mut self, point: Point, value: T) -> () {
        self.array[(point.0 - self.min_point.0) as usize][(point.1 - self.min_point.1) as usize] = value;
    }
}
