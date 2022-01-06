use core::fmt::Display;
use crate::{utils::get_many, utils::get_many_test, Solution};

pub struct Day02 {
    pub commands: Vec<Command>
}

pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
    None
}

fn parser(input: &str) -> Command {
    let s = input.split(" ").collect::<Vec<&str>>();

    if s.len() != 2 {
        return Command::None;
    }

    let steps = s[1].parse::<i32>().unwrap();

    match s[0] {
        "up" => Command::Up(steps),
        "down" => Command::Down(steps),
        _ => Command::Forward(steps)
    }
}

impl Day02 {
    pub fn new() -> Self {
        Day02 { commands: get_many(2021, 2, parser) }
    }    

    pub fn new_test() -> Self {
        Day02 { commands: get_many_test(2021, 2, parser) }
    }

    fn part_1(&self) -> i32 {
        let (x, y) = self.commands
            .iter()
            .fold((0, 0), |(x, y), cmd|
                match cmd {
                    Command::Forward(steps) => (x + steps, y),
                    Command::Down(steps) => (x, y + steps),
                    Command::Up(steps) => (x, y - steps),
                    _ => panic!("Unknown command")
                });
        x * y
    }

    fn part_2(&self) -> i32 {
        let (x, y, _) = self.commands
            .iter()
            .fold((0, 0, 0), |(x, y, a), cmd|
                match cmd {
                    Command::Forward(steps) => (x + steps, y + (a * steps), a),
                    Command::Down(steps) => (x, y, a + steps),
                    Command::Up(steps) => (x, y, a - steps),
                    _ => panic!("Unknown command")
                });
        x * y
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
    fn part_1() {
        let day = Day02::new_test();
        assert_eq!(day.part_1(), 150);
    }

    #[test]
    fn part_2() {
        let day = Day02::new_test();
        assert_eq!(day.part_2(), 900);
    }
}