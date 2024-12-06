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

    assert_eq!(31, result)
}

type Coordinate = (usize, usize);
type GridType = HashMap<Coordinate, char>;

pub fn solve_part1(data: &String) -> usize {
    let (grid, initial_position) = read_data(&data);
    let initial_direction = 't';
    let visited = move_traveller(&grid, initial_position, initial_direction);

    visited.len()
}

pub fn solve_part2(data: &String) -> i32 {
    return 0;
}

fn move_traveller(grid: &GridType, mut pos: Coordinate, mut dir: char) -> HashSet<Coordinate> {
    let mut visited = HashSet::new();

    while grid.contains_key(&pos) {
        println!("{} {}: {:?} {:?}", "❗", "pos", dir, pos);
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
                initial_position = Some((x, y));
                grid.insert((x, y), '.');
            } else {
                grid.insert((x, y), letter);
            }
        }
    }

    (grid, initial_position.unwrap())
}
