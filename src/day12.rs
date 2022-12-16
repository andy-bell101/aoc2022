use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Point(usize, usize, char);

fn find_start_and_end_positions(surf: &Vec<Vec<char>>) -> (Point, Point) {
    let mut start = None;
    let mut end = None;
    for (i, v) in surf.iter().enumerate() {
        for (j, c) in v.iter().enumerate() {
            if *c == 'S' {
                start = Some(Point(i, j, *c));
            } else if *c == 'E' {
                end = Some(Point(i, j, *c));
            }

            if start.is_some() && end.is_some() {
                break;
            }
        }
        if start.is_some() && end.is_some() {
            break;
        }
    }
    return (
        start.expect("failed to find start point"),
        end.expect("failed to find end point"),
    );
}

fn point_in_bounds(x: &isize, y: &isize, max_x: usize, max_y: usize) -> bool {
    return *x >= 0 && *x < (max_x as isize) && *y >= 0 && *y < (max_y as isize);
}

fn valid_step_in_chars(source_char: char, target_char: char) -> bool {
    let convert_char = |c| {
        (if c == 'S' {
            'a'
        } else if c == 'E' {
            'z'
        } else {
            c
        }) as i32
    };
    let source = convert_char(source_char);
    let target = convert_char(target_char);
    return target - source <= 1;
}

fn determine_valid_neighbours(
    surf: &Vec<Vec<char>>,
    cur: Point,
    max_x: usize,
    max_y: usize,
) -> Vec<Point> {
    let Point(x, y, in_char) = cur;
    return [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .map(|(i, j)| (x as isize + i, y as isize + j))
        .filter(|(x, y)| point_in_bounds(x, y, max_x, max_y))
        .map(|(x, y)| (x as usize, y as usize, surf[x as usize][y as usize]))
        .filter(|(_x, _y, c)| valid_step_in_chars(in_char, *c))
        .map(|(x, y, c)| Point(x, y, c))
        .collect();
}

fn breadth_first_search_single_iteration(
    surf: &Vec<Vec<char>>,
    end: &Point,
    visited: &mut Vec<Vec<bool>>,
    distances: &mut Vec<Vec<Option<usize>>>,
    queue: &mut VecDeque<Point>,
    predecessors: &mut Vec<Vec<Option<Point>>>,
    max_x: usize,
    max_y: usize,
) -> Option<usize> {
    let cur_point = queue.pop_front().unwrap();
    let Point(x, y, _c) = cur_point;
    let cur_distance = distances[x][y].unwrap_or(0);
    let neighbours = determine_valid_neighbours(&surf, cur_point, max_x, max_y);
    for point in neighbours.iter() {
        if point == end {
            return Some(cur_distance + 1);
        }
        let Point(nx, ny, _c) = point;
        if visited[*nx][*ny] {
            continue;
        }
        visited[*nx][*ny] = true;
        distances[*nx][*ny] = Some(cur_distance + 1);
        predecessors[*nx][*ny] = Some(cur_point);
        queue.push_back(*point);
    }
    return None;
}

fn breadth_first_search(
    surf: &Vec<Vec<char>>,
    start: &Point,
    end: &Point,
    max_x: usize,
    max_y: usize,
) -> Option<usize> {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; max_y]; max_x];
    let mut distances: Vec<Vec<Option<usize>>> = vec![vec![None; max_y]; max_x];
    let mut queue: VecDeque<Point> = VecDeque::from_iter([*start].iter().cloned());
    let mut predecessors: Vec<Vec<Option<Point>>> = vec![vec![None; max_y]; max_x];
    while !queue.is_empty() {
        match breadth_first_search_single_iteration(
            &surf,
            &end,
            &mut visited,
            &mut distances,
            &mut queue,
            &mut predecessors,
            max_x,
            max_y,
        ) {
            Some(x) => return Some(x),
            None => (),
        }
    }
    return None;
}

pub fn part_1(file_contents: &str) -> String {
    let surf: Vec<Vec<char>> = file_contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let max_x = surf.len();
    let max_y = surf[0].len();
    let (start, end) = find_start_and_end_positions(&surf);
    let distance = breadth_first_search(&surf, &start, &end, max_x, max_y);
    return distance.unwrap().to_string();
}

pub fn part_2(file_contents: &str) -> String {
    let surf: Vec<Vec<char>> = file_contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let max_x = surf.len();
    let max_y = surf[0].len();
    let (start, end) = find_start_and_end_positions(&surf);
    let possible_starts = surf
        .iter()
        .enumerate()
        .flat_map(move |(i, line)| line.iter().enumerate().map(move |(j, c)| (i, j, c)))
        .filter(|(_i, _j, &c)| c == 'a')
        .map(|(i, j, c)| Point(i, j, *c))
        .chain([start]);
    let distance = possible_starts
        .filter_map(|s| breadth_first_search(&surf, &s, &end, max_x, max_y))
        .min();
    return distance.unwrap().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const EXAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    const EXAMPLE_MAX_X: usize = 5;
    const EXAMPLE_MAX_Y: usize = 8;
    lazy_static! {
        static ref EXAMPLE_SURF: Vec<Vec<char>> = EXAMPLE_INPUT
            .lines()
            .map(|line| line.chars().collect())
            .collect();
    }

    #[test]
    fn finds_start_and_end_in_example_surface() {
        let expected_start = Point(0, 0, 'S');
        let expected_end = Point(2, 5, 'E');
        let (result_start, result_end) = find_start_and_end_positions(&EXAMPLE_SURF);
        assert_eq!(result_start, expected_start);
        assert_eq!(result_end, expected_end);
    }

    #[test]
    fn valid_neighbours_are_returned() {
        let input_point = Point(0, 0, 'S');
        let expected = HashSet::from([Point(0, 1, 'a'), Point(1, 0, 'a')]);
        let result =
            determine_valid_neighbours(&EXAMPLE_SURF, input_point, EXAMPLE_MAX_X, EXAMPLE_MAX_Y);
        assert_eq!(HashSet::from_iter(result.iter().cloned()), expected);

        let input_point = Point(1, 1, 'b');
        let expected = HashSet::from([
            Point(1, 2, 'c'),
            Point(0, 1, 'a'),
            Point(1, 0, 'a'),
            Point(2, 1, 'c'),
        ]);
        let result =
            determine_valid_neighbours(&EXAMPLE_SURF, input_point, EXAMPLE_MAX_X, EXAMPLE_MAX_Y);
        assert_eq!(HashSet::from_iter(result.iter().cloned()), expected);

        let input_point = Point(2, 4, 'z');
        let expected = HashSet::from([
            Point(2, 5, 'E'),
            Point(1, 4, 'y'),
            Point(2, 3, 's'),
            Point(3, 4, 'u'),
        ]);
        let result =
            determine_valid_neighbours(&EXAMPLE_SURF, input_point, EXAMPLE_MAX_X, EXAMPLE_MAX_Y);
        assert_eq!(HashSet::from_iter(result.iter().cloned()), expected);

        let input_point = Point(2, 6, 'x');
        let expected = HashSet::from([Point(2, 7, 'k'), Point(1, 6, 'x'), Point(3, 6, 'w')]);
        let result =
            determine_valid_neighbours(&EXAMPLE_SURF, input_point, EXAMPLE_MAX_X, EXAMPLE_MAX_Y);
        assert_eq!(HashSet::from_iter(result.iter().cloned()), expected);
    }
}
