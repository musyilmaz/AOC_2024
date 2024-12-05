#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

#[test]
fn test_part1() {
    let test_string = String::from(
        "47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47",
    );
    let result = solve_part1(&test_string);

    assert_eq!(143, result)
}

#[test]
fn test_part2() {
    let test_string = String::from("");
    let result = solve_part2(&test_string);

    assert_eq!(31, result)
}

#[derive(PartialEq)]
enum DataSection {
    Rules,
    Updates,
}

pub fn solve_part1(data: &String) -> i32 {
    let mut total = 0;
    let (r, u) = split_rules_and_updates(&data);

    let rules = read_rules(r);
    let updates = read_updates(u);

    for update in updates {
        let valid = check_update_validity(&update, &rules);

        if valid {
            total += update[update.len() / 2]
        }
    }

    total
}

pub fn solve_part2(data: &String) -> usize {
    return 0;
}

fn check_update_validity(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    let mut validity = true;

    for i in 0..update.len() - 1 {
        for j in i + 1..update.len() {
            let rule = rules.get(&update[i]);

            match rule {
                Some(values) => {
                    if !values.contains(&update[j]) {
                        validity = false
                    }
                }
                None => validity = false,
            }
        }
    }

    validity
}

fn read_rules(rules: Vec<&str>) -> HashMap<i32, Vec<i32>> {
    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for rule in rules {
        let splitted_rule = rule
            .split("|")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        rules_map
            .entry(splitted_rule[0])
            .or_insert_with(Vec::new)
            .push(splitted_rule[1]);
    }

    rules_map
}

fn read_updates(updates: Vec<&str>) -> Vec<Vec<i32>> {
    let mut updates_vec: Vec<Vec<i32>> = vec![];

    for update in updates {
        updates_vec.push(
            update
                .split(",")
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        );
    }

    updates_vec
}

fn split_rules_and_updates(data: &String) -> (Vec<&str>, Vec<&str>) {
    let mut data_section: DataSection = DataSection::Rules;
    let mut rules: Vec<&str> = vec![];
    let mut updates: Vec<&str> = vec![];

    for line in data.lines() {
        match line.len() {
            0 => data_section = DataSection::Updates,
            _ => {
                if data_section == DataSection::Rules {
                    rules.push(line.trim());
                } else {
                    updates.push(line.trim());
                }
            }
        }
    }

    (rules, updates)
}
