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
            let data = read_data_file(day);

            match day {
                "day1" => {
                    let part_1_result = day1::solve_part1(&data);
                    let part_2_result = day1::solve_part2(&data);

                    println!("{} {}: {:?}", "❗", "Part 1 - Result", part_1_result);
                    println!("{} {}: {:?}", "❗", "Part 2 - Result", part_2_result);
                }
                "day2" => {
                    let part_1_result = day2::solve_part1(&data);
                    let part_2_result = day2::solve_part2(&data);

                    println!("{} {}: {:?}", "❗", "Part 1 - Result", part_1_result);
                    println!("{} {}: {:?}", "❗", "Part 2 - Result", part_2_result);
                }
                "day3" => {
                    let part_1_result = day3::solve_part1(&data);
                    let part_2_result = day3::solve_part2(&data);

                    println!("{} {}: {:?}", "❗", "Part 1 - Result", part_1_result);
                    println!("{} {}: {:?}", "❗", "Part 2 - Result", part_2_result);
                }
                "day4" => {
                    let part_1_result = day4::solve_part1(&data);
                    let part_2_result = day4::solve_part2(&data);

                    println!("{} {}: {:?}", "❗", "Part 1 - Result", part_1_result);
                    println!("{} {}: {:?}", "❗", "Part 2 - Result", part_2_result);
                }
                "day5" => {
                    let part_1_result = day5::solve_part1(&data);
                    let part_2_result = day5::solve_part2(&data);

                    println!("{} {}: {:?}", "❗", "Part 1 - Result", part_1_result);
                    println!("{} {}: {:?}", "❗", "Part 2 - Result", part_2_result);
                }
                "day6" => {
                    let part_1_result = day6::solve_part1(&data);
                    let part_2_result = day6::solve_part2(&data);

                    println!("{} {}: {:?}", "❗", "Part 1 - Result", part_1_result);
                    println!("{} {}: {:?}", "❗", "Part 2 - Result", part_2_result);
                }
                "day7" => {
                    let part_1_result = day7::solve_part1(&data);
                    let part_2_result = day7::solve_part2(&data);

                    println!("{} {}: {:?}", "❗", "Part 1 - Result", part_1_result);
                    println!("{} {}: {:?}", "❗", "Part 2 - Result", part_2_result);
                }
                "day8" => {
                    let part_1_result = day8::solve_part1(&data);
                    let part_2_result = day8::solve_part2(&data);

                    println!("{} {}: {:?}", "❗", "Part 1 - Result", part_1_result);
                    println!("{} {}: {:?}", "❗", "Part 2 - Result", part_2_result);
                }
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
