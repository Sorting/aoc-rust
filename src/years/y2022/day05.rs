use core::fmt::Display;
use std::collections::{HashMap, VecDeque};

use crate::{
    utils::{get_single, get_single_test},
    Solution,
};

enum Instruction {
    Move(usize, usize, usize),
}
struct Data {
    stacks: Vec<VecDeque<char>>,
    instructions: Vec<Instruction>,
}
pub struct Day05 {
    data: Data,
}

fn parser(input: String) -> Data {
    match input.split("\n\n").into_iter().collect::<Vec<_>>()[..] {
        [layout_input, instr_input] => {
            let mut stacks = layout_input
                .lines()
                .fold(HashMap::new(), |mut map, x| {
                    x.chars().enumerate().for_each(|(i, c)| {
                        if char::is_alphabetic(c) {
                            map.entry(i).or_insert_with(VecDeque::new).push_back(c);
                        };
                    });
                    map
                })
                .into_iter()
                .collect::<Vec<(usize, VecDeque<char>)>>();

            stacks.sort_by_key(|(i, _)| *i);

            Data {
                stacks: stacks.into_iter().map(|(_, v)| v).collect(),
                instructions: instr_input
                    .lines()
                    .map(|line| {
                        let ns = line.split_whitespace().collect::<Vec<_>>();
                        match ns[..] {
                            [_, n, _, from, _, to] => Instruction::Move(
                                n.parse::<usize>().unwrap(),
                                from.parse::<usize>().unwrap(),
                                to.parse::<usize>().unwrap(),
                            ),
                            _ => panic!("Invalid instruction"),
                        }
                    })
                    .collect::<Vec<Instruction>>(),
            }
        }
        _ => panic!("Invalid input"),
    }
}

impl Day05 {
    pub fn new() -> Self {
        Day05 {
            data: get_single(2022, 5, parser),
        }
    }

    pub fn new_test() -> Self {
        Day05 {
            data: get_single_test(2022, 5, parser),
        }
    }

    fn part_1(&self) -> String {
        let mut stacks = self.data.stacks.clone();
        self.data
            .instructions
            .iter()
            .for_each(|Instruction::Move(n, from, to)| {
                let from_stack = &mut stacks[*from - 1].clone();
                let to_stack = &mut stacks[*to - 1].clone();

                for _ in 0..*n {
                    to_stack.push_front(from_stack.pop_front().unwrap());
                }

                stacks[*from - 1] = from_stack.clone();
                stacks[*to - 1] = to_stack.clone();
            });

        stacks.iter().map(|x| x[0]).collect::<String>()
    }

    fn part_2(&self) -> String {
        let mut stacks = self.data.stacks.clone();
        self.data
            .instructions
            .iter()
            .for_each(|Instruction::Move(n, from, to)| {
                let from_stack = &mut stacks[*from - 1].clone();
                let to_stack = &mut stacks[*to - 1].clone();

                let mut new_to_stack = from_stack.drain(..*n).collect::<VecDeque<char>>();
                new_to_stack.append(to_stack);

                stacks[*from - 1] = from_stack.clone();
                stacks[*to - 1] = new_to_stack;
            });

        stacks.iter().map(|x| x[0]).collect::<String>()
    }
}

impl Solution for Day05 {
    fn part_1_display(&self) -> Box<dyn Display> {
        Box::new(self.part_1())
    }

    fn part_2_display(&self) -> Box<dyn Display> {
        Box::new(self.part_2())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(Day05::new_test().part_1(), "CMZ");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Day05::new_test().part_2(), "MCD");
    }
}
