use std::env;

use aoc::years;
use aoc::print_solution;
use aoc::Solution;

fn main() {
    let solutions: Vec<(i16, i16, Box<dyn Solution>)> = vec![
        (2021, 1, Box::new(years::y2021::day01::Day01::new())),
        (2021, 2, Box::new(years::y2021::day02::Day02::new())),
        (2022, 1, Box::new(years::y2022::day01::Day01::new())),
        (2022, 2, Box::new(years::y2022::day02::Day02::new())),
        (2022, 3, Box::new(years::y2022::day03::Day03::new())),
        (2022, 4, Box::new(years::y2022::day04::Day04::new())),
        (2022, 5, Box::new(years::y2022::day05::Day05::new())),
    ];
    
    let (year, day) = match &env::args().collect::<Vec<String>>()[..] {
        [_] => (None, None),
        [_, y] => (Some(y.parse::<i16>().unwrap()), None),
        [_, y, d] => (Some(y.parse::<i16>().unwrap()), Some(d.parse::<i16>().unwrap())),
        _ => panic!("Unknown arguments")
    };

    match (year, day) {
        (Some(year), Some(day)) => 
            solutions
                .into_iter()
                .filter(|(y, d, _)| y == &year && d == &day)
                .for_each(|(y, d, s)| print_solution(y, d, s)),
        (Some(year), None) =>
            solutions
                .into_iter()
                .filter(|(y, _, _)| y == &year)
                .for_each(|(y, d, s)| print_solution(y, d, s)),
        _ =>
            solutions
                .into_iter()
                .for_each(|(y, d, s)| print_solution(y, d, s))

    }
}
