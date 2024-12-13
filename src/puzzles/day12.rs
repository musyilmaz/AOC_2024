use std::collections::HashMap;

use petgraph::{
    adj::NodeIndex, algo::condensation, dot::*, prelude::UnGraphMap, visit::IntoNodeReferences,
};

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

    assert_eq!(1930, result)
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

type Coordinate = (i32, i32);
type Map = HashMap<Coordinate, char>;
type RegionGraph = UnGraphMap<(i32, i32), ()>;

const DIRECTIONS: [Coordinate; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn solve_part1(data: &String) -> usize {
    let mut total = 0;
    let map = read_data(data);
    let region_graph = generate_map_graph(map);
    let condensed_region_graph =
        condensation(region_graph.clone().into_graph::<NodeIndex>(), false);

    for (index, node_list) in condensed_region_graph.node_references() {
        let area = node_list.len();
        let perimeter = node_list
            .iter()
            .map(|node| 4 - region_graph.neighbors(*node).count())
            .sum::<usize>();

        println!(
            "{}: {:?} {:?} {:?}",
            "❗",
            area,
            perimeter,
            area * perimeter
        );

        total += area * perimeter;
    }

    total
}

pub fn solve_part2(data: &String) -> i32 {
    0
}

fn read_data(data: &String) -> Map {
    data.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect::<Map>()
}

fn generate_map_graph(map: Map) -> RegionGraph {
    let mut region_graph: UnGraphMap<(i32, i32), ()> = UnGraphMap::new();

    for ((x, y), c) in map.iter() {
        let node = region_graph.add_node((*x, *y));

        for direction in DIRECTIONS.iter() {
            let neighbour_node = (x + direction.0, y + direction.1);
            if map
                .get(&neighbour_node)
                .is_some_and(|neighbour_char| neighbour_char == c)
            {
                region_graph.add_edge(node, neighbour_node, ());
            }
        }
    }

    region_graph
}
