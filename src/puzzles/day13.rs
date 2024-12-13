use std::{collections::HashMap, iter::Map};

use regex::Regex;

#[test]
fn test_part1() {
    let test_string = String::from(
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
    );
    let result = solve_part1(&test_string);

    assert_eq!(480, result)
}

#[test]
fn test_part2() {
    let test_string = String::from(
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
    );
    let result = solve_part2(&test_string);

    assert_eq!(0, result)
}

type Coordinate = (i32, i32);

#[derive(Debug)]
struct Machine {
    a: Coordinate,
    b: Coordinate,
    prize: Coordinate,
}

impl Machine {
    fn solve(&self) -> Option<i32> {
        // solve for A
        let a_top = self.prize.0 * self.b.1 - self.prize.1 * self.b.0;
        let a_bottom = self.a.0 * self.b.1 - self.b.0 * self.a.1;
        if a_bottom == 0 {
            return None;
        } else if a_top % a_bottom != 0 {
            return None;
        }

        let a_token = a_top / a_bottom;

        // solve for B
        let b_top = self.prize.1 * self.a.0 - self.prize.0 * self.a.1;
        let b_bottom = self.a.0 * self.b.1 - self.b.0 * self.a.1;

        if b_bottom == 0 {
            return None;
        } else if b_top % b_bottom != 0 {
            return None;
        }

        let b_token = b_top / b_bottom;

        Some(a_token * 3 + b_token)
    }
}

pub fn solve_part1(data: &String) -> i32 {
    let mut total = 0;
    let machines = parse_data(data);
    for machine in machines {
        let val = machine.solve();
        if val.is_some() {
            total += val.unwrap()
        }
    }

    total
}

pub fn solve_part2(data: &String) -> i32 {
    0
}

fn parse_data(data: &str) -> Vec<Machine> {
    let mut machines = vec![];
    for ruleset in data
        .split("\n\n")
        .into_iter()
        .map(|set| set.split("\n").collect::<Vec<_>>())
        .collect::<Vec<_>>()
    {
        let output = ruleset
            .iter()
            .enumerate()
            .filter(|(_, rule)| rule.len() != 0)
            .map(|(index, rule)| read_values_with_regex(index, rule))
            .collect::<Vec<(i32, i32)>>();

        if output.len() != 0 {
            machines.push(Machine {
                a: output[0],
                b: output[1],
                prize: output[2],
            });
        }
    }

    machines
}

fn read_values_with_regex(index: usize, rule: &str) -> (i32, i32) {
    let regex_rules = vec![
        Regex::new(r#"Button A: X\+(\d+), Y\+(\d+)"#).unwrap(),
        Regex::new(r#"Button B: X\+(\d+), Y\+(\d+)"#).unwrap(),
        Regex::new(r#"Prize: X=(\d+), Y=(\d+)"#).unwrap(),
    ];

    let caps = regex_rules[index].captures(rule).unwrap();

    let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();

    (x, y)
}
