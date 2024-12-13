#[test]
fn test_part1() {
    let test_string = String::from(
        "RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE",
    );
    let result = solve_part1(&test_string);

    assert_eq!(0, result)
}

#[test]
fn test_part2() {
    let test_string = String::from(
        "RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE",
    );
    let result = solve_part2(&test_string);

    assert_eq!(0, result)
}

type GridCell = (char, bool);
type Grid = Vec<Vec<GridCell>>;
type Coordinate = (isize, isize);

#[derive(Debug, Default)]
struct Map {
    grid: Grid,
    directions: Vec<Coordinate>,
    height: isize,
    width: isize,
}

impl Map {
    fn contains_coordinate(&self, coordinate: Coordinate) -> bool {
        let (x, y) = coordinate;

        if x < 0 || y < 0 {
            return false;
        }

        self.height > y && self.width > x
    }
}

pub fn solve_part1(data: &String) -> i32 {
    let map = read_data(data);
    println!("{} {}: {:?}", "❗", "map", map);
    0
}

pub fn solve_part2(data: &String) -> i32 {
    0
}

fn read_data(data: &String) -> Map {
    let mut map: Map = Map::default();
    let mut grid: Grid = vec![];

    for line in data.lines() {
        let mut row: Vec<GridCell> = vec![];
        for letter in line.trim().chars() {
            row.push((letter, false))
        }
        grid.push(row);
    }

    map.height = grid.len() as isize;
    map.width = grid[0].len() as isize;
    map.directions = Vec::from([(0, 1), (1, 0), (-1, 0), (0, -1)]);
    map.grid = grid;

    map
}
