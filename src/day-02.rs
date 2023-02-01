#[derive(PartialEq, Debug)]
pub enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl std::str::FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let value = parts[1].parse::<u32>().unwrap();
        match parts[0] {
            "forward" => Ok(Command::Forward(value)),
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            _ => Err(()),
        }
    }
}

mod part_1 {
    use super::Command;

    pub struct Submarine {
        pub position: u32,
        pub depth: u32,
    }

    impl Submarine {
        pub fn new() -> Submarine {
            Submarine {
                position: 0,
                depth: 0,
            }
        }

        pub fn apply_command(&mut self, command: &Command) {
            match command {
                Command::Forward(v) => self.position += v,
                Command::Down(v) => self.depth += v,
                Command::Up(v) => self.depth -= v,
            }
        }
    }

    pub fn run(commands: &Vec<Command>) {
        let mut submarine = Submarine::new();
        for command in commands.into_iter() {
            submarine.apply_command(command);
        }
        println!("Final position: {}", submarine.position);
        println!("Final depth: {}", submarine.depth);
        println!("Position x depth: {}", submarine.position * submarine.depth);
    }
}

mod part_2 {
    use super::Command;

    pub struct Submarine {
        pub position: u32,
        pub depth: u32,
        pub aim: u32,
    }

    impl Submarine {
        pub fn new() -> Submarine {
            Submarine {
                position: 0,
                depth: 0,
                aim: 0,
            }
        }

        pub fn apply_command(&mut self, command: &Command) {
            match command {
                Command::Forward(v) => {
                    self.position += v;
                    self.depth += v * self.aim
                }
                Command::Down(v) => self.aim += v,
                Command::Up(v) => self.aim -= v,
            }
        }
    }

    pub fn run(commands: &Vec<Command>) {
        let mut submarine = Submarine::new();
        for command in commands.into_iter() {
            submarine.apply_command(command);
        }
        println!("Final position: {}", submarine.position);
        println!("Final depth: {}", submarine.depth);
        println!("Position x depth: {}", submarine.position * submarine.depth);
    }
}

fn main() {
    let commands: Vec<Command> = match aoc_2021::parse_input("data/day-02.txt") {
        Ok(cmds) => cmds,
        Err(e) => panic!("Error parsing input file for day 02: {}", e),
    };
    println!("== PART 1");
    part_1::run(&commands);
    println!("== PART 2");
    part_2::run(&commands);
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use super::super::*;

        #[test]
        fn it_should_return_forward_command_with_value_10() {
            let input = "forward 10";
            let command = input.parse::<Command>().unwrap();
            assert_eq!(command, Command::Forward(10));
        }

        #[test]
        fn it_should_return_down_command_with_value_8() {
            let input = "down 8";
            let command = input.parse::<Command>().unwrap();
            assert_eq!(command, Command::Down(8));
        }

        #[test]
        fn it_should_return_up_command_with_value_9() {
            let input = "up 9";
            let command = input.parse::<Command>().unwrap();
            assert_eq!(command, Command::Up(9));
        }

        #[test]
        fn submarine_should_be_at_position_15_and_depth_12() {
            let mut submarine = part_1::Submarine::new();
            submarine.apply_command(&Command::Forward(10));
            submarine.apply_command(&Command::Down(3));
            submarine.apply_command(&Command::Up(1));
            submarine.apply_command(&Command::Forward(5));
            submarine.apply_command(&Command::Down(10));

            assert_eq!(submarine.position, 15);
            assert_eq!(submarine.depth, 12);
        }
    }
}
