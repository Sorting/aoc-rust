use core::fmt::Display;
use crate::{utils::{get_single, get_single_test}, Solution};

struct Elf {
    items: Vec<usize>,
}
pub struct Day01 {
    elves: Vec<Elf>,
}

fn parser(s: String) -> Vec<Elf> { 
    let lines: Vec<&str> = s.split("\n").collect();
    let mut elfs: Vec<Elf> = Vec::new();
    let mut elf = Elf { items: Vec::new() };

    for line in lines {
        let calories = line.parse::<usize>();
        match calories {
            Ok(calories) => elf.items.push(calories),
            Err(_) => {
                elfs.push(elf);
                elf = Elf { items: Vec::new() };    
                continue;
            } 
        }
    }

    elfs
}

impl Day01 {
    pub fn new() -> Self {
        Day01 { elves: get_single(2022, 1, parser)}
    }

    pub fn new_test() -> Self {
        Day01 { elves: get_single_test(2022, 1, parser)}
    }

    fn part_1(&self) -> usize {
        self.elves.iter().map(|elf| {
            elf.items.iter().sum::<usize>()
        }).max().unwrap()
    }

    fn part_2(&self) -> usize {
        let mut ne: Vec<usize> = self.elves.iter().map(|elf| {
            elf.items.iter().sum::<usize>()
        }).collect();
        ne.sort_by(|a, b| b.cmp(a));

        ne.into_iter().take(3).sum()
    }
}

impl Solution for Day01 {
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
        assert_eq!(Day01::new_test().part_1(), 24000);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Day01::new_test().part_2(), 45000);
    }
}
