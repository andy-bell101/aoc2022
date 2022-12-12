const TALLEST_POSSIBLE_TREE: u8 = 9;

fn parse_input(contents: &str) -> Vec<Vec<u8>> {
    return contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
}

fn produce_indices(max_x: usize, max_y: usize) -> Vec<Vec<(usize, usize)>> {
    let i1: Vec<Vec<(usize, usize)>> = (0..max_x)
        .map(move |i| (0..max_y).map(move |j| (i, j)).collect())
        .collect();
    let i2: Vec<Vec<(usize, usize)>> = (0..max_x)
        .map(move |i| (0..max_y).map(move |j| (i, j)).rev().collect())
        .rev()
        .collect();
    let i3: Vec<Vec<(usize, usize)>> = (0..max_y)
        .map(move |j| (0..max_x).map(move |i| (i, j)).collect())
        .collect();
    let i4: Vec<Vec<(usize, usize)>> = (0..max_y)
        .map(move |j| (0..max_x).map(move |i| (i, j)).rev().collect())
        .rev()
        .collect();

    let mut v: Vec<Vec<(usize, usize)>> = vec![];
    v.extend(i1);
    v.extend(i2);
    v.extend(i3);
    v.extend(i4);
    return v;
}

fn find_visible_trees(
    marks: &mut Vec<Vec<bool>>,
    trees: &Vec<Vec<u8>>,
    max_x: usize,
    max_y: usize,
) -> () {
    for iter in produce_indices(max_x, max_y).iter() {
        let mut max: isize = -1;
        for (i, j) in iter {
            let t = trees[*i][*j];
            if t as isize > max {
                max = t as isize;
                marks[*i][*j] = true;
            }
            // we won't be finding anymore trees in this case
            if t == TALLEST_POSSIBLE_TREE {
                break;
            }
        }
    }
}

pub fn part_1(file_contents: &str) -> String {
    let trees = parse_input(file_contents);
    let max_x = trees.len();
    let max_y = trees[0].len();
    let mut marks: Vec<Vec<bool>> = vec![vec![false; max_y]; max_x];

    find_visible_trees(&mut marks, &trees, max_x, max_y);

    return marks
        .into_iter()
        .flatten()
        .filter(|m| *m)
        .count()
        .to_string();
}

fn score_line(iter: impl Iterator<Item = (usize, usize)>, trees: &Vec<Vec<u8>>, cur_height: &u8) -> usize {
    let mut acc = 0;
    let mut cur_max: i8 = -1;
    for (i, j) in iter {
        let t = trees[i][j];
        if cur_max <= t as i8 {
            cur_max = t as i8;
            acc += 1;
        }
        if t >= *cur_height {
            break;
        }
    }
    return acc;
}

fn score_tree(trees: &Vec<Vec<u8>>, i: usize, j: usize, max_x: usize, max_y: usize) -> usize {
    let cur_height = trees[i][j];

    let up = score_line((0..i).rev().map(|x| (x, j)), &trees, &cur_height);
    let left = score_line((0..j).rev().map(|x| (i, x)), &trees, &cur_height);
    let right = score_line((j+1..max_y).map(|x| (i, x)), &trees, &cur_height);
    let down = score_line((i+1..max_x).map(|x| (x, j)), &trees, &cur_height);

    return left * right * up * down;
}

fn score_trees(scores: &mut Vec<Vec<usize>>, trees: &Vec<Vec<u8>>, max_x: usize, max_y: usize) -> () {
    // all edges can be skipped, given one of their directions will be 0 and
    // therefore will always score 0
    for i in 1..max_x - 1 {
        for j in 1..max_y - 1 {
            scores[i][j] = score_tree(&trees, i, j, max_x, max_y);
        }
    }
}

pub fn part_2(file_contents: &str) -> String {
    let trees = parse_input(file_contents);
    let max_x = trees.len();
    let max_y = trees[0].len();
    let mut scores: Vec<Vec<usize>> = vec![vec![0; max_y]; max_x];

    score_trees(&mut scores, &trees, max_x, max_y);

    return scores.iter().flatten().max().unwrap().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_produce_iterators() {
        let result: Vec<Vec<(usize, usize)>> = produce_indices(3, 3);
        let expected = vec![
            // first pass forward
            vec![(0, 0), (0, 1), (0, 2)],
            vec![(1, 0), (1, 1), (1, 2)],
            vec![(2, 0), (2, 1), (2, 2)],
            // first pass backward
            vec![(2, 2), (2, 1), (2, 0)],
            vec![(1, 2), (1, 1), (1, 0)],
            vec![(0, 2), (0, 1), (0, 0)],
            // second pass forward
            vec![(0, 0), (1, 0), (2, 0)],
            vec![(0, 1), (1, 1), (2, 1)],
            vec![(0, 2), (1, 2), (2, 2)],
            // second pass backward
            vec![(2, 2), (1, 2), (0, 2)],
            vec![(2, 1), (1, 1), (0, 1)],
            vec![(2, 0), (1, 0), (0, 0)],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mark_trees() {
        let trees: Vec<Vec<u8>> = vec![
            vec![3,0,3,7,3],
            vec![2,5,5,1,2],
            vec![6,5,3,3,2],
            vec![3,3,5,4,9],
            vec![3,5,3,9,0],
        ];
        let expected = vec![
            vec![true, true, true, true, true],
            vec![true, true, true, false, true],
            vec![true, true, false, true, true],
            vec![true, false, true, false, true],
            vec![true, true, true, true, true],
        ];
        let mut marks = vec![vec![false; 5]; 5];
        find_visible_trees(&mut marks, &trees, 5, 5);
        assert_eq!(marks, expected);
    }

    #[test]
    fn test_score_tree() {
        let trees: Vec<Vec<u8>> = vec![
            vec![3,0,3,7,3],
            vec![2,5,5,1,2],
            vec![6,5,3,3,2],
            vec![3,3,5,4,9],
            vec![3,5,3,9,0],
        ];
        assert_eq!(score_tree(&trees, 1, 2, 5, 5), 4);
        assert_eq!(score_tree(&trees, 3, 2, 5, 5), 8);
    }

    #[test]
    fn test_score_trees() {
        let trees: Vec<Vec<u8>> = vec![
            vec![3,0,3,7,3],
            vec![2,5,5,1,2],
            vec![6,5,3,3,2],
            vec![3,3,5,4,9],
            vec![3,5,3,9,0],
        ];
        let expected: Vec<Vec<usize>> = vec![
            vec![0,0,0,0,0],
            vec![0,1,4,1,0],
            vec![0,4,1,2,0],
            vec![0,1,8,2,0],
            vec![0,0,0,0,0],
        ];
        let mut scores: Vec<Vec<usize>> = vec![
            vec![0,0,0,0,0],
            vec![0,0,0,0,0],
            vec![0,0,0,0,0],
            vec![0,0,0,0,0],
            vec![0,0,0,0,0],
        ];
        score_trees(&mut scores, &trees, 5, 5);
        assert_eq!(scores, expected);
    }
}
