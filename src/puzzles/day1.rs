#[test]
fn test_part1() {
    let test_string = String::from(
        "3   4
         4   3
         2   5
         1   3
         3   9
         3   3",
    );
    let result = solve_part1(&test_string);

    assert_eq!(11, result)
}

#[test]
fn test_part2() {
    let test_string = String::from(
        "3   4
         4   3
         2   5
         1   3
         3   9
         3   3",
    );
    let result = solve_part2(&test_string);

    assert_eq!(31, result)
}

pub fn solve_part1(data: &String) -> i32 {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in data.lines() {
        if let [left_val, right_val] = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .as_slice()
        {
            left.push(*left_val);
            right.push(*right_val);
        }
    }

    left.sort();
    right.sort();

    let result: i32 = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    return result;
}

pub fn solve_part2(data: &String) -> usize {
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];

    for line in data.lines() {
        if let [left_val, right_val] = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .as_slice()
        {
            left.push(*left_val);
            right.push(*right_val);
        }
    }

    let result: usize = left
        .iter()
        .map(|l| l * right.iter().filter(|r| &l == r).count())
        .sum();

    return result;
}
