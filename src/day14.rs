use itertools::*;
use regex::Captures;
use regex::Regex;

lazy_static! {
    static ref POINT_RE: Regex = Regex::new(r"(\d+),(\d+)").expect("invalid regex");
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Point(usize, usize);

impl Point {
    fn next_sand_location(
        &self,
        walls: &ArrayWithCoords<bool>,
        existing_sand: &ArrayWithCoords<bool>,
    ) -> Option<Self> {
        let Self(x, y) = *self;
        for p in [Point(x, y + 1), Point(x - 1, y + 1), Point(x + 1, y + 1)] {
            if !walls.get_value(p) && !existing_sand.get_value(p) {
                return Some(p);
            }
        }
        return None;
    }
}

const SAND_ORIGIN: Point = Point(500, 0);

#[derive(PartialEq, Eq, Debug)]
struct ArrayWithCoords<T: PartialEq + Eq + Clone> {
    min_point: Point,
    max_point: Point,
    array: Vec<Vec<T>>,
}

impl<T: Clone + PartialEq + Eq + std::fmt::Debug + Copy> ArrayWithCoords<T> {
    fn new(min_point: Point, max_point: Point, init: T) -> Self {
        let Point(min_x, min_y) = min_point;
        let Point(max_x, max_y) = max_point;
        return Self {
            min_point,
            max_point,
            array: vec![vec![init; max_y - min_y + 2]; max_x - min_x + 2],
        };
    }

    fn get_value(&self, point: Point) -> &T {
        return &self.array[point.0 - self.min_point.0][point.1 - self.min_point.1];
    }

    fn set_value(&mut self, point: Point, value: T) -> () {
        self.array[point.0 - self.min_point.0][point.1 - self.min_point.1] = value;
    }
}

fn parse_wall_lines(input: &str) -> Vec<Vec<Point>> {
    let parser = |r: &Captures, i| r.get(i).unwrap().as_str().parse::<usize>().unwrap();
    return input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|p| POINT_RE.captures(p).unwrap())
                .map(|r| Point(parser(&r, 1), parser(&r, 2)))
                .collect()
        })
        .collect();
}

fn range_inclusive(x: usize, y: usize) -> Vec<usize> {
    if x <= y {
        return (x..=y).collect::<Vec<usize>>();
    } else {
        return (y..=x).rev().collect::<Vec<usize>>();
    }
}

fn construct_walls(lines: &Vec<Vec<Point>>) -> ArrayWithCoords<bool> {
    // find minima and maxima
    // sand can form a pyramid that covers the source. The tallest it can be is
    // 2 below the lowest wall. This height also determines the width of the
    // pyramid at the base

    let Point(x, y) = SAND_ORIGIN;
    let min_y = y;
    let max_y = lines
        .iter()
        .map(|line| {
            line.iter()
                .map(|p| {
                    let Point(_x, y) = p;
                    y
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    // subtract one to make sure we don't go out of bounds during simulations
    let min_x = x - max_y - 1;
    let max_x = x + max_y + 1;
    // initialise the grid, with some padding to allow sand to fall off
    // the edges
    let mut array = ArrayWithCoords::new(
        // can't subtract 1 from min_y since it's always 0
        Point(min_x - 1, min_y),
        Point(max_x + 1, max_y + 1),
        false,
    );

    // now flag the walls
    for line in lines.iter() {
        for (p1, p2) in line.iter().tuple_windows() {
            let Point(x1, y1) = *p1;
            let Point(x2, y2) = *p2;
            for i in range_inclusive(x1, x2).iter() {
                for j in range_inclusive(y1, y2).iter() {
                    array.set_value(Point(*i, *j), true);
                }
            }
        }
    }
    return array;
}

fn simulate_sand_particle_dropping(
    walls: &ArrayWithCoords<bool>,
    existing_sand: &mut ArrayWithCoords<bool>,
    allowed_to_touch_floor: bool,
) -> Option<Point> {
    let mut next_sand = SAND_ORIGIN.clone();
    let mut potential_next = next_sand.next_sand_location(&walls, &existing_sand);
    while potential_next.is_some() {
        next_sand = potential_next.unwrap();
        let Point(x, y) = next_sand;
        let Point(min_x, _min_y) = walls.min_point;
        let Point(max_x, max_y) = walls.max_point;
        if !allowed_to_touch_floor && (x == min_x || x == max_x || y == max_y - 1) {
            return None;
        }
        potential_next = next_sand.next_sand_location(&walls, &existing_sand);
    }
    existing_sand.set_value(next_sand, true);
    return Some(next_sand);
}

pub fn part_1(file_contents: &str) -> String {
    let wall_lines = parse_wall_lines(file_contents);
    let walls = construct_walls(&wall_lines);
    let mut existing_sand = ArrayWithCoords::new(walls.min_point, walls.max_point, false);
    let mut counter = 0;
    while simulate_sand_particle_dropping(&walls, &mut existing_sand, false).is_some() {
        counter += 1;
    }
    return counter.to_string();
}

pub fn part_2(file_contents: &str) -> String {
    let wall_lines = parse_wall_lines(file_contents);
    let mut walls = construct_walls(&wall_lines);
    // add the "floor"
    let j = walls.array[0].len();
    for i in 0..walls.array.len() {
        walls.array[i][j - 1] = true;
    }
    let mut existing_sand = ArrayWithCoords::new(walls.min_point, walls.max_point, false);
    let mut counter = 0;
    while let Some(x) = simulate_sand_particle_dropping(&walls, &mut existing_sand, true) {
        counter += 1;
        if x == SAND_ORIGIN {
            break;
        }
    }
    return counter.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://stackoverflow.com/a/64499219
    fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
        assert!(!v.is_empty());
        let len = v[0].len();
        let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
        (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| n.next().unwrap())
                    .collect::<Vec<T>>()
            })
            .collect()
    }

    fn print_array(array: &ArrayWithCoords<bool>) -> () {
        println!(
            "{}",
            transpose(array.array.clone())
                .iter()
                .map(|line| line
                    .iter()
                    .map(|&b| if b { '#' } else { '.' })
                    .collect::<String>())
                .join("\n")
        );
    }

    #[test]
    fn points_parsed_correctly() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let expected = vec![
            vec![Point(498, 4), Point(498, 6), Point(496, 6)],
            vec![Point(503, 4), Point(502, 4), Point(502, 9), Point(494, 9)],
        ];
        assert_eq!(parse_wall_lines(input), expected);
    }

    #[test]
    fn construct_array_with_coords() {
        let result = ArrayWithCoords::new(Point(400, 0), Point(500, 10), false);
        assert_eq!(result.array.len(), 102);
        assert_eq!(result.array[0].len(), 12);
        assert!(result
            .array
            .iter()
            .map(|line| line.iter().all(|&b| b == false))
            .all(|b| b == true));
    }

    #[test]
    fn example_walls_generated_correctly() {
        let input = parse_wall_lines(
            "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
        );
        let mut expected = ArrayWithCoords::new(Point(489, 0), Point(511, 10), false);
        // first wall
        expected.set_value(Point(496, 6), true);
        expected.set_value(Point(497, 6), true);
        expected.set_value(Point(498, 6), true);
        expected.set_value(Point(498, 5), true);
        expected.set_value(Point(498, 4), true);
        // second wall
        expected.set_value(Point(494, 9), true);
        expected.set_value(Point(495, 9), true);
        expected.set_value(Point(496, 9), true);
        expected.set_value(Point(497, 9), true);
        expected.set_value(Point(498, 9), true);
        expected.set_value(Point(499, 9), true);
        expected.set_value(Point(500, 9), true);
        expected.set_value(Point(501, 9), true);
        expected.set_value(Point(502, 9), true);
        expected.set_value(Point(502, 8), true);
        expected.set_value(Point(502, 7), true);
        expected.set_value(Point(502, 6), true);
        expected.set_value(Point(502, 5), true);
        expected.set_value(Point(502, 4), true);
        expected.set_value(Point(503, 4), true);

        print_array(&expected);
        println!("");
        print_array(&construct_walls(&input));

        assert_eq!(construct_walls(&input), expected);
    }
}
