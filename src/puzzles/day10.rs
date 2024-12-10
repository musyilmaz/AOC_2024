use std::collections::HashMap;

#[test]
fn test_part1() {
    let test_string = String::from(
        "89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732",
    );
    let result = solve_part1(&test_string);

    assert_eq!(36, result)
}

#[test]
fn test_part2() {
    let test_string = String::from(
        "89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732",
    );
    let result = solve_part2(&test_string);

    assert_eq!(81, result)
}

struct Map {
    grid: Grid,
    max_row: usize,
    max_col: usize,
}

type Grid = Vec<Vec<usize>>;
type Coordinate = (usize, usize);

pub fn solve_part1(data: &String) -> usize {
    let map = read_data(data);
    let head_coordinates = generate_head_coordinates(&map);
    get_trail_score(head_coordinates, map)
}

pub fn solve_part2(data: &String) -> usize {
    let map = read_data(data);
    let head_coordinates = generate_head_coordinates(&map);
    get_trail_count(head_coordinates, map)
}

fn read_data(data: &str) -> Map {
    let mut grid: Grid = vec![];

    for line in data.lines() {
        grid.push(
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>(),
        )
    }

    let max_row = grid.len();
    let max_col = grid[0].len();

    Map {
        grid,
        max_row,
        max_col,
    }
}

fn generate_head_coordinates(map: &Map) -> Vec<Coordinate> {
    let mut head_coordinates = vec![];

    for ((_, y), row) in (0..map.max_row).enumerate().zip(&map.grid) {
        for ((_, x), height) in (0..map.max_col).enumerate().zip(row) {
            if *height == 0 {
                head_coordinates.push((x, y))
            }
        }
    }

    head_coordinates
}

fn get_trail_score(head_coordinates: Vec<Coordinate>, map: Map) -> usize {
    let mut result: usize = 0;

    for (head_x, head_y) in head_coordinates {
        let mut stack: Vec<Coordinate> = vec![];
        let mut trails: HashMap<Coordinate, bool> = HashMap::new();

        stack.push((head_x, head_y));

        while stack.len() > 0 {
            let (curr_x, curr_y) = stack.pop().unwrap();

            let curr_height = map.grid[curr_y][curr_x];
            let target_height = curr_height + 1;

            if curr_height == 9 {
                trails.insert((curr_x, curr_y), true);
                continue;
            }

            stack = check_neighbour_coordinates(&mut stack, (curr_x, curr_y), target_height, &map);
        }
        result += trails.len();
        trails.clear();
    }

    result
}

fn check_neighbour_coordinates(
    stack: &mut Vec<Coordinate>,
    (curr_x, curr_y): Coordinate,
    target_height: usize,
    map: &Map,
) -> Vec<Coordinate> {
    if curr_x > 0 && map.grid[curr_y][curr_x - 1] == target_height {
        stack.push((curr_x - 1, curr_y))
    }
    if curr_y > 0 && map.grid[curr_y - 1][curr_x] == target_height {
        stack.push((curr_x, curr_y - 1))
    }
    if curr_x + 1 < map.max_col && map.grid[curr_y][curr_x + 1] == target_height {
        stack.push((curr_x + 1, curr_y))
    }
    if curr_y + 1 < map.max_row && map.grid[curr_y + 1][curr_x] == target_height {
        stack.push((curr_x, curr_y + 1))
    }

    stack.to_vec()
}

fn get_trail_count(head_coordinates: Vec<Coordinate>, map: Map) -> usize {
    let mut result = 0;

    for (head_x, head_y) in head_coordinates {
        let mut stack: Vec<Coordinate> = vec![];
        stack.push((head_x, head_y));

        while stack.len() > 0 {
            let (curr_x, curr_y) = stack.pop().unwrap();

            let curr_height = map.grid[curr_y][curr_x];
            let target_height = curr_height + 1;

            if curr_height == 9 {
                result += 1;
                continue;
            }

            stack = check_neighbour_coordinates(&mut stack, (curr_x, curr_y), target_height, &map);
        }
    }

    result
}
