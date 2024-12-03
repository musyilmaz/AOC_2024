#[test]
fn test_part1() {
    let result = solve_part1(String::from(
        "7 6 4 2 1
         1 2 7 8 9
         9 7 6 2 1
         1 3 2 4 5
         8 6 4 4 1
         1 3 6 7 9",
    ));

    assert_eq!(2, result)
}

#[test]
fn test_part2() {
    let result = solve_part2(String::from(
        "7 6 4 2 1
         1 2 7 8 9
         9 7 6 2 1
         1 3 2 4 5
         8 6 4 4 1
         1 3 6 7 9",
    ));

    assert_eq!(4, result)
}

pub fn solve_part1(data: String) -> i32 {
    let mut result: i32 = 0;

    for line in data.lines() {
        let row = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if check_safety(&row) {
            result += 1
        }
    }

    return result;
}

pub fn solve_part2(data: String) -> i32 {
    let mut result: i32 = 0;

    for line in data.lines() {
        let row = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if check_safety(&row) || check_safety_with_dampener(&row) {
            result += 1
        }
    }

    return result;
}

fn check_safety(row: &Vec<i32>) -> bool {
    let is_descending = row[0] > row[1];
    let mut is_safe: bool = true;

    for i in 0..(row.len() - 1) {
        let a = row[i];
        let b = row[i + 1];
        let diff = (a - b).abs();

        if is_descending && a < b || !is_descending && a > b || diff < 1 || diff > 3 {
            is_safe = false;
        }
    }

    return is_safe;
}

fn check_safety_with_dampener(row: &Vec<i32>) -> bool {
    (0..row.len()).any(|i| {
        let mut tmp_row = row.clone();
        tmp_row.remove(i);
        check_safety(&tmp_row)
    })
}
