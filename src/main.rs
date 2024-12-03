mod puzzles;

use crate::puzzles::*;
use std::fs;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    match args.len() {
        1 => {
            panic!("Not enough arguments")
        }
        _ => {
            let day = args[1].as_str();
            let part = args[2].as_str();
            let data = read_data_file(day);

            match day {
                "day1" => match part {
                    "part1" => {
                        let _ = day1::solve_part1(data);
                    }
                    "part2" => {
                        let _ = day1::solve_part2(data);
                    }
                    _ => {
                        panic!("Invalid argument")
                    }
                },
                _ => {
                    panic!("Invalid argument")
                }
            }
        }
    }
}

fn read_data_file(day: &str) -> String {
    return fs::read_to_string(format!("data/{}.txt", day)).unwrap();
}
