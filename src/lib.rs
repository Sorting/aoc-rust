use std::fmt::Display;
use stopwatch::Stopwatch;
use termion::{color};

pub mod utils;
pub mod years;

pub trait Solution {
    fn part_1_display(&self) -> Box<dyn Display>;
    fn part_2_display(&self) -> Box<dyn Display>;
}

pub fn print_solution(year: i16, day: i16, solution: Box<dyn Solution>) {
    let print_execution_time = |seconds: f32| {
        print!("{} - It took {} seconds\n", color::Fg(color::Green), seconds);
    };

    println!("\n{}🎄🎄-{} Day {}-🎄🎄", color::Fg(color::Red), year, if day < 10 { format!("0{}", day) } else { format!("{}", day) });

    let mut sw = Stopwatch::start_new();
    let res1 = solution.part_1_display();

    sw.stop();

    print!("{}🎅 Part 1: {:>20}", color::Fg(color::White), res1);
    print_execution_time(sw.elapsed().as_secs_f32());
    
    sw.restart();
    let res2 = solution.part_2_display();
    sw.stop();

    print!("{}🎅 Part 2: {:>20}", color::Fg(color::White), res2);
    print_execution_time(sw.elapsed().as_secs_f32());
}