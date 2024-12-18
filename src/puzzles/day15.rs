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

#[derive(Debug)]
enum GridObject {
    Empty,
    Wall,
    Box,
    Robot,
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

type Map = Vec<Vec<GridObject>>;

pub fn solve_part1(data: &str) -> i32 {
    let (map, movements) = parse_data(data);
    println!("{} {}: {:?}", "❗", "movements", movements);
    println!("{} {}: {:?}", "❗", "map", map);
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
    let mut grid: Vec<Vec<GridObject>> = vec![];
    for line in grid_data.lines() {
        let mut row: Vec<GridObject> = vec![];

        for letter in line.trim().chars() {
            match letter {
                '#' => row.push(GridObject::Wall),
                'O' => row.push(GridObject::Box),
                '@' => row.push(GridObject::Robot),
                _ => row.push(GridObject::Empty),
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
