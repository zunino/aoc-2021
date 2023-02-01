use std::fs::read_to_string;

type Octopuses = Vec<Vec<u8>>;

#[derive(Debug, PartialEq)]
struct Point(i16, i16);

fn parse_input(input: &str) -> Octopuses {
    let mut octopuses: Octopuses = Vec::new();

    let lines = input.split_terminator("\n");
    for line in lines {
        octopuses.push(Vec::from_iter(line.bytes().map(|b| (b - 48) as u8)));
    }

    octopuses
}

fn _print_octopuses(octopuses: &Octopuses) {
    for row in octopuses {
        println!("{:?}", row);
    }
}

fn get_adjacent_points(octopuses: &Octopuses, base: Point) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let rows = octopuses.len() as i16;
    let cols = octopuses[0].len() as i16;
    let r = base.0;
    let c = base.1;

    for i in r - 1..(r + 2) {
        for j in c - 1..(c + 2) {
            points.push(Point(i, j));
        }
    }

    points.retain(|p| p.0 > -1 && p.1 > -1 && p.0 < rows && p.1 < cols && *p != base);

    points
}

fn flash(octopuses: &mut Octopuses, base: Point, flashed: &[Point]) {
    let points = get_adjacent_points(octopuses, base);
    for point in points {
        if flashed.contains(&point) {
            continue;
        }
        let row = point.0 as usize;
        let col = point.1 as usize;
        octopuses[row][col] += 1;
    }
}

fn increment_all_energy_levels(octopuses: &mut Octopuses) {
    for row in octopuses {
        for octopus in row {
            *octopus += 1;
        }
    }
}

fn flash_em_up(octopuses: &mut Octopuses) -> u64 {
    let mut flash_count = 0u64;
    let mut flashed: Vec<Point> = Vec::new();

    loop {
        let orig_flash_count = flash_count;
        for row in 0..octopuses.len() {
            for col in 0..octopuses[0].len() {
                let point = Point(row as i16, col as i16);
                if flashed.contains(&point) {
                    continue;
                }
                if octopuses[row][col] > 9 {
                    flash(octopuses, Point(row as i16, col as i16), &flashed);
                    flashed.push(point);
                    flash_count += 1;
                    octopuses[row][col] = 0;
                }
            }
        }
        if flash_count == orig_flash_count {
            break;
        }
    }

    flash_count
}

mod part_1 {
    use super::*;

    pub fn count_total_flashes(mut octopuses: Octopuses, steps: u8) -> u64 {
        let mut total_flashes = 0u64;
        for _ in 0..steps {
            increment_all_energy_levels(&mut octopuses);
            total_flashes += flash_em_up(&mut octopuses);
        }
        total_flashes
    }
}

mod part_2 {
    use super::*;

    pub fn find_first_step_where_all_flash_simultaneously(
        mut octopuses: Octopuses,
        steps: u8,
    ) -> u8 {
        let octopuses_count = octopuses.len() * octopuses[0].len();
        for s in 0..steps {
            increment_all_energy_levels(&mut octopuses);
            let flash_count = flash_em_up(&mut octopuses);
            if flash_count == octopuses_count as u64 {
                return s + 1;
            }
        }
        return 0;
    }
}

fn main() {
    let octopuses = parse_input(&read_to_string("data/day-11.txt").unwrap());

    println!("== PART 1");
    let total_flashes = part_1::count_total_flashes(octopuses.clone(), 100);
    println!("Total flashes after 100 steps: {total_flashes}");

    println!("== PART 2");
    let step = part_2::find_first_step_where_all_flash_simultaneously(octopuses.clone(), 255);
    println!("First step where all flash simultaneously: {step}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "237163
893210
032265
235498";

    #[test]
    fn input_parsing_into_2d_vec() {
        let input = "123
456";
        let octopuses = parse_input(input);
        assert_eq!(octopuses.len(), 2);
        assert_eq!(octopuses[0].len(), 3);
    }

    #[test]
    fn finding_adjacent_points_of_the_left_corner_point() {
        let octopuses = parse_input(INPUT);
        let points = get_adjacent_points(&octopuses, Point(0, 0));
        assert_eq!(points, vec![Point(0, 1), Point(1, 0), Point(1, 1)]);
    }

    #[test]
    fn finding_adjacent_points_of_a_middle_point() {
        let octopuses = parse_input(INPUT);
        let points = get_adjacent_points(&octopuses, Point(2, 3));
        assert_eq!(
            points,
            vec![
                Point(1, 2),
                Point(1, 3),
                Point(1, 4),
                Point(2, 2),
                Point(2, 4),
                Point(3, 2),
                Point(3, 3),
                Point(3, 4)
            ]
        );
    }

    #[test]
    fn finding_adjacent_points_of_the_right_corner_point() {
        let octopuses = parse_input(INPUT);
        let points = get_adjacent_points(&octopuses, Point(3, 5));
        assert_eq!(points, vec![Point(2, 4), Point(2, 5), Point(3, 4)]);
    }

    #[test]
    fn increment_of_all_energy_levels() {
        let mut octopuses = parse_input(INPUT);
        increment_all_energy_levels(&mut octopuses);
        assert_eq!(octopuses[0], [3, 4, 8, 2, 7, 4]);
        assert_eq!(octopuses[3], [3, 4, 6, 5, 10, 9]);
    }

    #[test]
    fn total_number_of_flashes() {
        let input = "183
456";
        let octopuses = parse_input(input);
        let total_flashes = part_1::count_total_flashes(octopuses, 3);
        assert_eq!(total_flashes, 3);
    }

    #[test]
    fn total_number_of_flashes_problem_example() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let octopuses = parse_input(input);
        let total_flashes = part_1::count_total_flashes(octopuses, 100);
        assert_eq!(total_flashes, 1656);
    }
}
