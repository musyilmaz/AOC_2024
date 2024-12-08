#[test]
fn test_part1() {
    let test_string = String::from(
        "190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20",
    );
    let result = solve_part1(&test_string);

    assert_eq!(3749, result)
}

#[test]
fn test_part2() {
    let test_string = String::from(
        "190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20",
    );
    let result = solve_part2(&test_string);

    assert_eq!(11387, result)
}

pub fn solve_part1(data: &String) -> usize {
    let inputs = read_data(data);
    let mut total = 0;
    for (result, values) in inputs {
        let output = solver(result, &values);
        if output {
            total += result
        }
    }
    total
}

pub fn solve_part2(data: &String) -> usize {
    let inputs = read_data(data);
    let mut total = 0;

    for (result, values) in inputs {
        let output = solver_2(result, &values);
        if output {
            total += result
        }
    }

    total
}

fn solver(goal: usize, values: &[usize]) -> bool {
    if values.len() == 1 {
        return goal == values[0];
    }

    let (&last, rest) = values.split_last().unwrap();

    if goal % last == 0 && solver(goal / last, rest) {
        return true;
    }
    if goal > last && solver(goal - last, rest) {
        return true;
    }
    false
}

fn solver_2(goal: usize, values: &[usize]) -> bool {
    if values.len() == 1 {
        return goal == values[0];
    }

    let (&last, rest) = values.split_last().unwrap();

    if goal % last == 0 && solver_2(goal / last, rest) {
        return true;
    }
    if goal > last && solver_2(goal - last, rest) {
        return true;
    }

    let last_len = last.ilog10() + 1;
    let magnitude = 10u64.pow(last_len) as usize;
    let goal_len = goal.ilog10() + 1;
    let ending = goal % magnitude;
    if goal_len > last_len && last == ending && solver_2(goal / magnitude, rest) {
        return true;
    }
    false
}

fn read_data(data: &String) -> Vec<(usize, Vec<usize>)> {
    let mut inputs: Vec<(usize, Vec<usize>)> = vec![];

    for line in data.lines() {
        let (r, vals) = line.split_once(":").unwrap();
        let r: usize = r.trim().parse::<usize>().unwrap();
        let vals = vals
            .trim()
            .split(" ")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|v| v.trim().parse().unwrap())
            .collect::<Vec<usize>>();

        inputs.push((r, vals))
    }

    inputs
}
