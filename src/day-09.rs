use std::cmp::Ordering;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    row: usize,
    col: usize,
    value: u8,
    visited: bool,
}

type HeightMap = Vec<Vec<Point>>;

type Basin = Vec<Point>;

fn parse_input(input: &str) -> HeightMap {
    let mut height_map: HeightMap = Vec::new();
    let input_rows: Vec<&str> = input.split_terminator("\n").collect();

    for row in 0..input_rows.len() {
        height_map.push(Vec::<Point>::new());
        let input_row = input_rows[row];
        let row_bytes = input_row.as_bytes();
        for col in 0..row_bytes.len() {
            let point = Point {
                row,
                col,
                value: row_bytes[col] - 48,
                visited: false,
            };
            height_map[row].push(point);
        }
    }

    height_map
}

fn get_adjacent_points(height_map: &HeightMap, row: usize, col: usize) -> Vec<Point> {
    let row_len = height_map.len();
    let col_len = height_map[0].len();

    let mut points: Vec<Point> = Vec::new();

    if col < col_len - 1 {
        points.push(height_map[row][col + 1]);
    }
    if col > 0 {
        points.push(height_map[row][col - 1]);
    }
    if row < row_len - 1 {
        points.push(height_map[row + 1][col]);
    }
    if row > 0 {
        points.push(height_map[row - 1][col]);
    }

    points
}

fn find_low_points(height_map: &mut HeightMap) -> Vec<Point> {
    let row_len = height_map.len();
    let col_len = height_map[0].len();

    let mut low_points: Vec<Point> = Vec::new();

    for r in 0..row_len {
        for c in 0..col_len {
            let point = height_map[r][c];
            let adjacent = get_adjacent_points(height_map, r, c);
            if adjacent.iter().all(|p| p.value > point.value) {
                low_points.push(point);
            }
        }
    }

    low_points
}

mod part_1 {
    use super::*;

    pub fn calculate_sum_of_risk_level_of_low_points(height_map: &mut HeightMap) -> u32 {
        let low_points = find_low_points(height_map);
        low_points.iter().map(|p| (p.value + 1) as u32).sum()
    }
}

mod part_2 {
    use super::*;

    fn explore_basin(height_map: &mut HeightMap, row: usize, col: usize) -> Basin {
        let mut basin: Basin = Vec::new();

        let point = height_map[row][col];

        if point.visited {
            return basin;
        }

        height_map[row][col].visited = true;

        if point.value == 9 {
            return basin;
        }

        basin.push(point);

        let adj_points = get_adjacent_points(height_map, row, col);

        for adj_point in adj_points {
            basin.extend(explore_basin(height_map, adj_point.row, adj_point.col));
        }

        basin
    }

    fn find_basins(height_map: &mut HeightMap, low_points: &mut Vec<Point>) -> Vec<Basin> {
        let mut basins: Vec<Basin> = Vec::new();
        for low_point in low_points {
            basins.push(explore_basin(height_map, low_point.row, low_point.col))
        }
        basins
    }

    pub fn multiply_sizes_of_3_largest_basins(height_map: &mut HeightMap) -> u64 {
        let mut low_points = find_low_points(height_map);

        let mut basins = find_basins(height_map, &mut low_points);
        basins.sort_unstable_by(|b1, b2| {
            if b1.len() > b2.len() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        basins.iter().take(3).map(|b| b.len() as u64).product()
    }
}

fn main() {
    let mut height_map = parse_input(&read_to_string("data/day-09.txt").unwrap());

    println!("== PART 1");
    let risk_sum = part_1::calculate_sum_of_risk_level_of_low_points(&mut height_map);
    println!("Sum of risk levels: {risk_sum}");

    println!("== PART 2");
    let size_of_basins = part_2::multiply_sizes_of_3_largest_basins(&mut height_map);
    println!("Sizes of the 3 largest basins: {size_of_basins}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_height_map_parsing() {
        let height_map = parse_input(INPUT);
        assert_eq!(height_map.len(), 5);
        assert_eq!(height_map[0].len(), 10);
    }

    #[test]
    fn test_calculate_risk_level_sum() {
        let mut height_map = parse_input(INPUT);
        let risk_sum = part_1::calculate_sum_of_risk_level_of_low_points(&mut height_map);
        assert_eq!(risk_sum, 15);
    }

    #[test]
    fn test_multiply_sizes_of_3_largest_basins() {
        let mut height_map = parse_input(INPUT);
        let size_product = part_2::multiply_sizes_of_3_largest_basins(&mut height_map);
        assert_eq!(size_product, 1134);
    }
}
