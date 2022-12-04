use core::fmt::Display;
use std::collections::HashSet;
use crate::{utils::{get_many, get_many_test}, Solution};

pub struct Day04 {
    ranges: Vec<(HashSet<u8>, HashSet<u8>)>,
}

fn parser(line: &str) -> (HashSet<u8>, HashSet<u8>) {
    match line.split(',').collect::<Vec<_>>()[..] {
        [a, b] => (
            match a.split('-').collect::<Vec<_>>()[..] {
                [x, y] => {
                    (x.parse::<u8>().unwrap() ..= y.parse::<u8>().unwrap())
                        .collect::<HashSet<_>>()
                },
                _ => panic!("Invalid input"),
            },
            match b.split('-').collect::<Vec<_>>()[..] {
                [x, y] => {
                    (x.parse::<u8>().unwrap() ..= y.parse::<u8>().unwrap())
                        .collect::<HashSet<_>>()
                },
                _ => panic!("Invalid input"),
            }),
        _ => panic!("Invalid input"),
    }
}

impl Day04 {
    pub fn new() -> Self {
        Day04 { ranges: get_many(2022, 4, parser)}
    }

    pub fn new_test() -> Self {
        Day04 { ranges: get_many_test(2022, 4, parser)}
    }

    fn part_1(&self) -> usize {
        self.ranges.iter().filter(|(a, b)| {
            a.is_subset(b) || a.is_superset(b)
        }).count()
    }

    fn part_2(&self) -> usize {
        self.ranges.iter().filter(|(a, b)| {
            a.intersection(b).count() > 0
        }).count()
    }
}

impl Solution for Day04 {
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
        assert_eq!(Day04::new_test().part_1(), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Day04::new_test().part_2(), 4);
    }
}
