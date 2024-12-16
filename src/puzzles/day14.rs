use glam::IVec2;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

#[test]
fn test_part1() {
    let test_string = String::from(
        "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
    );

    let map = IVec2::new(11, 7);
    let result = solve_part1(&test_string, map);

    assert_eq!(12, result)
}

#[test]
fn test_part2() {
    let test_string = String::from(
        "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
    );

    let map = IVec2::new(11, 7);
    let result = solve_part2(&test_string, map);

    assert_eq!(0, result)
}

#[derive(Debug)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

pub fn solve_part1(data: &String, map: IVec2) -> usize {
    let (_, robots) = parse_data(data).unwrap();

    let robots = move_robots(robots, 100, map);
    let safety_factor = generate_safety_factor(robots, map);
    safety_factor
}

pub fn solve_part2(data: &String, map: IVec2) -> usize {
    let mut frame: usize = 0;
    let mut safety_factor: Option<(usize, usize)> = None;

    let quadrants = [
        (0..(map / 2).x, 0..(map / 2).y),
        ((map / 2).x + 1..map.x, 0..(map / 2).y),
        (0..(map / 2).x, (map / 2).y + 1..map.y),
        ((map / 2).x + 1..map.x, (map / 2).y + 1..map.y),
    ];

    let (_, mut robots) = parse_data(data).unwrap();

    loop {
        // To solve the problem uncomment below and comment line with 7847 like number.
        // It is used to draw my own inputs grid on the terminal screen.
        //
        //
        //
        // if frame == map.x as usize * map.y as usize {
        if frame == 7847 {
            break;
        }

        for robot in robots.iter_mut() {
            robot.position = (robot.position + robot.velocity).rem_euclid(map);
        }

        let frame_safety_factor: usize = quadrants
            .iter()
            .map(|(xs, ys)| {
                robots
                    .iter()
                    .filter(|robot| {
                        xs.contains(&robot.position.x) && ys.contains(&robot.position.y)
                    })
                    .count()
            })
            .product();

        if safety_factor.is_none() {
            safety_factor = Some((frame, frame_safety_factor))
        } else {
            if frame_safety_factor < safety_factor.unwrap().1 {
                safety_factor = Some((frame, frame_safety_factor))
            }
        }

        frame += 1;
    }

    draw_robots(robots, map);

    frame
}

fn parse_data(data: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(
        line_ending,
        separated_pair(
            preceded(tag("p="), parse_coordinate),
            space1,
            preceded(tag("v="), parse_coordinate),
        )
        .map(|(position, velocity)| Robot { position, velocity }),
    )(data)
}

fn parse_coordinate(data: &str) -> IResult<&str, IVec2> {
    let (data, (x, y)) = separated_pair(complete::i32, tag(","), complete::i32)(data)?;
    Ok((data, IVec2::new(x, y)))
}

fn move_robots(mut robots: Vec<Robot>, size: usize, map: IVec2) -> Vec<Robot> {
    for _ in 0..size {
        for robot in robots.iter_mut() {
            robot.position = (robot.position + robot.velocity).rem_euclid(map);
        }
    }

    robots
}

fn generate_safety_factor(robots: Vec<Robot>, map: IVec2) -> usize {
    let quadrants = [
        (0..(map / 2).x, 0..(map / 2).y),
        ((map / 2).x + 1..map.x, 0..(map / 2).y),
        (0..(map / 2).x, (map / 2).y + 1..map.y),
        ((map / 2).x + 1..map.x, (map / 2).y + 1..map.y),
    ];

    quadrants
        .iter()
        .map(|(xs, ys)| {
            robots
                .iter()
                .filter(|robot| xs.contains(&robot.position.x) && ys.contains(&robot.position.y))
                .count()
        })
        .product()
}

fn draw_robots(robots: Vec<Robot>, map: IVec2) {
    let mut grid: Vec<Vec<String>> = vec![];

    for y in 0..map.y {
        let mut row = vec![];
        for x in 0..map.x {
            let robot_count = robots
                .iter()
                .filter(|robot| robot.position == IVec2::new(x, y))
                .count();
            if robot_count == 0 {
                row.push(".".to_string());
            } else {
                row.push(robot_count.to_string());
            }
        }
        grid.push(row);
    }

    for line in grid {
        println!("{}", line.join(""));
    }
}
