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
    Empty,
    Wall,
    Box,
    Robot,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

type Map = Vec<Vec<(GridObject, IVec2)>>;

pub fn solve_part1(data: &str) -> i32 {
    let dirs = [
        IVec2::new(0, -1),
        IVec2::new(1, 0),
        IVec2::new(0, 1),
        IVec2::new(-1, 0),
    ];

    let (mut map, movements) = parse_data(data);

    for movement in movements {
        let robot = find_robot(&map);
        println!("{} {}: {:?}", "❗", "robot", robot);
    }
    0
}

pub fn solve_part2(data: &str) -> i32 {
    0
}

fn parse_data(data: &str) -> (Map, Vec<Direction>) {
    let input_data = data.split("\n\n").collect::<Vec<&str>>();
    let grid_data = input_data[0];
    let movement_data = input_data[1];

    let map = parse_grid(grid_data);

    let movements = parse_movement(movement_data);

    (map, movements)
}

fn parse_grid(grid_data: &str) -> Map {
    let mut grid: Map = vec![];
    for (y, line) in grid_data.lines().enumerate() {
        let mut row: Vec<(GridObject, IVec2)> = vec![];

        for (x, letter) in line.trim().chars().enumerate() {
            let coordinate = IVec2::new(x as i32, y as i32);

            match letter {
                '#' => row.push((GridObject::Wall, coordinate)),
                'O' => row.push((GridObject::Box, coordinate)),
                '@' => row.push((GridObject::Robot, coordinate)),
                _ => row.push((GridObject::Empty, coordinate)),
            }
        }
        grid.push(row);
    }
    grid
}

fn parse_movement(movement_data: &str) -> Vec<Direction> {
    let mut movements: Vec<Direction> = vec![];
    for char in movement_data.iter_elements() {
        match char {
            '^' => movements.push(Direction::Up),
            '>' => movements.push(Direction::Right),
            'v' => movements.push(Direction::Down),
            '<' => movements.push(Direction::Left),
            _ => break,
        }
    }
    movements
}

fn find_robot(map: &Map) -> IVec2 {
    let mut robot_pos = IVec2::new(0, 0);

    for row in map.iter() {
        for cell in row.iter() {
            if cell.0 != GridObject::Robot {
                continue;
            }

            robot_pos = cell.1;
        }
    }

    robot_pos
}
