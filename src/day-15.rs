use std::collections::{BinaryHeap, HashSet};
use std::fs::read_to_string;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NodeInfo {
    position: Position,
    edge_cost: u8,
    dist_from_src: usize,
}

impl NodeInfo {
    fn default() -> Self {
        NodeInfo {
            position: Position { row: 0, col: 0 },
            edge_cost: 0,
            dist_from_src: usize::MAX,
        }
    }
}

// https://doc.rust-lang.org/std/collections/binary_heap/index.html
impl Ord for NodeInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .dist_from_src
            .cmp(&self.dist_from_src)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for NodeInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct RiskLevelMap {
    nodes: Vec<Vec<NodeInfo>>,
    width: usize,
    height: usize,
}

impl RiskLevelMap {
    fn new(nodes: Vec<Vec<NodeInfo>>) -> Self {
        let width = &nodes[0].len();
        let height = &nodes.len();
        Self {
            nodes,
            width: *width,
            height: *height,
        }
    }
}

fn parse_input(input: &str) -> RiskLevelMap {
    let rows_str: Vec<&str> = input.split_terminator("\n").collect();
    let mut nodes = Vec::<Vec<NodeInfo>>::new();
    let mut row_idx = 0;
    for row_str in rows_str {
        let row: Vec<NodeInfo> = row_str
            .chars()
            .enumerate()
            .map(|(col_idx, c)| NodeInfo {
                position: Position {
                    row: row_idx,
                    col: col_idx,
                },
                edge_cost: c.to_digit(10).unwrap() as u8,
                dist_from_src: usize::MAX,
            })
            .collect();
        nodes.push(row);
        row_idx += 1;
    }
    RiskLevelMap::new(nodes)
}

fn candidate_neighbors(
    map: &RiskLevelMap,
    position: &Position,
    finished_positions: &HashSet<Position>,
) -> Vec<Position> {
    let mut candidates = Vec::new();

    // left
    if position.col > 0 {
        let candidate = Position {
            row: position.row,
            col: position.col - 1,
        };
        if !finished_positions.contains(&candidate) {
            candidates.push(candidate);
        }
    }
    // top
    if position.row > 0 {
        let candidate = Position {
            row: position.row - 1,
            col: position.col,
        };
        if !finished_positions.contains(&candidate) {
            candidates.push(candidate);
        }
    }
    // right
    if position.col < map.width - 1 {
        let candidate = Position {
            row: position.row,
            col: position.col + 1,
        };
        if !finished_positions.contains(&candidate) {
            candidates.push(candidate);
        }
    }
    // bottom
    if position.row < map.height - 1 {
        let candidate = Position {
            row: position.row + 1,
            col: position.col,
        };
        if !finished_positions.contains(&candidate) {
            candidates.push(candidate);
        }
    }

    candidates
}

fn shortest_path_to_bottom_right(map: &mut RiskLevelMap) {
    let end_pos = Position {
        row: map.height - 1,
        col: map.width - 1,
    };

    map.nodes[0][0].dist_from_src = 0;

    let mut heap: BinaryHeap<NodeInfo> = BinaryHeap::new();
    heap.push(map.nodes[0][0].clone());

    let mut finished_positions = HashSet::new();
    while let Some(curr_node_info) = heap.pop() {
        let neighbor_positions =
            candidate_neighbors(&map, &curr_node_info.position, &finished_positions);
        for neighbor_position in neighbor_positions {
            let neighbor_info = map.nodes[neighbor_position.row][neighbor_position.col];
            let cmp_dist = curr_node_info.dist_from_src + neighbor_info.edge_cost as usize;
            if neighbor_info.dist_from_src > cmp_dist {
                map.nodes[neighbor_position.row][neighbor_position.col].dist_from_src = cmp_dist;
                heap.push(map.nodes[neighbor_position.row][neighbor_position.col].clone());
                if neighbor_info.position == end_pos {
                    return;
                }
            }
        }
        finished_positions.insert(curr_node_info.position);
    }
}

mod part_2 {
    use super::*;

    const EXPANSION_FACTOR: usize = 5;

    pub fn expand_map(map: &RiskLevelMap) -> RiskLevelMap {
        let new_height = EXPANSION_FACTOR * map.height;
        let new_width = EXPANSION_FACTOR * map.width;
        let mut new_nodes = vec![vec![NodeInfo::default(); new_width]; new_height];
        for row in 0..new_height {
            let src_row = row % map.height;
            let vertical_cost_increase = (row / map.height) as u8;
            for col in 0..new_width {
                let src_col = col % map.width;
                let horizontal_cost_increase = (col / map.width) as u8;
                let mut new_cost = map.nodes[src_row][src_col].edge_cost
                    + vertical_cost_increase
                    + horizontal_cost_increase;
                if new_cost > 9 {
                    new_cost -= 9;
                }
                new_nodes[row][col].edge_cost = new_cost;
                new_nodes[row][col].position = Position { row, col };
            }
        }
        RiskLevelMap::new(new_nodes)
    }
}

fn main() {
    println!("== PART 1");
    let mut map = parse_input(&read_to_string("data/day-15.txt").unwrap());
    shortest_path_to_bottom_right(&mut map);
    println!(
        "Minimum cost to get to the bottom right position: {}",
        map.nodes[map.height - 1][map.width - 1].dist_from_src
    );

    println!();

    println!("== PART 2");
    let map = parse_input(&read_to_string("data/day-15.txt").unwrap());
    let mut expanded_map = part_2::expand_map(&map);
    shortest_path_to_bottom_right(&mut expanded_map);
    println!(
        "Minimum cost to get to the bottom right position: {}",
        expanded_map.nodes[expanded_map.height - 1][expanded_map.width - 1].dist_from_src
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_data() -> RiskLevelMap {
        parse_input(&read_to_string("data/day-15-example.txt").unwrap())
        // parse_input(&read_to_string("data/day-15-min-example.txt").unwrap())
    }

    #[test]
    fn test_input_parsing() {
        let map = example_data();
        assert_eq!(10, map.width);
        assert_eq!(10, map.height);

        let top_left = map.nodes[0][0];
        assert_eq!(Position { row: 0, col: 0 }, top_left.position);
        assert_eq!(usize::MAX, top_left.dist_from_src);

        let top_right = map.nodes[0][map.width - 1];
        assert_eq!(
            Position {
                row: 0,
                col: map.width - 1
            },
            top_right.position
        );
        assert_eq!(usize::MAX, top_right.dist_from_src);

        let bottom_left = map.nodes[map.height - 1][0];
        assert_eq!(
            Position {
                row: map.height - 1,
                col: 0
            },
            bottom_left.position
        );
        assert_eq!(usize::MAX, bottom_left.dist_from_src);

        let bottom_right = map.nodes[map.height - 1][map.width - 1];
        assert_eq!(
            Position {
                row: map.height - 1,
                col: map.width - 1
            },
            bottom_right.position
        );
        assert_eq!(usize::MAX, bottom_right.dist_from_src);
    }

    #[test]
    fn test_candidate_neighbors_from_top_left() {
        let map = example_data();
        let position = Position { row: 0, col: 0 };
        let finished_positions = HashSet::new();
        let neighbors = candidate_neighbors(&map, &position, &finished_positions);
        assert_eq!(2, neighbors.len());
        assert!(neighbors.contains(&Position { row: 0, col: 1 }));
        assert!(neighbors.contains(&Position { row: 1, col: 0 }));
    }

    #[test]
    fn test_candidate_neighbors_from_first_row_second_col() {
        let map = example_data();
        let position = Position { row: 0, col: 1 };
        let finished_positions = HashSet::new();
        let neighbors = candidate_neighbors(&map, &position, &finished_positions);
        assert_eq!(3, neighbors.len());
        assert!(neighbors.contains(&Position { row: 0, col: 0 }));
        assert!(neighbors.contains(&Position { row: 0, col: 2 }));
        assert!(neighbors.contains(&Position { row: 1, col: 1 }));
    }

    #[test]
    fn test_node_info_should_be_ordered_based_on_dist_from_src() {
        let node_1 = NodeInfo {
            position: Position { row: 5, col: 5 },
            edge_cost: 0,
            dist_from_src: 10,
        };
        let node_2 = NodeInfo {
            position: Position { row: 9, col: 3 },
            edge_cost: 0,
            dist_from_src: 8,
        };
        let node_3 = NodeInfo {
            position: Position { row: 7, col: 1 },
            edge_cost: 0,
            dist_from_src: 6,
        };
        let node_4 = NodeInfo {
            position: Position { row: 6, col: 2 },
            edge_cost: 0,
            dist_from_src: 7,
        };
        let mut heap: BinaryHeap<NodeInfo> = BinaryHeap::new();
        heap.push(node_1);
        heap.push(node_2);
        heap.push(node_3);
        heap.push(node_4);

        assert_eq!(6, heap.pop().unwrap().dist_from_src);
        assert_eq!(7, heap.pop().unwrap().dist_from_src);
        assert_eq!(8, heap.pop().unwrap().dist_from_src);
        assert_eq!(10, heap.pop().unwrap().dist_from_src);
    }

    #[test]
    fn test_shortest_path() {
        let mut map = example_data();
        shortest_path_to_bottom_right(&mut map);

        println!(
            "\nmin dist to bottom right: {:?}",
            map.nodes[map.width - 1][map.height - 1]
        );
    }
}
