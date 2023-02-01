use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Vec2 {
    x: i32,
    y: i32,
}

type Velocity = Vec2;
type Position = Vec2;

#[derive(Debug, PartialEq)]
pub struct Area {
    p1: Position,
    p2: Position,
}

impl Area {
    fn contains(&self, pos: &Position) -> bool {
        pos.x >= self.p1.x && pos.x <= self.p2.x && pos.y <= self.p1.y && pos.y >= self.p2.y
    }
}

#[derive(Debug)]
pub enum LaunchResult {
    TargetHit {
        positions: Vec<Position>,
        launch_velocity: Velocity,
        maximum_y: i32,
        valid_velocities: usize,
    },
    TargetMissed {
        position: Position,
    },
}

const INITIAL_POSITION: Position = Position { x: 0, y: 0 };

fn parse_input(input: &str) -> Area {
    let number_pairs: Vec<&str> = input["target area: x=".len()..].split(", y=").collect();

    let numbers: Vec<_> = number_pairs
        .into_iter()
        .map(|p| p.split(".."))
        .flatten()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    Area {
        p1: Position {
            x: numbers[0],
            y: numbers[3],
        },
        p2: Position {
            x: numbers[1],
            y: numbers[2],
        },
    }
}

fn simulate_launch_step(position: &Position, velocity: &mut Velocity) -> Position {
    let mut new_position = position.clone();
    new_position.x += velocity.x;
    new_position.y += velocity.y;
    if velocity.x != 0 {
        if velocity.x > 0 {
            velocity.x -= 1
        } else {
            velocity.x += 1
        };
    }
    velocity.y -= 1;
    new_position
}

fn simulate_launch(target_area: &Area, mut velocity: Velocity) -> LaunchResult {
    let mut position = INITIAL_POSITION;
    let mut positions = Vec::new();
    positions.push(INITIAL_POSITION);
    let launch_velocity = velocity.clone();
    let mut maximum_y = i32::MIN;
    while position.x <= target_area.p2.x && position.y >= target_area.p2.y {
        position = simulate_launch_step(&position, &mut velocity);
        positions.push(position.clone());
        if position.y > maximum_y {
            maximum_y = position.y;
        }
        if target_area.contains(&position) {
            return LaunchResult::TargetHit {
                positions,
                launch_velocity,
                maximum_y,
                valid_velocities: 1,
            };
        }
    }
    return LaunchResult::TargetMissed { position };
}

pub fn find_launch_velocity_that_hits_target_and_reaches_highest_y(
    target_area: &Area,
) -> LaunchResult {
    let mut hit_maximum_y = i32::MIN;
    let mut hit_positions = Vec::new();
    let mut hit_launch_velocity = Velocity { x: 0, y: 0 };
    let mut valid_launch_velocities = HashSet::new();
    for y_velocity in -200..200 {
        for x_velocity in 1..300 {
            let velocity = Velocity {
                x: x_velocity,
                y: y_velocity,
            };
            let result = simulate_launch(target_area, velocity.clone());
            if let LaunchResult::TargetHit {
                positions,
                launch_velocity,
                maximum_y,
                valid_velocities: _,
            } = result
            {
                valid_launch_velocities.insert(launch_velocity.clone());
                if maximum_y > hit_maximum_y {
                    hit_positions = positions;
                    hit_launch_velocity = launch_velocity;
                    hit_maximum_y = maximum_y;
                }
            }
        }
    }
    LaunchResult::TargetHit {
        positions: hit_positions,
        launch_velocity: hit_launch_velocity,
        maximum_y: hit_maximum_y,
        valid_velocities: valid_launch_velocities.len(),
    }
}

fn draw_launch_diagram(target_area: &Area, positions: &Vec<Position>) {
    let min_y = min(
        positions
            .iter()
            .map(|pos| pos.y)
            .min_by(|y1, y2| y1.partial_cmp(&y2).unwrap())
            .unwrap(),
        target_area.p2.y,
    );
    let max_y = max(
        positions
            .iter()
            .map(|pos| pos.y)
            .min_by(|y1, y2| y2.partial_cmp(&y1).unwrap())
            .unwrap(),
        target_area.p1.y,
    );
    let min_x = positions[0].x;
    let max_x = max(
        positions
            .iter()
            .map(|pos| pos.x)
            .min_by(|x1, x2| x2.partial_cmp(&x1).unwrap())
            .unwrap(),
        target_area.p2.x,
    );

    let cols = max_x - min_x + 1;
    if cols > 50 {
        println!("Diagram exceeds maximum size");
        return;
    }

    let mut sorted_positions = positions.clone();
    sorted_positions.sort_by(|pos_1, pos_2| {
        return pos_2.y.partial_cmp(&pos_1.y).unwrap();
    });
    sorted_positions.dedup();

    print!("    ");
    for col in (0..cols).step_by(10) {
        print!("{:<20}", col / 10);
    }
    println!();
    print!("    ");
    for col in 0..cols {
        print!("{} ", col % 10);
    }
    println!();
    let mut pos_idx = 0;
    for row in (min_y..=max_y).rev() {
        print!("{row:3} ");
        for col in min_x..=max_x {
            if pos_idx < sorted_positions.len() {
                let pos = &sorted_positions[pos_idx];
                if pos.y == row && pos.x == col {
                    let mut probe_char = '\u{25ce}';
                    if pos == &INITIAL_POSITION {
                        probe_char = '\u{25c9}';
                    }
                    print!("{probe_char} ");
                    pos_idx += 1;
                    continue;
                }
            }
            if target_area.contains(&Position { x: col, y: row }) {
                print!("{} ", '\u{25cb}');
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

fn main() {
    let input = &read_to_string("data/day-17.txt").unwrap();
    let target_area = parse_input(input.trim());
    let result = find_launch_velocity_that_hits_target_and_reaches_highest_y(&target_area);

    if let LaunchResult::TargetHit {
        positions,
        launch_velocity,
        maximum_y,
        valid_velocities,
    } = result
    {
        println!("== PART 1");
        println!("Maximum y: {}", maximum_y);
        println!("Launch velocity: {:?}", launch_velocity);

        println!();

        println!("== PART 2");
        println!("Valid velocities that hit the target: {valid_velocities}");

        //draw_launch_diagram(&target_area, &positions);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        let input = "target area: x=-5..2, y=0..10";
        let area = parse_input(input);
        let expected = Area {
            p1: Position { x: -5, y: 10 },
            p2: Position { x: 2, y: 0 },
        };
        assert_eq!(expected, area);
    }

    #[test]
    fn test_area_contains() {
        let area = Area {
            p1: Position { x: -5, y: 10 },
            p2: Position { x: 2, y: 0 },
        };
        assert!(&area.contains(&Position { x: -1, y: 2 }));
        assert!(&area.contains(&Position { x: -5, y: 10 }));
        assert!(&area.contains(&Position { x: 2, y: 0 }));
        assert!(!&area.contains(&Position { x: -6, y: 5 }));
        assert!(!&area.contains(&Position { x: 0, y: -1 }));
    }
}
