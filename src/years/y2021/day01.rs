use core::fmt::Display;
use crate::{utils::{get_many, get_many_test}, Solution};

pub struct Day01 {
    pub numbers: Vec<i32>
}

fn parser(s: &str) -> i32 { s.parse::<i32>().unwrap() }

impl Day01 {
    pub fn new() -> Self {
        Day01 { numbers: get_many(2021, 1, parser) }
    }

    pub fn new_test() -> Self {
        Day01 { numbers: get_many_test(2021, 1, parser) }
    }

    fn part_1(&self) -> i32 {
        let (res, _) = self.numbers.iter()
            .fold(
                (0, None),
                |(count, prev), n| 
                    match prev {
                        Some(p) => (count + if n > p { 1 } else { 0 }, Some(n)),
                        _ => (count, Some(n))
                    }
            );
            
        res
    }

    fn part_2(&self) -> i32 {
        let window_size = 3;

        let mut prev = 0; 
        let mut total = 0;
        let numbers_length = self.numbers.len() - 1;

        for n in 0..numbers_length {
            let mut sum = 0;
           for k in n..n + window_size {
               if k == numbers_length {
                   break;
               }
                sum += self.numbers[k]
           };

           if sum > prev {
               total += 1;
           }

           prev = sum;
        };

        total
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
    fn part_1() {
        let day01 = Day01::new_test();

        assert_eq!(day01.part_1(), 7);
    }

    #[test]
    fn part_2() {
        let day01 = Day01::new_test();

        assert_eq!(day01.part_2(), 5);
    }
}
