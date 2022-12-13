use itertools::*;
use regex::Regex;
use std::collections::HashMap;

const SCREEN_SIZE: usize = 40;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Operation {
    NOOP,
    ADDX,
}

struct Instruction {
    op: Operation,
    number: Option<isize>,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"\w+ (-?\d+)").expect("invalid regex");
    static ref OP_TIMES: HashMap<Operation, usize> =
        HashMap::from([(Operation::NOOP, 0), (Operation::ADDX, 1)]);
}

fn parse_cmd(line: &str) -> Instruction {
    let caps = RE.captures(line);
    return match caps {
        Some(x) => Instruction {
            op: Operation::ADDX,
            number: x.get(1).unwrap().as_str().parse::<isize>().ok(),
        },
        None => Instruction {
            op: Operation::NOOP,
            number: None,
        },
    };
}

fn register_through_time(file_contents: &str) -> Vec<isize> {
    // 3* in case every operation is an add, largest possible size
    let mut register: Vec<isize> = Vec::with_capacity(file_contents.lines().count() * 3);
    register.push(1);
    for line in file_contents.lines() {
        let top = register.last().unwrap().clone();
        let cmd = parse_cmd(line);
        for _i in 0..OP_TIMES[&cmd.op] {
            register.push(top);
        }
        match cmd.number {
            Some(x) => register.push(top + x),
            None => register.push(top),
        }
    }
    return register;
}

pub fn part_1(file_contents: &str) -> String {
    let register = register_through_time(file_contents);
    return register
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, acc)| ((i + 1) as isize) * acc)
        .sum::<isize>()
        .to_string();
}

fn render_screen(screen: &[bool]) -> String {
    return screen
        .iter()
        .map(|&b| if b { '#' } else { '.' })
        .chunks(SCREEN_SIZE)
        .into_iter()
        .map(|chars| chars.collect::<String>())
        .join("\n");
}

pub fn part_2(file_contents: &str) -> String {
    let register = register_through_time(file_contents);
    let mut screen: Vec<bool> = vec![false; register.len()];
    for (i, sprite_pos) in register.iter().enumerate() {
        let trial = ((*sprite_pos as isize) - ((i % SCREEN_SIZE) as isize)).abs();
        if trial <= 1 {
            screen[i] = true;
        }
    }
    // not sure why but we get a spurious extra line with a single entry if we
    // don't cut off the last value
    return "\n".to_owned() + &render_screen(&screen[0..screen.len() - 1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_register_for_small_example_part_1() {
        // remember to include the zeroth step
        let expected: Vec<isize> = vec![1, 1, 1, 4, 4, -1];
        let input = "noop\naddx 3\naddx -5";
        assert_eq!(register_through_time(input), expected);
    }
}
