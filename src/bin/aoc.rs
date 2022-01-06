use aoc::years;
use aoc::print_solution;
use aoc::Solution;

fn main() {
    let solutions: Vec<(i16, i16, Box<dyn Solution>)> = vec![
        (2021, 1, Box::new(years::y2021::day01::Day01::new())),
        (2021, 2, Box::new(years::y2021::day02::Day02::new())),
    ];

    solutions.into_iter()
        .for_each(|(year, day, solution)| print_solution(year, day, solution));
    
   // print_solution(2021, 1, &day_1);
}