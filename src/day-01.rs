fn count_window_increases(readings: &[u32], window_size: usize) -> u32 {
    let windows = readings.windows(window_size);
    let mut increases: u32 = 0;
    windows.fold(u32::MAX, |prev, curr| {
        let curr_sum = curr.iter().sum();
        if curr_sum > prev {
            increases += 1;
        }
        curr_sum
    });
    increases
}

fn count_increases(readings: &[u32]) -> u32 {
    return count_window_increases(readings, 1);
}

fn main() -> Result<(), std::io::Error> {
    let readings: Vec<u32> = aoc_2021::parse_input("data/day-01.txt")?;
    println!("Total readings: {}", readings.len());
    println!("== PART 1");
    part_1(&readings);
    println!("== PART 2");
    part_2(&readings);
    Ok(())
}

fn part_1(readings: &Vec<u32>) {
    println!("Depth increases: {}", count_increases(&readings));
}

fn part_2(readings: &Vec<u32>) {
    println!(
        "Depth increases (3-window): {}",
        count_window_increases(&readings, 3)
    );
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use super::super::*;

        #[test]
        fn it_should_return_0_when_there_are_only_decreases() {
            let input = [120, 115, 109, 100];
            assert_eq!(count_increases(&input), 0);
        }

        #[test]
        fn it_should_return_1_when_there_is_1_increase() {
            let input = [120, 115, 116, 100];
            assert_eq!(count_increases(&input), 1);
        }

        #[test]
        fn it_should_return_2_when_there_are_2_increases() {
            let input = [120, 115, 116, 100, 111];
            assert_eq!(count_increases(&input), 2);
        }
    }

    mod part_2 {
        use super::super::*;

        #[test]
        fn it_should_return_0_when_there_are_only_decreases() {
            let input = [120, 115, 109, 110, 82, 76];
            assert_eq!(count_window_increases(&input, 3), 0);
        }

        #[test]
        fn it_should_return_1_when_there_is_1_increase() {
            let input = [120, 115, 109, 110, 82, 76, 95, 180];
            assert_eq!(count_window_increases(&input, 3), 1);
        }

        #[test]
        fn it_should_return_2_when_there_are_2_increases() {
            let input = [50, 60, 70, 60, 50, 90];
            assert_eq!(count_window_increases(&input, 3), 2);
        }
    }
}
