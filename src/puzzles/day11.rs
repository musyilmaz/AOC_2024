use std::{collections::HashMap, usize};

#[test]
fn test_part1() {
    let test_string = String::from("125 17");
    let test_iteration = 6;
    let result = solve_part1(&test_string, test_iteration);

    assert_eq!(22, result)
}

#[test]
fn test_part2() {
    let test_string = String::from("125 17");
    let test_iteration = 6;
    let result = solve_part2(&test_string, test_iteration);

    assert_eq!(22, result)
}

pub fn solve_part1(data: &String, iteration: i32) -> usize {
    let init_stones: Vec<usize> = read_data(data);
    let total_stone = solver_1(init_stones, iteration);

    total_stone
}

pub fn solve_part2(data: &String, iteration: i32) -> usize {
    let init_stones: Vec<usize> = read_data(data);
    let mut stones: HashMap<usize, usize> = HashMap::new();

    for item in init_stones {
        *stones.entry(item).or_default() += 1
    }

    for _ in 0..iteration {
        stones = solver_2(stones)
    }

    stones.values().sum()
}

fn read_data(data: &String) -> Vec<usize> {
    data.split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn solver_1(init_stones: Vec<usize>, iteration: i32) -> usize {
    let mut stones: Vec<usize> = init_stones.clone();
    let mut curr_iteration: i32 = 0;

    while curr_iteration < iteration {
        let mut new_stones: Vec<usize> = vec![];

        for stone in &stones {
            if stone.to_string() == "0" {
                new_stones.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                let stone_str = stone.to_string();
                let (first, second) = stone_str.split_at(stone.to_string().len() / 2);
                let first_val = first.parse::<usize>().unwrap();
                let second_val = second.parse::<usize>().unwrap();
                new_stones.push(first_val);
                new_stones.push(second_val);
            } else {
                new_stones.push(*stone * 2024);
            }
        }
        stones = new_stones;
        curr_iteration += 1;
    }

    stones.len()
}

fn solver_2(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones = HashMap::with_capacity(stones.len());

    for (stone, count) in stones {
        let stone_str = format!("{}", stone);

        match stone {
            0 => *new_stones.entry(1).or_default() += count,
            _ => {
                if stone_str.len() % 2 == 0 {
                    let (first, second) = stone_str.split_at(stone_str.len() / 2);
                    let first_val = first.parse::<usize>().unwrap();
                    let second_val = second.parse::<usize>().unwrap();
                    *new_stones.entry(first_val).or_default() += count;
                    *new_stones.entry(second_val).or_default() += count;
                } else {
                    *new_stones.entry(2024 * stone).or_default() += count
                }
            }
        }
    }

    new_stones
}
