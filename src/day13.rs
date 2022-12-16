use crate::utils;
use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Node {
    Leaf(usize),
    Branch(Vec<Node>),
}

fn in_order(left: &Node, right: &Node) -> Ordering {
    return match left {
        Node::Leaf(x) => match right {
            Node::Leaf(y) => {
                if x < y {
                    Ordering::Less
                } else if x > y {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            Node::Branch(_y) => in_order(&Node::Branch(vec![left.clone()]), right),
        },
        Node::Branch(x) => match right {
            Node::Leaf(_y) => in_order(left, &Node::Branch(vec![right.clone()])),
            Node::Branch(y) => {
                for (i, j) in x.iter().zip(y.iter()) {
                    match in_order(i, j) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        _ => (),
                    }
                }
                return x.len().cmp(&y.len());
            }
        },
    };
}

/// Build the tree from the input
///
/// I couldn't figure out how to avoid this, but you need to drop the first
/// '[' char to get the function to work. Quite annoying.
fn build_tree(input: &mut VecDeque<char>) -> Node {
    let mut branch: Node = Node::Branch(vec![]);
    let mut num_to_parse: Vec<char> = vec![];
    let parse_num =
        |b: &mut Node, n: &mut Vec<char>| match n.iter().collect::<String>().parse::<usize>() {
            Ok(x) => {
                if let Node::Branch(y) = b {
                    y.push(Node::Leaf(x));
                }
            }
            Err(_) => (),
        };
    while !input.is_empty() {
        let c = input.pop_front();
        match c.unwrap() {
            '0'..='9' => num_to_parse.push(c.unwrap()),
            ',' => {
                parse_num(&mut branch, &mut num_to_parse);
                num_to_parse.clear();
            }
            '[' => {
                if let Node::Branch(ref mut x) = branch {
                    x.push(build_tree(input));
                }
            }
            ']' => {
                parse_num(&mut branch, &mut num_to_parse);
                return branch;
            }
            _ => panic!("unrecognised input char!"),
        }
    }
    return branch;
}

fn break_into_pairs(file_contents: &str) -> Vec<Vec<&str>> {
    return utils::divide_iterator_on_predicate(file_contents.lines(), |&s| s == "");
}

pub fn part_1(file_contents: &str) -> String {
    return break_into_pairs(file_contents)
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let trans = |s: &str| {
                let mut v = VecDeque::from(s.chars().collect::<Vec<char>>());
                v.pop_front();
                v
            };
            let t1 = build_tree(&mut trans(p[0]));
            let t2 = build_tree(&mut trans(p[1]));
            // remember that we need to be one-indexed
            return (i + 1, in_order(&t1, &t2));
        })
        .filter(|(_i, b)| *b == Ordering::Less)
        .map(|(i, _b)| i)
        .sum::<usize>()
        .to_string();
}

pub fn part_2(file_contents: &str) -> String {
    let decoder_packet_1: Node = Node::Branch(vec![Node::Branch(vec![Node::Leaf(2)])]);
    let decoder_packet_2: Node = Node::Branch(vec![Node::Branch(vec![Node::Leaf(6)])]);
    let mut vec: Vec<Node> = file_contents
        .lines()
        .filter(|&s| s != "")
        .map(|s| VecDeque::from(s.chars().collect::<Vec<char>>()))
        .map(|mut v| build_tree(&mut v))
        .collect();
    vec.push(decoder_packet_1.clone());
    vec.push(decoder_packet_2.clone());
    vec.sort_by(in_order);
    return vec
        .iter()
        .enumerate()
        .filter(|(_i, p)| **p == decoder_packet_1 || **p == decoder_packet_2)
        .map(|(i, _p)| i + 1)
        .product::<usize>()
        .to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_tree_from_example_inputs() {
        let expected = Node::Branch(vec![
            Node::Leaf(1),
            Node::Leaf(1),
            Node::Leaf(3),
            Node::Leaf(1),
            Node::Leaf(1),
        ]);
        assert_eq!(
            build_tree(&mut "1,1,3,1,1]".chars().collect::<VecDeque<char>>()),
            expected
        );

        let expected = Node::Branch(vec![
            Node::Branch(vec![Node::Leaf(1)]),
            Node::Branch(vec![Node::Leaf(2), Node::Leaf(3), Node::Leaf(4)]),
        ]);

        assert_eq!(
            build_tree(&mut "[1],[2,3,4]]".chars().collect::<VecDeque<char>>()),
            expected
        );

        let expected = Node::Branch(vec![Node::Branch(vec![Node::Branch(vec![])])]);

        assert_eq!(
            build_tree(&mut "[[]]]".chars().collect::<VecDeque<char>>()),
            expected
        );
    }

    #[test]
    fn in_order_on_example_1() {
        let left = Node::Branch(vec![
            Node::Leaf(1),
            Node::Leaf(1),
            Node::Leaf(3),
            Node::Leaf(1),
            Node::Leaf(1),
        ]);
        let right = Node::Branch(vec![
            Node::Leaf(1),
            Node::Leaf(1),
            Node::Leaf(5),
            Node::Leaf(1),
            Node::Leaf(1),
        ]);
        assert_eq!(in_order(&left, &right), Ordering::Less);
    }

    #[test]
    fn in_order_on_example_2() {
        let left = Node::Branch(vec![
            Node::Branch(vec![Node::Leaf(1)]),
            Node::Branch(vec![Node::Leaf(2), Node::Leaf(3), Node::Leaf(4)]),
        ]);
        let right = Node::Branch(vec![Node::Branch(vec![Node::Leaf(1)]), Node::Leaf(4)]);
        assert_eq!(in_order(&left, &right), Ordering::Less);
    }

    #[test]
    fn in_order_on_example_3() {
        let left = Node::Branch(vec![Node::Leaf(9)]);
        let right = Node::Branch(vec![Node::Branch(vec![
            Node::Leaf(8),
            Node::Leaf(7),
            Node::Leaf(6),
        ])]);
        assert_eq!(in_order(&left, &right), Ordering::Greater);
    }

    #[test]
    fn in_order_on_example_4() {
        let left = Node::Branch(vec![
            Node::Branch(vec![Node::Leaf(4), Node::Leaf(4)]),
            Node::Leaf(4),
            Node::Leaf(4),
        ]);
        let right = Node::Branch(vec![
            Node::Branch(vec![Node::Leaf(4), Node::Leaf(4)]),
            Node::Leaf(4),
            Node::Leaf(4),
            Node::Leaf(4),
        ]);
        assert_eq!(in_order(&left, &right), Ordering::Less);
    }

    #[test]
    fn in_order_on_example_5() {
        let left = Node::Branch(vec![
            Node::Leaf(7),
            Node::Leaf(7),
            Node::Leaf(7),
            Node::Leaf(7),
        ]);
        let right = Node::Branch(vec![Node::Leaf(7), Node::Leaf(7), Node::Leaf(7)]);
        assert_eq!(in_order(&left, &right), Ordering::Greater);
    }

    #[test]
    fn in_order_on_example_6() {
        let left = Node::Branch(vec![]);
        let right = Node::Branch(vec![Node::Leaf(3)]);
        assert_eq!(in_order(&left, &right), Ordering::Less);
    }

    #[test]
    fn in_order_on_example_7() {
        let left = Node::Branch(vec![Node::Branch(vec![Node::Branch(vec![])])]);
        let right = Node::Branch(vec![Node::Branch(vec![])]);
        assert_eq!(in_order(&left, &right), Ordering::Greater);
    }

    #[test]
    fn in_order_on_example_8() {
        let left = Node::Branch(vec![
            Node::Leaf(1),
            Node::Branch(vec![
                Node::Leaf(2),
                Node::Branch(vec![
                    Node::Leaf(3),
                    Node::Branch(vec![
                        Node::Leaf(4),
                        Node::Branch(vec![Node::Leaf(5), Node::Leaf(6), Node::Leaf(7)]),
                    ]),
                ]),
            ]),
            Node::Leaf(8),
            Node::Leaf(9),
        ]);
        let right = Node::Branch(vec![
            Node::Leaf(1),
            Node::Branch(vec![
                Node::Leaf(2),
                Node::Branch(vec![
                    Node::Leaf(3),
                    Node::Branch(vec![
                        Node::Leaf(4),
                        Node::Branch(vec![Node::Leaf(5), Node::Leaf(6), Node::Leaf(0)]),
                    ]),
                ]),
            ]),
            Node::Leaf(8),
            Node::Leaf(9),
        ]);
        assert_eq!(in_order(&left, &right), Ordering::Greater);
    }
}
