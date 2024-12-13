use itertools::Itertools;
use std::collections::HashMap;

use petgraph::{
    adj::NodeIndex, algo::condensation, prelude::UnGraphMap, visit::IntoNodeReferences,
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

    assert_eq!(1206, result)
}

type Coordinate = (i32, i32);
type Map = HashMap<Coordinate, char>;
type RegionGraph = UnGraphMap<(i32, i32), ()>;

const DIRECTIONS: [Coordinate; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn solve_part1(data: &String) -> usize {
    let mut total = 0;
    let map = read_data(data);
    let region_graph = generate_map_graph(&map);
    let condensed_region_graph =
        condensation(region_graph.clone().into_graph::<NodeIndex>(), false);

    for (_, node_list) in condensed_region_graph.node_references() {
        let area = node_list.len();
        let perimeter = node_list
            .iter()
            .map(|node| 4 - region_graph.neighbors(*node).count())
            .sum::<usize>();

        total += area * perimeter;
    }

    total
}

pub fn solve_part2(data: &String) -> usize {
    let mut total = 0;
    let map = read_data(data);
    let region_graph = generate_map_graph(&map);
    let condensed_region_graph =
        condensation(region_graph.clone().into_graph::<NodeIndex>(), false);

    for (_, node_list) in condensed_region_graph.node_references() {
        let area = node_list.len();
        let sides = node_list
            .iter()
            .map(|node| calculate_corners(node, &map))
            .sum::<usize>();

        total += area * sides;
    }
    total
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

fn generate_map_graph(map: &Map) -> RegionGraph {
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

fn calculate_corners(node: &Coordinate, map: &Map) -> usize {
    let mut corners = 0;

    for (dir1, dir2) in DIRECTIONS
        .iter()
        .circular_tuple_windows::<(&Coordinate, &Coordinate)>()
    {
        let node_1 = (node.0 + dir1.0, node.1 + dir1.1);
        let node_2 = (node.0 + dir2.0, node.1 + dir2.1);
        let diagonal_node = (node.0 + dir1.0 + dir2.0, node.1 + dir1.1 + dir2.1);

        let node_char = map.get(node).unwrap();
        let is_same_node_with_node_1 = map.get(&node_1).is_some_and(|c1| c1 == node_char);
        let is_same_node_with_node_2 = map.get(&node_2).is_some_and(|c2| c2 == node_char);
        let is_not_same_node_with_diagonal =
            map.get(&diagonal_node).is_some_and(|cd| cd != node_char);

        if is_same_node_with_node_1 && is_same_node_with_node_2 && is_not_same_node_with_diagonal {
            corners += 1;
        }
        if !is_same_node_with_node_1 && !is_same_node_with_node_2 {
            corners += 1;
        }
    }

    corners
}
