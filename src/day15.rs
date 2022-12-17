use crate::utils::{range_inclusive, Point};
use regex::Captures;
use regex::Regex;

lazy_static! {
    static ref SENSOR_RE: Regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .expect("invalid regex");
}

#[derive(PartialEq, Eq, Debug)]
enum BeaconExists {
    Yes,
    No,
    Maybe,
}

impl Point {
    fn manhattan_distance(&self, other: &Self) -> isize {
        let Point(x1, y1) = self;
        let Point(x2, y2) = other;
        return (x2 - x1).abs() + (y2 - y1).abs();
    }
}

struct Sensor {
    location: Point,
    nearest_beacon: Point,
    manhattan_distance: isize,
}

impl Sensor {
    fn new(location: Point, nearest_beacon: Point) -> Self {
        let manhattan_distance = location.manhattan_distance(&nearest_beacon);
        return Self {
            location,
            nearest_beacon,
            manhattan_distance,
        };
    }

    fn beacon_at_location(&self, point: &Point) -> BeaconExists {
        if self.nearest_beacon == *point {
            return BeaconExists::Yes;
        } else if self.location.manhattan_distance(point) <= self.manhattan_distance {
            return BeaconExists::No;
        } else {
            return BeaconExists::Maybe;
        }
    }

    fn points_just_outside_circle(&self) -> Vec<Point> {
        let radius = self.manhattan_distance + 1;
        return range_inclusive(0, radius)
            .iter()
            .chain(range_inclusive(radius - 1, -radius).iter())
            .zip(-radius..radius)
            .map(|(i, j)| Point(*i + self.location.0, j + self.location.1))
            .collect();
    }
}

fn parse_lines(input: &str) -> (Vec<Sensor>, Point, Point) {
    let caps: Vec<Captures> = input
        .lines()
        .map(|line| SENSOR_RE.captures(line).unwrap())
        .collect();
    let parse_num = |c: &Captures, i: usize| c.get(i).unwrap().as_str().parse::<isize>().unwrap();
    let mut vec: Vec<Sensor> = vec![];
    let mut min_x: isize = isize::max_value();
    let mut min_y: isize = isize::max_value();
    let mut max_x: isize = isize::min_value();
    let mut max_y: isize = isize::min_value();
    let mut max_manhattan: isize = 0;
    for cap in caps.iter() {
        let sx = parse_num(cap, 1);
        let sy = parse_num(cap, 2);
        let bx = parse_num(cap, 3);
        let by = parse_num(cap, 4);

        let p = Sensor::new(Point(sx, sy), Point(bx, by));
        if max_manhattan < p.manhattan_distance {
            max_manhattan = p.manhattan_distance;
        }
        vec.push(p);

        if sx < min_x {
            min_x = sx;
        }
        if sx > max_x {
            max_x = sx;
        }

        if bx < min_x {
            min_x = bx;
        }
        if bx > max_x {
            max_x = bx;
        }

        if sy < min_y {
            min_y = sy;
        }
        if sy > max_y {
            max_y = sy;
        }

        if by < min_y {
            max_y = by;
        }
        if by > max_y {
            max_y = by;
        }
    }
    return (
        vec,
        Point(min_x - max_manhattan, min_y - max_manhattan),
        Point(max_x + max_manhattan, max_y + max_manhattan),
    );
}

pub fn part_1_solver(file_contents: &str, target_y: isize) -> String {
    let (sensors, min_point, max_point) = parse_lines(file_contents);
    let Point(min_x, _min_y) = min_point;
    let Point(max_x, _max_y) = max_point;
    let mut counter = 0;
    for i in min_x..max_x {
        if sensors
            .iter()
            .map(|s| s.beacon_at_location(&Point(i, target_y)))
            .any(|b| b == BeaconExists::No)
        {
            counter += 1;
        }
    }
    return counter.to_string();
}

pub fn part_1(file_contents: &str) -> String {
    return part_1_solver(file_contents, 2_000_000);
}

pub fn part_2_solver(file_contents: &str, coord_limit: isize) -> String {
    let (sensors, _min_point, _max_point) = parse_lines(file_contents);
    let mut point: Option<Point> = None;
    let potential_points = sensors.iter().flat_map(|s| s.points_just_outside_circle()).filter(|p| {
        let Point(x, y) = p;
        &0 <= x && x <= &coord_limit && &0 < y && y <= &coord_limit
    });
    for p in potential_points {
        if sensors.iter().all(|s| s.beacon_at_location(&p) == BeaconExists::Maybe) {
            point = Some(p);
            break;
        }
    }
    let Point(x, y) = point.expect("could not find valid point!");
    return (x * 4_000_000 + y).to_string();
}

pub fn part_2(file_contents: &str) -> String {
    return part_2_solver(file_contents, 4_000_000);
}
