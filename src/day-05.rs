use std::cmp::{min, Ordering, PartialOrd};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct LocId(String);

impl LocId {
    fn new(x: u16, y: u16) -> LocId {
        LocId(format!("{},{}", x, y))
    }
}

type Count = u16;
type DiagramMap = HashMap<LocId, Count>;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug, PartialEq)]
pub struct Line {
    p1: Point,
    p2: Point,
}

impl FromStr for Point {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s
            .split(",")
            .map(|v| v.parse::<u16>().unwrap())
            .collect::<Vec<u16>>();
        Ok(Point {
            x: coords[0],
            y: coords[1],
        })
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        min(self.p1.x, self.p2.x).partial_cmp(&min(other.p1.x, other.p2.x))
    }
}

impl FromStr for Line {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .split("->")
            .map(|p| p.trim().parse::<Point>().unwrap())
            .collect::<Vec<Point>>();
        Ok(Line::new(points[0], points[1]))
    }
}

impl Line {
    fn new(p1: Point, p2: Point) -> Line {
        if p1.x <= p2.x {
            Line { p1, p2 }
        } else {
            Line { p1: p2, p2: p1 }
        }
    }

    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn get_loc_ids(&self) -> Vec<LocId> {
        let mut loc_ids = Vec::<LocId>::new();
        if self.is_horizontal() {
            let y = self.p1.y;
            for x in self.p1.x..=self.p2.x {
                loc_ids.push(LocId::new(x, y));
            }
        } else if self.is_vertical() {
            let x = self.p1.x;
            let (mut y1, mut y2) = (self.p1.y, self.p2.y);
            if self.p1.y > self.p2.y {
                std::mem::swap(&mut y1, &mut y2);
            }
            for y in y1..=y2 {
                loc_ids.push(LocId::new(x, y));
            }
        } else {
            // diagonal case (part 2)
            let (x1, x2) = (self.p1.x, self.p2.x);
            let mut y = self.p1.y;
            let y_inc: i8 = if self.p1.y <= self.p2.y { 1 } else { -1 };
            for x in x1..=x2 {
                loc_ids.push(LocId::new(x, y));
                if y_inc > 0 {
                    y += 1;
                } else {
                    y -= 1;
                };
            }
        }
        loc_ids
    }
}

fn parse_lines(input: &str) -> Vec<Line> {
    input
        .trim()
        .split("\n")
        .map(|l| l.parse::<Line>().unwrap())
        .collect::<Vec<Line>>()
}

fn make_diagram_map(lines: &Vec<Line>) -> DiagramMap {
    let mut diagram_map = DiagramMap::new();
    for line in lines {
        let loc_ids = line.get_loc_ids();
        for loc_id in loc_ids {
            *diagram_map.entry(loc_id).or_insert(0) += 1;
        }
    }
    diagram_map
}

mod part_1 {
    use super::*;

    pub fn get_orthogonal_lines(input: &str) -> Vec<Line> {
        let lines = parse_lines(input);
        let lines = lines
            .into_iter()
            .filter(|l| l.is_horizontal() || l.is_vertical())
            .collect();
        lines
    }

    pub fn count_intersections(diagram_map: &DiagramMap) -> usize {
        diagram_map
            .values()
            .filter(|v| **v > 1u16)
            .map(|v| *v as u32)
            .count()
    }
}

mod part_2 {
    use super::*;

    pub fn get_all_lines(input: &str) -> Vec<Line> {
        parse_lines(input)
    }
}

fn main() {
    println!("== PART 1");
    let lines = part_1::get_orthogonal_lines(&std::fs::read_to_string("data/day-05.txt").unwrap());
    let diagram_map = make_diagram_map(&lines);
    let intersection_count = part_1::count_intersections(&diagram_map);
    println!("Intersection count: {}", intersection_count);

    println!("== PART 2");
    let lines = part_2::get_all_lines(&std::fs::read_to_string("data/day-05.txt").unwrap());
    let diagram_map = make_diagram_map(&lines);
    let intersection_count = part_1::count_intersections(&diagram_map);
    println!("Intersection count: {}", intersection_count);
}

#[cfg(test)]
mod tests {
    use super::part_1::*;
    use super::part_2::*;
    use super::*;

    const INPUT: &str = "880,493 -> 880,58
937,831 -> 131,25
520,921 -> 476,965
760,147 -> 461,147
646,108 -> 646,27
99,906 -> 99,591";

    #[test]
    fn points_with_same_coords_should_be_equal() {
        let p1 = Point { x: 5, y: 7 };
        let p2 = Point { x: 5, y: 7 };
        assert_eq!(p1, p2);
    }

    #[test]
    fn points_with_different_coords_should_not_be_equal() {
        let p1 = Point { x: 5, y: 7 };
        let p2 = Point { x: 5, y: 9 };
        assert_ne!(p1, p2);
    }

    #[test]
    fn lines_should_be_sortable_based_on_their_first_x_coord() {
        let line_1 = Line {
            p1: Point { x: 3, y: 7 },
            p2: Point { x: 8, y: 7 },
        };
        let line_2 = Line {
            p1: Point { x: 5, y: 2 },
            p2: Point { x: 1, y: 2 },
        };
        let line_3 = Line {
            p1: Point { x: 2, y: 3 },
            p2: Point { x: 2, y: 6 },
        };
        let lines: &mut [&Line] = &mut [&line_1, &line_2, &line_3];
        lines.sort_unstable_by(|l1, l2| l1.partial_cmp(l2).unwrap());
        assert_eq!(lines, [&line_2, &line_3, &line_1]);
    }

    #[test]
    fn pattern_x_y_arrow_x_y_should_be_parsed_into_a_line() {
        let input = "10,20 -> 30,40";
        let line = input.parse::<Line>().unwrap();
        assert_eq!(line.p1, Point { x: 10, y: 20 });
        assert_eq!(line.p2, Point { x: 30, y: 40 });
    }

    #[test]
    fn the_first_point_of_a_line_should_be_the_one_with_smaller_x() {
        let input = "30,20 -> 10,40";
        let line = input.parse::<Line>().unwrap();
        assert_eq!(line.p1, Point { x: 10, y: 40 });
        assert_eq!(line.p2, Point { x: 30, y: 20 });
    }

    #[test]
    fn input_should_be_parsed_into_list_of_lines() {
        let lines = parse_lines(INPUT);
        assert_eq!(lines.len(), 6);
    }

    #[test]
    fn a_line_should_be_able_to_produce_a_list_of_its_loc_ids() {
        let line = Line::new(Point { x: 3, y: 5 }, Point { x: 7, y: 5 });
        let loc_ids = line.get_loc_ids();
        assert_eq!(loc_ids.len(), 5);
        assert_eq!(loc_ids[0], LocId::new(3, 5));
        assert_eq!(loc_ids[1], LocId::new(4, 5));
        assert_eq!(loc_ids[2], LocId::new(5, 5));
        assert_eq!(loc_ids[3], LocId::new(6, 5));
        assert_eq!(loc_ids[4], LocId::new(7, 5));
    }

    #[test]
    fn intersections_happen_when_diagram_map_value_is_greater_than_1() {
        let lines: Vec<Line> = vec![
            Line::new(Point { x: 2, y: 3 }, Point { x: 5, y: 3 }),
            Line::new(Point { x: 3, y: 0 }, Point { x: 3, y: 10 }),
            Line::new(Point { x: 0, y: 6 }, Point { x: 9, y: 6 }),
        ];
        let diagram_map = make_diagram_map(&lines);
        let intersections = count_intersections(&diagram_map);
        assert_eq!(intersections, 2);
    }

    mod part_1 {
        use super::*;

        #[test]
        fn a_diagram_map_should_hold_loc_ids_and_count_of_ocurrences() {
            let lines = get_orthogonal_lines(INPUT);
            let diagram_map = make_diagram_map(&lines);
            let loc_id_count = lines.iter().map(|l| l.get_loc_ids().len()).sum();
            assert_eq!(diagram_map.len(), loc_id_count);
        }

        #[test]
        fn get_lines_should_filter_out_non_orthogonal_lines() {
            let lines = part_1::get_orthogonal_lines(INPUT);
            assert_eq!(lines.len(), 4);
            assert!(lines.iter().all(|l| l.is_horizontal() || l.is_vertical()));
        }
    }

    mod part_2 {
        use super::*;

        #[test]
        fn loc_ids_should_be_calculated_for_descending_diagnoal_lines() {
            let line = Line::new(Point { x: 2, y: 1 }, Point { x: 4, y: 3 });
            let loc_ids = line.get_loc_ids();
            assert_eq!(loc_ids.len(), 3);
            assert_eq!(loc_ids[0], LocId::new(2, 1));
            assert_eq!(loc_ids[1], LocId::new(3, 2));
            assert_eq!(loc_ids[2], LocId::new(4, 3));
        }

        #[test]
        fn a_diagram_map_should_hold_loc_ids_and_count_of_ocurrences() {
            let lines = get_all_lines(INPUT);
            let diagram_map = make_diagram_map(&lines);
            let loc_id_count = lines.iter().map(|l| l.get_loc_ids().len()).sum();
            assert_eq!(diagram_map.len(), loc_id_count);
        }

        #[test]
        fn loc_ids_should_be_calculated_for_ascending_diagnoal_lines() {
            let line = Line::new(Point { x: 1, y: 10 }, Point { x: 4, y: 7 });
            let loc_ids = line.get_loc_ids();
            assert_eq!(loc_ids.len(), 4);
            assert_eq!(loc_ids[0], LocId::new(1, 10));
            assert_eq!(loc_ids[1], LocId::new(2, 9));
            assert_eq!(loc_ids[2], LocId::new(3, 8));
            assert_eq!(loc_ids[3], LocId::new(4, 7));
        }
    }
}
