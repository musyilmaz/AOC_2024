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
    let result = solve_part2(&test_string);

    assert_eq!(0, result)
}

pub fn solve_part1(data: &String, iteration: i32) -> usize {
    let init_stones: Vec<usize> = read_data(data);
    let total_stone = solver_1(init_stones, iteration);

    total_stone
}

pub fn solve_part2(data: &String) -> i32 {
    0
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
