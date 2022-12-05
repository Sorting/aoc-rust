use crate::{
    utils::{get_many, get_many_test},
    Solution,
};
use core::fmt::Display;

pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

pub struct Round {
    my_choice: &'static Choice,
    their_choice: &'static Choice,
}

pub struct Day02 {
    part_1_rounds: Vec<Round>,
    part_2_rounds: Vec<Round>,
}

fn part_1_parser(s: &str) -> Option<Round> {
    let choices = s.split_whitespace().collect::<Vec<_>>();
    match choices[..] {
        [their, my] => Some(Round {
            my_choice: match my {
                "X" => &Choice::Rock,
                "Y" => &Choice::Paper,
                "Z" => &Choice::Scissors,
                _ => return None,
            },
            their_choice: match their {
                "A" => &Choice::Rock,
                "B" => &Choice::Paper,
                "C" => &Choice::Scissors,
                _ => return None,
            },
        }),
        _ => None,
    }
}

fn part_2_parser(s: &str) -> Option<Round> {
    let choices = s.split_whitespace().collect::<Vec<_>>();
    match choices[..] {
        [their, action] => {
            let their_choice = match their {
                "A" => &Choice::Rock,
                "B" => &Choice::Paper,
                "C" => &Choice::Scissors,
                _ => panic!("Invalid choice"),
            };
            Some(Round {
                their_choice,
                my_choice: match action {
                    "X" => match their_choice {
                        Choice::Rock => &Choice::Scissors,
                        Choice::Paper => &Choice::Rock,
                        Choice::Scissors => &Choice::Paper,
                    },
                    "Y" => their_choice,
                    "Z" => match their_choice {
                        Choice::Rock => &Choice::Paper,
                        Choice::Paper => &Choice::Scissors,
                        Choice::Scissors => &Choice::Rock,
                    },
                    _ => panic!("Invalid choice"),
                },
            })
        }
        _ => None,
    }
}

fn calc_score(round: &Round) -> usize {
    match (round.my_choice, round.their_choice) {
        (Choice::Rock, Choice::Scissors) => 7,
        (Choice::Paper, Choice::Rock) => 8,
        (Choice::Scissors, Choice::Paper) => 9,
        (Choice::Rock, Choice::Rock) => 4,
        (Choice::Paper, Choice::Paper) => 5,
        (Choice::Scissors, Choice::Scissors) => 6,
        (Choice::Rock, _) => 1,
        (Choice::Paper, _) => 2,
        (Choice::Scissors, _) => 3,
    }
}

impl Day02 {
    pub fn new() -> Self {
        Day02 {
            part_1_rounds: get_many(2022, 2, part_1_parser)
                .into_iter()
                .flatten()
                .collect(),
            part_2_rounds: get_many(2022, 2, part_2_parser)
                .into_iter()
                .flatten()
                .collect(),
        }
    }

    pub fn new_test() -> Self {
        Day02 {
            part_1_rounds: get_many_test(2022, 2, part_1_parser)
                .into_iter()
                .flatten()
                .collect(),
            part_2_rounds: get_many_test(2022, 2, part_2_parser)
                .into_iter()
                .flatten()
                .collect(),
        }
    }

    fn part_1(&self) -> usize {
        self.part_1_rounds.iter().map(calc_score).sum()
    }

    fn part_2(&self) -> usize {
        self.part_2_rounds.iter().map(calc_score).sum()
    }
}

impl Solution for Day02 {
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
        assert_eq!(Day02::new_test().part_1(), 15);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Day02::new_test().part_2(), 12);
    }
}
