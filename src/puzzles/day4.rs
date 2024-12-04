use std::usize;

#[test]
fn test_part1() {
    let test_string = String::from(
        "MMMSXXMASM
         MSAMXMSMSA
         AMXSXMAAMM
         MSAMASMSMX
         XMASAMXAMM
         XXAMMXXAMA
         SMSMSASXSS
         SAXAMASAAA
         MAMMMXMMMM
         MXMXAXMASX",
    );
    let result = solve_part1(&test_string);

    assert_eq!(18, result)
}

#[test]
fn test_part2() {
    let test_string = String::from(
        "MMMSXXMASM
         MSAMXMSMSA
         AMXSXMAAMM
         MSAMASMSMX
         XMASAMXAMM
         XXAMMXXAMA
         SMSMSASXSS
         SAXAMASAAA
         MAMMMXMMMM
         MXMXAXMASX",
    );
    let result = solve_part2(&test_string);

    assert_eq!(9, result)
}

const DIRECTIONS: [[i32; 2]; 8] = [
    [-1, -1],
    [-1, 1],
    [1, -1],
    [1, 1],
    [-1, 0],
    [1, 0],
    [0, -1],
    [0, 1],
];

const X_DIRECTIONS: [[i32; 2]; 4] = [[-1, -1], [-1, 1], [1, -1], [1, 1]];

const SEARCH: &str = "XMAS";
const X_SEARCH: &str = "MAS";

pub fn solve_part1(data: &String) -> i32 {
    let grid: Vec<Vec<String>> = generate_grid(data);
    let mut total: i32 = 0;
    let grid_width: i32 = grid[0].len() as i32;
    let grid_height: i32 = grid.len() as i32;

    for (x, row) in (0i32..).zip(&grid) {
        for (y, letter) in (0i32..).zip(row) {
            if *letter == SEARCH.chars().nth(0).unwrap().to_string() {
                total += traverse_grid_for_word(&grid, x, y, grid_width, grid_height);
            }
        }
    }

    total
}

pub fn solve_part2(data: &String) -> i32 {
    let grid: Vec<Vec<String>> = generate_grid(data);
    let mut total: i32 = 0;
    let grid_width: i32 = grid[0].len() as i32;
    let grid_height: i32 = grid.len() as i32;

    for (x, row) in (0i32..).zip(&grid) {
        for (y, letter) in (0i32..).zip(row) {
            if *letter == X_SEARCH.chars().nth(1).unwrap().to_string() {
                if x != 0 && y != 0 && x + 1 != grid_width && y + 1 != grid_height {
                    if traverse_grid_for_x_search(&grid, x, y) {
                        total += 1
                    }
                }
            }
        }
    }

    total
}

fn generate_grid(data: &String) -> Vec<Vec<String>> {
    let mut grid: Vec<Vec<String>> = vec![];

    for line in data.lines() {
        let mut row: Vec<String> = vec![];
        for letter in line.trim().chars() {
            row.push(letter.to_string())
        }
        grid.push(row);
    }

    grid
}

fn traverse_grid_for_word(grid: &Vec<Vec<String>>, pos_x: i32, pos_y: i32, w: i32, h: i32) -> i32 {
    let mut count: i32 = 0;

    for [dir_x, dir_y] in DIRECTIONS {
        let mut found = true;

        for (i, letter) in (0i32..).zip(SEARCH.chars().enumerate()) {
            let coor_x = (pos_x + (dir_x * i)) as usize;
            let coor_y = (pos_y + (dir_y * i)) as usize;
            if coor_x >= w as usize || coor_y >= h as usize {
                found = false;
                break;
            }

            let grid_letter = &grid[coor_x][coor_y];
            if *grid_letter != letter.1.to_string() {
                found = false
            }
        }

        if found {
            count += 1;
        }
    }

    count
}

fn traverse_grid_for_x_search(grid: &Vec<Vec<String>>, pos_x: i32, pos_y: i32) -> bool {
    let top_left = &grid[(pos_x - 1) as usize][(pos_y - 1) as usize];
    let bottom_left = &grid[(pos_x - 1) as usize][(pos_y + 1) as usize];
    let top_right = &grid[(pos_x + 1) as usize][(pos_y - 1) as usize];
    let bottom_right = &grid[(pos_x + 1) as usize][(pos_y + 1) as usize];

    if top_left == "M" && bottom_right == "S" || bottom_right == "M" && top_left == "S" {
        if top_right == "M" && bottom_left == "S" || bottom_left == "M" && top_right == "S" {
            return true;
        }
    }

    false
}
