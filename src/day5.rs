use crate::utils::divide_iterator_on_predicate;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("invalid regex");
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Moves {
    source: u32,
    dest: u32,
    count: u32,
}

fn build_stack(stack_strings: &Vec<&str>) -> Vec<Vec<char>> {
    let num_stacks_line = stack_strings.last().unwrap();
    let n_stacks = num_stacks_line
        .trim()
        .split(' ')
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut stack_vecs: Vec<Vec<char>> = (0..n_stacks).map(|_n| Vec::new()).collect();
    for line in stack_strings.iter().rev().skip(1) {
        for (c, v) in line.chars().skip(1).step_by(4).zip(stack_vecs.iter_mut()) {
            if c != ' ' {
                v.push(c);
            }
        }
    }
    return stack_vecs;
}

fn parse_moves<'a>(instructions: &'a Vec<&str>) -> impl Iterator<Item = Moves> + 'a {
    return instructions
        .iter()
        .map(|i| RE.captures(i).unwrap())
        .map(|c| Moves {
            // subtract one to make stsacks zero-indexed
            source: c[2].parse::<u32>().unwrap() - 1,
            dest: c[3].parse::<u32>().unwrap() - 1,
            count: c[1].parse::<u32>().unwrap(),
        });
}

fn perform_moves_part_1(stacks: &mut Vec<Vec<char>>, moves: &Vec<Moves>) -> () 
{
    for m in moves.into_iter() {
        for _i in 1..=m.count {
            // can't borrow both source and dest at the same time
            let tmp = {
                let source = stacks.get_mut(m.source as usize).unwrap();
                source.pop().unwrap()
            };
            let dest = stacks.get_mut(m.dest as usize).unwrap();
            dest.push(tmp);
        }
    }
}

fn get_stacks_and_moves(file_contents: &str) -> (Vec<Vec<char>>, Vec<Moves>) 
{
    let v = divide_iterator_on_predicate(file_contents.lines(), |&s| s == "");
    let (stack_strings, instruction_strings) = (&v[0], &v[1]);
    return (build_stack(&stack_strings), parse_moves(&instruction_strings).collect());
}

pub fn part_1(file_contents: &str) -> String {
    let (mut stacks, moves) = get_stacks_and_moves(file_contents);
    perform_moves_part_1(&mut stacks, &moves);
    return stacks.iter().map(|v| v.last().unwrap()).collect();
}

fn perform_moves_part_2(stacks: &mut Vec<Vec<char>>, moves: &Vec<Moves>) -> () 
{
    for m in moves.into_iter() {
        // can't borrow both source and dest at the same time
        let tmp = {
            let source = stacks.get_mut(m.source as usize).unwrap();
            let new_len = source.len() - m.count as usize;
            let copies = source[new_len..].to_owned();
            source.resize(new_len, '_');
            copies
        };
        let dest = stacks.get_mut(m.dest as usize).unwrap();
        dest.extend(tmp);
    }
}

pub fn part_2(file_contents: &str) -> String {
    let (mut stacks, moves) = get_stacks_and_moves(file_contents);
    perform_moves_part_2(&mut stacks, &moves);
    return stacks.iter().map(|v| v.last().unwrap()).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_build() {
        let stack_strings = vec!["    [D]    ", "[N] [C]    ", "[Z] [M] [P]", " 1   2   3 "];
        let expected = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(build_stack(&stack_strings), expected);
    }

    #[test]
    fn test_parse_moves() {
        let move_strings = vec![
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        let expected = vec![
            Moves{source: 1, dest: 0, count: 1},
            Moves{source: 0, dest: 2, count: 3},
            Moves{source: 1, dest: 0, count: 2},
            Moves{source: 0, dest: 1, count: 1},
        ];
        assert_eq!(parse_moves(&move_strings).collect::<Vec<Moves>>(), expected);
    }

    #[test]
    fn test_perform_moves_part_1() {
        let mut stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let moves = vec![
            Moves{source: 1, dest: 0, count: 1},
            Moves{source: 0, dest: 2, count: 3},
            Moves{source: 1, dest: 0, count: 2},
            Moves{source: 0, dest: 1, count: 1},
        ];
        perform_moves_part_1(&mut stacks, &moves);
        let expected = vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']];
        assert_eq!(stacks, expected);
    }

    #[test]
    fn test_perform_moves_part_2() {
        let mut stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let moves = vec![
            Moves{source: 1, dest: 0, count: 1},
            Moves{source: 0, dest: 2, count: 3},
            Moves{source: 1, dest: 0, count: 2},
            Moves{source: 0, dest: 1, count: 1},
        ];
        perform_moves_part_2(&mut stacks, &moves);
        let expected = vec![vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']];
        assert_eq!(stacks, expected);
    }
}
