use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[test]
fn test_part1() {
    let test_string = String::from(
        "............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............",
    );
    let result = solve_part1(&test_string);

    assert_eq!(14, result)
}

#[test]
fn test_part2() {
    let test_string = String::from(
        "............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............",
    );
    let result = solve_part2(&test_string);

    assert_eq!(0, result)
}

type Coordinate = (i32, i32);
type Antinode = HashSet<Coordinate>;
type GridLocations = HashMap<char, HashSet<Coordinate>>;

#[derive(Debug)]
struct Grid {
    locations: GridLocations,
    height: i32,
    width: i32,
}

impl Grid {
    fn contains_coordinate(&self, coordinate: Coordinate) -> bool {
        let (x, y) = coordinate;

        if x < 0 || y < 0 {
            return false;
        }

        self.height > y && self.width > x
    }
}

pub fn solve_part1(data: &String) -> usize {
    let mut antinodes: Antinode = HashSet::new();
    let grid = read_data(data);

    for (_char, locations) in grid.locations.iter() {
        solver(&mut antinodes, locations, &grid)
    }

    antinodes.len()
}

pub fn solve_part2(data: &String) -> i32 {
    0
}

fn read_data(input: &str) -> Grid {
    let mut locations: GridLocations = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line
            .split_whitespace()
            .collect::<String>()
            .chars()
            .enumerate()
        {
            if matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9') {
                locations
                    .entry(char)
                    .or_default()
                    .insert((x as i32, y as i32));
            }
        }
    }

    return Grid {
        locations,
        height: input.lines().count() as i32,
        width: input.lines().next().unwrap().chars().count() as i32,
    };
}

fn solver(antinodes: &mut Antinode, locations: &HashSet<Coordinate>, grid: &Grid) {
    for (&first, &second) in locations.iter().tuple_combinations() {
        let (first_antinode, second_antinode) = get_antinode_coordinates(first, second);

        if grid.contains_coordinate(first_antinode) {
            antinodes.insert(first_antinode);
        }

        if grid.contains_coordinate(second_antinode) {
            antinodes.insert(second_antinode);
        }
    }
}

fn get_antinode_coordinates(first: Coordinate, second: Coordinate) -> (Coordinate, Coordinate) {
    let x_delta = first.0 - second.0;
    let y_delta = first.1 - second.1;

    (
        (first.0 + x_delta, first.1 + y_delta),
        (second.0 - x_delta, second.1 - y_delta),
    )
}
