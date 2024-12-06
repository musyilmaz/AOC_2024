use std::collections::{HashMap, HashSet};

#[test]
fn test_part1() {
    let test_string = String::from(
        "....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...",
    );
    let result = solve_part1(&test_string);

    assert_eq!(41, result)
}

#[test]
fn test_part2() {
    let test_string = String::from(
        "....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...",
    );
    let result = solve_part2(&test_string);

    assert_eq!(6, result)
}

type Coordinate = (i32, i32);
type GridType = HashMap<Coordinate, char>;

pub fn solve_part1(data: &String) -> usize {
    let (grid, initial_position) = read_data(&data);
    let initial_direction = 't';
    let visited = move_traveller(&grid, initial_position, initial_direction);

    visited.len()
}

pub fn solve_part2(data: &String) -> i32 {
    let (grid, initial_position) = read_data(&data);
    let initial_direction = 't';
    let visited = move_traveller(&grid, initial_position, initial_direction);
    let mut loop_count = 0;

    for (_, coor) in visited.iter().enumerate() {
        if *coor == initial_position {
            continue;
        }

        let mut modified_grid = grid.clone();
        modified_grid.entry(*coor).insert_entry('#');

        if move_traveller_to_detect_loop(&modified_grid, initial_position, initial_direction) {
            loop_count += 1;
        }
    }

    loop_count
}

fn move_traveller_to_detect_loop(grid: &GridType, mut pos: Coordinate, mut dir: char) -> bool {
    let mut is_looping = false;
    let mut extended_visited: HashSet<(Coordinate, char)> = HashSet::new();

    while !is_looping {
        if !grid.contains_key(&pos) {
            break;
        }
        match extended_visited.get(&(pos, dir)) {
            None => extended_visited.insert((pos, dir)),
            Some(_) => {
                is_looping = true;
                break;
            }
        };

        let (x, y) = pos;

        let next = match dir {
            't' => (x, y - 1),
            'r' => (x + 1, y),
            'b' => (x, y + 1),
            'l' => (x - 1, y),
            _ => unreachable!("all other directions are handled"),
        };

        let target = grid.get(&next);
        match target {
            Some('#') => {
                dir = match dir {
                    't' => 'r',
                    'r' => 'b',
                    'b' => 'l',
                    'l' => 't',
                    _ => unreachable!("all other directions are handled"),
                }
            }
            Some('.') => pos = next,
            _ => break,
        }
    }

    is_looping
}

fn move_traveller(grid: &GridType, mut pos: Coordinate, mut dir: char) -> HashSet<Coordinate> {
    let mut visited = HashSet::new();

    while grid.contains_key(&pos) {
        visited.insert(pos);
        let (x, y) = pos;

        let next = match dir {
            't' => (x, y - 1),
            'r' => (x + 1, y),
            'b' => (x, y + 1),
            'l' => (x - 1, y),
            _ => unreachable!("all other directions are handled"),
        };

        let target = grid.get(&next);
        match target {
            Some('#') => {
                dir = match dir {
                    't' => 'r',
                    'r' => 'b',
                    'b' => 'l',
                    'l' => 't',
                    _ => unreachable!("all other directions are handled"),
                }
            }
            _ => pos = next,
        }
    }

    visited
}

fn read_data(data: &String) -> (GridType, Coordinate) {
    let mut grid: GridType = HashMap::new();
    let mut initial_position: Option<Coordinate> = None;

    for (y, line) in data.lines().enumerate() {
        for (x, letter) in line
            .split_whitespace()
            .collect::<String>()
            .chars()
            .enumerate()
        {
            if letter == '^' {
                initial_position = Some((x as i32, y as i32));
                grid.insert((x as i32, y as i32), '.');
            } else {
                grid.insert((x as i32, y as i32), letter);
            }
        }
    }

    (grid, initial_position.unwrap())
}
