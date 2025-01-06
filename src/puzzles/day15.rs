use std::collections::{HashMap, HashSet};

use glam::IVec2;
use nom::InputIter;

#[test]
fn test_part1() {
    let test_string = String::from(
        "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
",
    );
    let result = solve_part1(&test_string);

    assert_eq!(2028, result)
}

#[test]
fn test_part2() {
    let test_string = String::from(
        "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
",
    );
    let result = solve_part2(&test_string);

    assert_eq!(0, result)
}

#[derive(Debug, PartialEq, Eq)]
enum GridObject {
    Wall,
    Box,
    Robot,
}

type Map = HashMap<IVec2, GridObject>;

pub fn solve_part1(data: &str) -> i32 {
    let (mut map, movements) = parse_data(data);

    for movement in movements {
        let robot_coor = map
            .iter()
            .find(|(_, object)| object == &&GridObject::Robot)
            .unwrap()
            .0
            .clone();

        let next_pos = robot_coor + movement;

        let Some(next) = map.get(&next_pos) else {
            println!("==========================");
            println!("Movement as {:?}", movement);
            println!("Moving Robot without anything else to {:?}", next_pos);
            println!("==========================");
            map.remove(&robot_coor);
            map.insert(next_pos, GridObject::Robot);

            continue;
        };

        match next {
            GridObject::Wall => continue,
            GridObject::Robot => {
                unreachable!("second robot")
            }
            GridObject::Box => {
                let mut movable_items = vec![next_pos];

                while Some(&GridObject::Box)
                    == map.get(&(movable_items.iter().last().unwrap() + movement))
                {
                    movable_items.push(movable_items.iter().last().unwrap() + movement);
                }

                let Some(_) = map.get(&(movable_items.iter().last().unwrap() + movement)) else {
                    let robot = map.remove(&robot_coor).unwrap();

                    let next_item_pos = movable_items.iter().next().unwrap();
                    let next_item = map.remove(next_item_pos).unwrap();

                    map.insert(*next_item_pos, robot);

                    match movable_items.iter().last() {
                        Some(last_item_pos) => map.insert(*last_item_pos + movement, next_item),
                        None => map.insert(*next_item_pos + movement, next_item),
                    };

                    continue;
                };
            }
        }
    }

    map.iter()
        .filter(|(_, object)| object == &&GridObject::Box)
        .map(|(position, _)| 100 * position.y + position.x)
        .sum()
}

pub fn solve_part2(data: &str) -> i32 {
    0
}

fn parse_data(data: &str) -> (Map, Vec<IVec2>) {
    let input_data = data.split("\n\n").collect::<Vec<&str>>();
    let grid_data = input_data[0];
    let movement_data = input_data[1];

    let map = parse_grid(grid_data);
    let movements = parse_movement(movement_data);

    (map, movements)
}

fn parse_grid(grid_data: &str) -> Map {
    let mut map: Map = HashMap::new();

    for (y, line) in grid_data.lines().enumerate() {
        for (x, letter) in line.trim().chars().enumerate() {
            let coordinate = IVec2::new(x as i32, y as i32);

            if letter == '#' {
                map.insert(coordinate, GridObject::Wall);
            } else if letter == 'O' {
                map.insert(coordinate, GridObject::Box);
            } else if letter == '@' {
                map.insert(coordinate, GridObject::Robot);
            }
        }
    }

    map
}

fn parse_movement(movement_data: &str) -> Vec<IVec2> {
    let mut movements: Vec<IVec2> = vec![];
    for char in movement_data.iter_elements() {
        match char {
            '^' => movements.push(IVec2::NEG_Y),
            '>' => movements.push(IVec2::X),
            'v' => movements.push(IVec2::Y),
            '<' => movements.push(IVec2::NEG_X),
            _ => break,
        }
    }
    movements
}
