use std::fs;
use std::env;

fn get_contents(year: i16, day: i16, is_test: bool) -> String {
   let path = env::current_dir().unwrap();

   fs::read_to_string(format!("{}/input/{}/day0{}{}", path.display(), year, day, if is_test { "_test" } else { "" }))
        .expect("Something went wrong when reading the file")
}

pub fn get_single<T>(year: i16, day: i16, parser: fn(String) -> T) -> T {
   parser(get_contents(year, day, false))
}

pub fn get_single_test<T>(year: i16, day: i16, parser: fn(String) -> T) -> T {
   parser(get_contents(year, day, true))
}

pub fn get_many<T>(year: i16, day: i16, parser: fn(&str) -> T) -> Vec<T> {
    get_many_from_string(get_contents(year, day, false), parser)
}

pub fn get_many_test<T>(year: i16, day: i16, parser: fn(&str) -> T) -> Vec<T> {
    get_many_from_string(get_contents(year, day, true), parser)
}

pub fn get_many_from_string<T>(content: String, parser: fn(&str) -> T) -> Vec<T> {
    content
        .split('\n')
        .map(|c| parser(c))
        .collect()
}