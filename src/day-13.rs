use std::collections::HashSet;
use std::convert::Infallible;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(u32, u32);

#[derive(Debug, PartialEq)]
pub enum Fold {
    X(u32),
    Y(u32),
}

impl FromStr for Point {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<u32> = s.split(",").map(|v| v.parse::<u32>().unwrap()).collect();
        Ok(Point(coords[0], coords[1]))
    }
}

impl FromStr for Fold {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = &s["fold along ".len()..];
        let parts: Vec<&str> = data.split("=").collect();
        if parts[0] == "x" {
            return Ok(Fold::X(parts[1].parse::<u32>().unwrap()));
        }
        Ok(Fold::Y(parts[1].parse::<u32>().unwrap()))
    }
}

fn parse_input(input: &str) -> (Vec<Point>, Vec<Fold>) {
    let mut points: Vec<Point> = Vec::new();
    let mut folds: Vec<Fold> = Vec::new();

    let lines: Vec<&str> = input.split_terminator("\n").collect();

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with("fold") {
            folds.push(Fold::from_str(line).unwrap());
        } else {
            points.push(Point::from_str(line).unwrap());
        }
    }

    (points, folds)
}

fn fold_on_x(p: &Point, x: u32) -> Point {
    Point(x * 2 - p.0, p.1)
}

fn fold_on_y(p: &Point, y: u32) -> Point {
    Point(p.0, y * 2 - p.1)
}

fn apply_fold(points: &mut [Point], fold: &Fold) -> Vec<Point> {
    let mut point_set: HashSet<Point> = HashSet::new();
    match fold {
        Fold::X(x) => {
            for p in points {
                if p.0 < *x {
                    point_set.insert(*p);
                } else if p.0 > *x {
                    point_set.insert(fold_on_x(&p, *x));
                }
            }
        }
        Fold::Y(y) => {
            for p in points {
                if p.1 < *y {
                    point_set.insert(*p);
                } else if p.1 > *y {
                    point_set.insert(fold_on_y(&p, *y));
                }
            }
        }
    }
    point_set.into_iter().collect()
}

fn print_points(points: &[Point]) {
    let mut max_x = 0u32;
    let mut max_y = 0u32;
    for p in points {
        if p.0 > max_x {
            max_x = p.0;
        }
        if p.1 > max_y {
            max_y = p.1;
        }
    }
    let x_size = (max_x + 1) as usize;
    let y_size = (max_y + 1) as usize;
    let mut grid: Vec<Vec<bool>> = vec![vec![false; x_size]; y_size];
    for p in points {
        grid[p.1 as usize][p.0 as usize] = true;
    }
    for y in 0..y_size {
        for x in 0..x_size {
            if grid[y][x] == true {
                print!("█");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

mod part_1 {
    use super::*;

    pub fn dot_count_after_first_fold(points: &mut Vec<Point>, fold: &Fold) -> usize {
        apply_fold(points, fold).len()
    }
}

mod part_2 {
    use super::*;

    pub fn apply_folds(mut points: Vec<Point>, folds: &[Fold]) {
        let folds_len = folds.len();
        for f in 0..folds_len {
            let fold = &folds[f];
            points = apply_fold(&mut points, fold);
            if f == folds_len - 1 {
                print_points(&points);
            }
        }
    }
}

fn main() {
    let (mut points, folds) = parse_input(&read_to_string("data/day-13.txt").unwrap());

    let mut mx = 0;
    let mut my = 0;
    for p in points.iter() {
        if p.0 > mx { mx = p.0; }
        if p.1 > my { my = p.1; }
    }
    println!("max x: {mx}");
    println!("max y: {my}");

    println!("== PART 1");
    let dot_count = part_1::dot_count_after_first_fold(&mut points, &folds[0]);
    println!("Number of visible points after first fold: {dot_count}");

    println!("== PART 2");
    println!("Code to activate the infrared thermal imaging camera system:");
    part_2::apply_folds(points.clone(), &folds);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1,2
10,20
100,200

fold along x=50
fold along y=100";

    #[test]
    fn points_should_be_comparable() {
        let p_1 = Point(2, 3);
        let p_2 = Point(5, 1);
        let p_3 = Point(2, 3);
        assert_ne!(p_1, p_2);
        assert_eq!(p_1, p_3);
    }

    #[test]
    fn point_parsing() {
        let input = "10,17";
        let p = Point::from_str(input).unwrap();
        assert_eq!(p, Point(10, 17));
    }

    #[test]
    fn fold_parsing() {
        let input = "fold along x=110";
        let f = Fold::from_str(input).unwrap();
        assert_eq!(f, Fold::X(110));
    }

    #[test]
    fn input_parsing() {
        let (points, folds) = parse_input(INPUT);
        assert_eq!(points.len(), 3);
        assert_eq!(points, vec![Point(1, 2), Point(10, 20), Point(100, 200)]);
        assert_eq!(folds.len(), 2);
        assert_eq!(folds, vec![Fold::X(50), Fold::Y(100)]);
    }

    #[test]
    fn point_folding_on_x() {
        let p = Point(3, 1);
        let folded = fold_on_x(&p, 2);
        assert_eq!(folded, Point(1, 1));
    }

    #[test]
    fn point_folding_on_y() {
        let p = Point(1, 2);
        let folded = fold_on_y(&p, 1);
        assert_eq!(folded, Point(1, 0));
    }

    #[test]
    fn printing_points() {
        let (points, _) = parse_input(INPUT);
        print_points(&points);
    }
}
