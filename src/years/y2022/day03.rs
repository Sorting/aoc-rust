use std::collections::{HashSet, HashMap};
use core::fmt::Display;
use crate::{utils::{get_many, get_many_test, get_single_test, get_single}, Solution};

struct Rucksack(HashSet<char>, HashSet<char>);
struct Group(HashSet<char>, HashSet<char>, HashSet<char>);

pub struct Day03 {
    rucksacks: Vec<Rucksack>,
    groups: Vec<Group>,
    prio_list: HashMap<char, u8>,
}

fn compartments_parser(line: &str) -> Rucksack {
    let chars = line.chars().into_iter().map(|c| c).collect::<Vec<_>>();
    let (comp_a, comp_b) = chars.split_at(chars.len() / 2);
    Rucksack(comp_a.into_iter().fold(HashSet::new(), |mut acc, c| {
            acc.insert(*c);
            acc
        }), comp_b.into_iter().fold(HashSet::new(), |mut acc, c| {
            acc.insert(*c);
            acc
        }))
}

fn groups_parser(input: String) -> Vec<Group> {
    input
        .lines()
        .into_iter()
        .collect::<Vec<_>>()
        .chunks(3)
        .into_iter()
        .map(|x| {
            match x {
                [a, b, c] => {
                    let a = a.chars().into_iter().collect::<HashSet<_>>();
                    let b = b.chars().into_iter().collect::<HashSet<_>>();
                    let c = c.chars().into_iter().collect::<HashSet<_>>();
                    Group(a, b, c)
                },
                _ => panic!("Invalid input"),
            }
        }).collect()
}

fn get_prio_list() -> HashMap<char, u8> {
    let mut prio_list = HashMap::new();
    let alph = ('a' ..= 'z')
        .into_iter()
        .chain('A' ..= 'Z')
        .into_iter();

    for (i, c) in alph.enumerate() {
        prio_list.insert(c, (i + 1) as u8);
    }
    prio_list
}

impl Day03 {
    pub fn new() -> Self {
        Day03 {
            rucksacks: get_many(2022, 3, compartments_parser),
            groups: get_single(2022, 3, groups_parser),
            prio_list: get_prio_list(),
        }
    }

    pub fn new_test() -> Self {
        Day03 {
            rucksacks: get_many_test(2022, 3, compartments_parser),
            groups: get_single_test(2022, 3, groups_parser),
            prio_list: get_prio_list(),
        }
    }

    fn part_1(&self) -> usize {
        self.rucksacks.iter().flat_map(|Rucksack(compartment_a, compartment_b)| {
            compartment_a.iter().map(|x| {
                if compartment_b.contains(x) {
                    *self.prio_list.get(x).unwrap() as usize
                } else {
                    0
                }
            })
        }).sum()
    }

    fn part_2(&self) -> usize {
        self.groups.iter().flat_map(|Group(a, b, c)| {
            a.iter().map(|x| {
                if b.contains(x) && c.contains(x) {
                    *self.prio_list.get(x).unwrap() as usize
                } else {
                    0
                }
            })
        }).sum()
    }
}

impl Solution for Day03 {
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
        assert_eq!(Day03::new_test().part_1(), 157);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Day03::new_test().part_2(), 70);
    }
}
