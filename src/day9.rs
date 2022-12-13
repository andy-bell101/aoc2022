use impl_ops::*;
use regex::Regex;
use std::collections::HashSet;
use std::ops;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\w) (\d+)").expect("invalid regex");
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Vector(isize, isize);

impl Vector {
    fn r#move(&self, direction: &Self) -> Self {
        return Self(self.0 + direction.0, self.1 + direction.1);
    }

    fn move_mutable(&mut self, direction: &Self) -> () {
        self.0 += direction.0;
        self.1 += direction.1;
    }

    fn is_touching(&self, other: &Self) -> bool {
        let x = (self.0 - other.0).abs();
        let y = (self.1 - other.1).abs();
        return x <= 1 && y <= 1;
    }

    fn determine_tail_vector(&self) -> Self {
        let x = if self.0 == 0 {
            0
        } else {
            self.0 / self.0 * self.0.signum()
        };
        let y = if self.1 == 0 {
            0
        } else {
            self.1 / self.1 * self.1.signum()
        };
        return Self(x, y);
    }
}

impl_op_ex!(+ |a: &Vector, b: &Vector| -> Vector {Vector(a.0 + b.0, a.1 + b.1)});
impl_op_ex!(-|a: &Vector, b: &Vector| -> Vector { Vector(a.0 - b.0, a.1 - b.1) });

#[derive(PartialEq, Eq, Debug)]
struct Motion {
    direction: Direction,
    vector: Vector,
    amount: usize,
}

impl Motion {
    fn new(line: &str) -> Self {
        let caps = RE.captures(line).unwrap();
        let (direction, vector) = match caps.get(1).unwrap().as_str() {
            "L" => (Direction::Left, Vector(-1, 0)),
            "R" => (Direction::Right, Vector(1, 0)),
            "U" => (Direction::Up, Vector(0, 1)),
            "D" => (Direction::Down, Vector(0, -1)),
            _ => panic!("Unrecognised input!"),
        };
        let amount = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        return Self {
            direction,
            vector,
            amount,
        };
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Rope {
    knots: Vec<Vector>,
}

impl Rope {
    fn new(length: usize) -> Self {
        return Self {
            knots: vec![Vector(0, 0); length],
        };
    }

    fn r#move(&self, motion: &Motion) -> (Self, HashSet<Vector>) {
        let Motion {
            direction: _,
            vector,
            amount,
        } = motion;
        let mut knots = self.knots.clone();
        let mut set = HashSet::from([*self.knots.last().unwrap()]);
        for _i in 0..*amount {
            let mut new_knots: Vec<Vector> = vec![];
            {
                let head = knots.get_mut(0).unwrap();
                head.move_mutable(&vector);
                new_knots.push(*head);
            }
            for t in knots.iter().skip(1) {
                let h = new_knots.last().unwrap();
                if !h.is_touching(&t) {
                    let diff = h - t;
                    let tail_vector = diff.determine_tail_vector();
                    new_knots.push(t.r#move(&tail_vector));
                } else {
                    new_knots.push(*t);
                }
            }
            set.insert(*new_knots.last().unwrap());
            knots = new_knots;
        }
        return (Self { knots }, set);
    }
}

fn parse_input(contents: &str) -> Vec<Motion> {
    return contents.lines().map(|line| Motion::new(line)).collect();
}

fn solver(file_contents: &str, rope_length: usize) -> String {
    let motions = parse_input(file_contents);
    let (_r, visited) =
        motions
            .iter()
            .fold((Rope::new(rope_length), HashSet::new()), |(r, s), m| {
                let (new_r, new_s) = r.r#move(&m);
                (new_r, s.union(&new_s).cloned().collect())
            });
    return visited.len().to_string();
}

pub fn part_1(file_contents: &str) -> String {
    return solver(file_contents, 2);
}

pub fn part_2(file_contents: &str) -> String {
    return solver(file_contents, 10);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_operations() {
        let v1 = Vector(1, 5);
        let v2 = Vector(7, 16);

        assert_eq!(v1 + v2, Vector(8, 21));
        assert_eq!(&v1 + v2, Vector(8, 21));
        assert_eq!(v1 + &v2, Vector(8, 21));
        assert_eq!(&v1 + &v2, Vector(8, 21));

        assert_eq!(v1 - v2, Vector(-6, -11));
        assert_eq!(&v1 - v2, Vector(-6, -11));
        assert_eq!(v1 - &v2, Vector(-6, -11));
        assert_eq!(&v1 - &v2, Vector(-6, -11));
    }

    #[test]
    fn parses_example_input_part_1() {
        let expected = vec![
            Motion {
                direction: Direction::Right,
                vector: Vector(1, 0),
                amount: 4,
            },
            Motion {
                direction: Direction::Up,
                vector: Vector(0, 1),
                amount: 4,
            },
            Motion {
                direction: Direction::Left,
                vector: Vector(-1, 0),
                amount: 3,
            },
            Motion {
                direction: Direction::Down,
                vector: Vector(0, -1),
                amount: 1,
            },
            Motion {
                direction: Direction::Right,
                vector: Vector(1, 0),
                amount: 4,
            },
            Motion {
                direction: Direction::Down,
                vector: Vector(0, -1),
                amount: 1,
            },
            Motion {
                direction: Direction::Left,
                vector: Vector(-1, 0),
                amount: 5,
            },
            Motion {
                direction: Direction::Right,
                vector: Vector(1, 0),
                amount: 2,
            },
        ];
        let input = include_str!("../tests/example_files/day9_part1.txt");
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn rope_moves_correctly_example_part_1() {
        let r = Rope::new(2);

        let m = Motion::new("R 4");
        let (r, s) = r.r#move(&m);
        assert_eq!(
            r,
            Rope {
                knots: vec![Vector(4, 0), Vector(3, 0)]
            }
        );
        assert_eq!(
            s,
            HashSet::from([Vector(0, 0), Vector(1, 0), Vector(2, 0), Vector(3, 0)])
        );

        let m = Motion::new("U 4");
        let (r, s) = r.r#move(&m);
        assert_eq!(
            r,
            Rope {
                knots: vec![Vector(4, 4), Vector(4, 3)]
            }
        );
        assert_eq!(
            s,
            HashSet::from([Vector(3, 0), Vector(4, 1), Vector(4, 2), Vector(4, 3)])
        );

        let m = Motion::new("L 3");
        let (r, s) = r.r#move(&m);
        assert_eq!(
            r,
            Rope {
                knots: vec![Vector(1, 4), Vector(2, 4)]
            }
        );
        assert_eq!(s, HashSet::from([Vector(4, 3), Vector(3, 4), Vector(2, 4)]));

        let m = Motion::new("D 1");
        let (r, s) = r.r#move(&m);
        assert_eq!(
            r,
            Rope {
                knots: vec![Vector(1, 3), Vector(2, 4)]
            }
        );
        assert_eq!(s, HashSet::from([Vector(2, 4)]));

        let m = Motion::new("R 4");
        let (r, s) = r.r#move(&m);
        assert_eq!(
            r,
            Rope {
                knots: vec![Vector(5, 3), Vector(4, 3)]
            }
        );
        assert_eq!(s, HashSet::from([Vector(2, 4), Vector(3, 3), Vector(4, 3)]));

        let m = Motion::new("D 1");
        let (r, s) = r.r#move(&m);
        assert_eq!(
            r,
            Rope {
                knots: vec![Vector(5, 2), Vector(4, 3)]
            }
        );
        assert_eq!(s, HashSet::from([Vector(4, 3)]));

        let m = Motion::new("L 5");
        let (r, s) = r.r#move(&m);
        assert_eq!(
            r,
            Rope {
                knots: vec![Vector(0, 2), Vector(1, 2)]
            }
        );
        assert_eq!(
            s,
            HashSet::from([Vector(4, 3), Vector(3, 2), Vector(2, 2), Vector(1, 2)])
        );

        let m = Motion::new("R 2");
        let (r, s) = r.r#move(&m);
        assert_eq!(
            r,
            Rope {
                knots: vec![Vector(2, 2), Vector(1, 2)]
            }
        );
        assert_eq!(s, HashSet::from([Vector(1, 2)]));
    }

    #[test]
    fn rope_moves_correctly_example_part_2() {
        let r = Rope::new(10);

        let m = Motion::new("R 5");
        let (r, s) = r.r#move(&m);
        assert_eq!(
            r,
            Rope {
                knots: vec![
                    Vector(5, 0),
                    Vector(4, 0),
                    Vector(3, 0),
                    Vector(2, 0),
                    Vector(1, 0),
                    Vector(0, 0),
                    Vector(0, 0),
                    Vector(0, 0),
                    Vector(0, 0),
                    Vector(0, 0),
                ]
            }
        );
        assert_eq!(s, HashSet::from([Vector(0, 0)]));

        let m = Motion::new("U 8");
        let (r, s) = r.r#move(&m);
        assert_eq!(
            r,
            Rope {
                knots: vec![
                    Vector(5, 8),
                    Vector(5, 7),
                    Vector(5, 6),
                    Vector(5, 5),
                    Vector(5, 4),
                    Vector(4, 4),
                    Vector(3, 3),
                    Vector(2, 2),
                    Vector(1, 1),
                    Vector(0, 0),
                ]
            }
        );
        assert_eq!(s, HashSet::from([Vector(0, 0)]));

        let m = Motion::new("L 8");
        let (r, s) = r.r#move(&m);
        assert_eq!(
            r,
            Rope {
                knots: vec![
                    Vector(-3, 8),
                    Vector(-2, 8),
                    Vector(-1, 8),
                    Vector(0, 8),
                    Vector(1, 8),
                    Vector(1, 7),
                    Vector(1, 6),
                    Vector(1, 5),
                    Vector(1, 4),
                    Vector(1, 3),
                ]
            }
        );
        assert_eq!(
            s,
            HashSet::from([Vector(0, 0), Vector(1, 1), Vector(2, 2), Vector(1, 3),])
        );

        let m = Motion::new("D 3");
        let (r, s) = r.r#move(&m);
        assert_eq!(
            r,
            Rope {
                knots: vec![
                    Vector(-3, 5),
                    Vector(-3, 6),
                    Vector(-2, 7),
                    Vector(-1, 7),
                    Vector(0, 7),
                    Vector(1, 7),
                    Vector(1, 6),
                    Vector(1, 5),
                    Vector(1, 4),
                    Vector(1, 3),
                ]
            }
        );
        assert_eq!(s, HashSet::from([Vector(1, 3),]));

        let m = Motion::new("R 17");
        let (r, s) = r.r#move(&m);
        assert_eq!(
            r,
            Rope {
                knots: vec![
                    Vector(14, 5),
                    Vector(13, 5),
                    Vector(12, 5),
                    Vector(11, 5),
                    Vector(10, 5),
                    Vector(9, 5),
                    Vector(8, 5),
                    Vector(7, 5),
                    Vector(6, 5),
                    Vector(5, 5),
                ]
            }
        );
        assert_eq!(
            s,
            HashSet::from([
                Vector(1, 3),
                Vector(2, 4),
                Vector(3, 5),
                Vector(4, 5),
                Vector(5, 5),
            ])
        );

        let m = Motion::new("D 10");
        let (r, s) = r.r#move(&m);
        assert_eq!(
            r,
            Rope {
                knots: vec![
                    Vector(14, -5),
                    Vector(14, -4),
                    Vector(14, -3),
                    Vector(14, -2),
                    Vector(14, -1),
                    Vector(14, 0),
                    Vector(13, 0),
                    Vector(12, 0),
                    Vector(11, 0),
                    Vector(10, 0),
                ]
            }
        );
        assert_eq!(
            s,
            HashSet::from([
                Vector(5, 5),
                Vector(6, 4),
                Vector(7, 3),
                Vector(8, 2),
                Vector(9, 1),
                Vector(10, 0),
            ])
        );

        let m = Motion::new("L 25");
        let (r, s) = r.r#move(&m);
        assert_eq!(
            r,
            Rope {
                knots: vec![
                    Vector(-11, -5),
                    Vector(-10, -5),
                    Vector(-9, -5),
                    Vector(-8, -5),
                    Vector(-7, -5),
                    Vector(-6, -5),
                    Vector(-5, -5),
                    Vector(-4, -5),
                    Vector(-3, -5),
                    Vector(-2, -5),
                ]
            }
        );
        assert_eq!(
            s,
            HashSet::from([
                Vector(10, 0),
                Vector(9, -1),
                Vector(8, -2),
                Vector(7, -3),
                Vector(6, -4),
                Vector(5, -5),
                Vector(4, -5),
                Vector(3, -5),
                Vector(2, -5),
                Vector(1, -5),
                Vector(0, -5),
                Vector(-1, -5),
                Vector(-2, -5),
            ])
        );

        let m = Motion::new("U 20");
        let (r, s) = r.r#move(&m);
        assert_eq!(
            r,
            Rope {
                knots: vec![
                    Vector(-11, 15),
                    Vector(-11, 14),
                    Vector(-11, 13),
                    Vector(-11, 12),
                    Vector(-11, 11),
                    Vector(-11, 10),
                    Vector(-11, 9),
                    Vector(-11, 8),
                    Vector(-11, 7),
                    Vector(-11, 6),
                ]
            }
        );
        assert_eq!(
            s,
            HashSet::from([
                Vector(-2, -5),
                Vector(-3, -4),
                Vector(-4, -3),
                Vector(-5, -2),
                Vector(-6, -1),
                Vector(-7, 0),
                Vector(-8, 1),
                Vector(-9, 2),
                Vector(-10, 3),
                Vector(-11, 4),
                Vector(-11, 5),
                Vector(-11, 6),
            ])
        );
    }
}
