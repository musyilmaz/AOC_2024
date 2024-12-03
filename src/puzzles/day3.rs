use regex::Regex;

#[test]
fn test_part1() {
    let test_string =
        String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");

    let result = solve_part1(&test_string);

    assert_eq!(161, result)
}

#[test]
fn test_part2() {
    let test_string =
        String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");

    let result = solve_part2(&test_string);

    assert_eq!(48, result)
}

pub fn solve_part1(data: &String) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let results: Vec<i32> = re
        .captures_iter(data)
        .map(|caps| {
            let (_, [num1, num2]) = caps.extract();
            return num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
        })
        .collect();

    return results.iter().sum();
}

enum RegexMatchResult {
    Mul(i32, i32),
    Do,
    Dont,
}

pub fn solve_part2(data: &String) -> i32 {
    let mut processing: bool = true;
    let mut sum: i32 = 0;
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();

    let results: Vec<RegexMatchResult> = re
        .captures_iter(data)
        .filter_map(|caps| {
            if caps.get(0)?.as_str() == "do()" {
                return Some(RegexMatchResult::Do);
            } else if caps.get(0)?.as_str() == "don't()" {
                return Some(RegexMatchResult::Dont);
            } else if caps.get(1).is_some() {
                let num1 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let num2 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                return Some(RegexMatchResult::Mul(num1, num2));
            } else {
                return None;
            }
        })
        .collect();

    for res in results {
        match res {
            RegexMatchResult::Mul(first, second) if processing => sum += first * second,
            RegexMatchResult::Do => processing = true,
            RegexMatchResult::Dont => processing = false,
            _ => {}
        }
    }

    return sum;
}

// This solver is not working apparently.
// Though cannot debug it as it is generating right code in test_string

#[allow(dead_code)]
pub fn solve_part2_old(data: &String) -> i32 {
    let modified_data = format!("do(){}don't()", data);
    let mut sum: i32 = 0;

    let split_re = Regex::new(r"do\(\)(.*?)don't\(\)").unwrap();
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let results: Vec<&str> = split_re
        .captures_iter(&modified_data)
        .map(|caps| {
            let (_, [cap]) = caps.extract();
            return cap;
        })
        .collect();

    for part in results {
        let results: Vec<i32> = re
            .captures_iter(part)
            .map(|caps| {
                let (_, [num1, num2]) = caps.extract();
                return num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
            })
            .collect();

        sum += results.iter().sum::<i32>();
    }

    return sum;
}
