use crate::utils;
use regex::Regex;
use std::collections::VecDeque;

lazy_static! {
    static ref RE_MONKEY: Regex = Regex::new(r"Monkey (\d+)").expect("invalid regex");
    static ref RE_ITEMS: Regex = Regex::new(r"Starting items: (.+)").expect("invalid regex");
    static ref RE_OPERATION: Regex =
        Regex::new(r"Operation: new = old (.) (.+)").expect("invalid regex");
    static ref RE_TEST: Regex = Regex::new(r"Test: divisible by (\d+)").expect("invalid regex");
    static ref RE_TRUE_FALSE: Regex =
        Regex::new(r"If (true|false): throw to monkey (\d+)").expect("invalid regex");
}

/// Convenience function to get a number out of a regex that we know is in
/// there
fn get_number_from_regex(re: &Regex, text: &str, match_pos: usize) -> usize {
    return re
        .captures(text)
        .unwrap()
        .get(match_pos)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
}

#[derive(PartialEq, Eq, Debug)]
struct Monkey<'a> {
    number: usize,
    items: VecDeque<usize>,
    operator: &'a str,
    rhs: Option<usize>,
    modulus: usize,
    true_target: usize,
    false_target: usize,
    inspections: usize,
}

impl<'a> Monkey<'a> {
    fn new(text: &Vec<&'a str>) -> Self {
        let (operator, rhs) = Self::get_operation(text[2]);
        return Monkey {
            number: get_number_from_regex(&RE_MONKEY, text[0], 1) as usize,
            items: Self::get_held_items(text[1]),
            operator,
            rhs,
            modulus: get_number_from_regex(&RE_TEST, text[3], 1),
            true_target: get_number_from_regex(&RE_TRUE_FALSE, text[4], 2) as usize,
            false_target: get_number_from_regex(&RE_TRUE_FALSE, text[5], 2) as usize,
            inspections: 0,
        };
    }

    fn get_held_items(text: &str) -> VecDeque<usize> {
        let num_str = RE_ITEMS.captures(text).unwrap().get(1).unwrap().as_str();
        return num_str
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
    }

    fn get_operation(text: &str) -> (&str, Option<usize>) {
        let caps = RE_OPERATION.captures(text).unwrap();
        let possible_number = caps.get(2).unwrap().as_str();
        let validated_number: Option<usize> = if possible_number == "old" {
            None
        } else {
            Some(possible_number.parse::<usize>().unwrap())
        };

        let operator = caps.get(1).unwrap().as_str();
        return (operator, validated_number);
    }

    fn perform_op(item: usize, operator: &str, rhs: Option<usize>) -> usize {
        return match rhs {
            Some(x) => match operator {
                "+" => item + x,
                "-" => item - x,
                "*" => item * x,
                "/" => item / x,
                _ => panic!("invalid operator"),
            },
            None => match operator {
                "+" => item + item,
                "-" => item - item,
                "*" => item * item,
                "/" => item / item,
                _ => panic!("invalid operator"),
            },
        };
    }

    fn inspect_items_and_throw(
        &mut self,
        worry_reduction_factor: usize,
        least_common_multiple: usize,
    ) -> Vec<(usize, usize)> {
        // first increment the number of inspections
        self.inspections += self.items.len() as usize;

        // now the actual inspection procedure. we have to clone these values
        // so we can mutably borrow to drain the items from the vector
        let op = self.operator.clone();
        let rhs = self.rhs.clone();
        let modulus = self.modulus.clone();
        let true_target = self.true_target.clone();
        let false_target = self.false_target.clone();

        return self
            .items
            .drain(..)
            .map(|i| {
                (Self::perform_op(i, op, rhs) / worry_reduction_factor) % least_common_multiple
            })
            .map(|i| {
                if (i as usize) % modulus == 0 {
                    (true_target, i)
                } else {
                    (false_target, i)
                }
            })
            .collect();
    }

    fn distribute_thrown_items(monkeys: &mut Vec<Monkey>, items: &Vec<(usize, usize)>) -> () {
        for (target, item) in items.iter() {
            let monkey = monkeys.get_mut(*target).unwrap();
            monkey.items.push_back(*item);
        }
    }
}

fn single_round(
    monkeys: &mut Vec<Monkey>,
    worry_factor: usize,
    least_common_multiple: usize,
) -> () {
    for i in 0..monkeys.len() {
        let items = {
            let monkey = monkeys.get_mut(i).unwrap();
            monkey.inspect_items_and_throw(worry_factor, least_common_multiple)
        };
        Monkey::distribute_thrown_items(monkeys, &items);
    }
}

fn parse_input_to_monkeys(file_contents: &str) -> Vec<Monkey> {
    return utils::divide_iterator_on_predicate(file_contents.lines(), |&s| s == "")
        .iter()
        .map(|v| Monkey::new(v))
        .collect();
}

fn solver(file_contents: &str, num_rounds: usize, worry_factor: usize) -> String {
    let mut monkeys = parse_input_to_monkeys(file_contents);
    let least_common_multiple = monkeys.iter().map(|m| m.modulus).product::<usize>();
    for _i in 0..num_rounds {
        single_round(&mut monkeys, worry_factor, least_common_multiple);
    }
    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections.reverse();
    return inspections.iter().take(2).product::<usize>().to_string();
}

pub fn part_1(file_contents: &str) -> String {
    return solver(file_contents, 20, 3);
}

pub fn part_2(file_contents: &str) -> String {
    return solver(file_contents, 10000, 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn monkey_construction() {
        let example_str = include_str!("../tests/example_files/day11.txt");
        let input_vec = utils::divide_iterator_on_predicate(example_str.lines(), |&s| s == "");
        let expected_monkeys = [
            Monkey {
                number: 0,
                items: VecDeque::from(vec![79, 98]),
                operator: "*",
                rhs: Some(19),
                modulus: 23,
                true_target: 2,
                false_target: 3,
                inspections: 0,
            },
            Monkey {
                number: 1,
                items: VecDeque::from(vec![54, 65, 75, 74]),
                operator: "+",
                rhs: Some(6),
                modulus: 19,
                true_target: 2,
                false_target: 0,
                inspections: 0,
            },
            Monkey {
                number: 2,
                items: VecDeque::from(vec![79, 60, 97]),
                operator: "*",
                rhs: None,
                modulus: 13,
                true_target: 1,
                false_target: 3,
                inspections: 0,
            },
            Monkey {
                number: 3,
                items: VecDeque::from(vec![74]),
                operator: "+",
                rhs: Some(3),
                modulus: 17,
                true_target: 0,
                false_target: 1,
                inspections: 0,
            },
        ];
        for (input, expected) in input_vec.iter().zip(expected_monkeys.iter()) {
            assert_eq!(Monkey::new(&input), *expected);
        }
    }
}
