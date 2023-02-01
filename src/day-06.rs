fn parse_input(input: &str) -> Vec<u8> {
    input.split(",")
        .map(|v| v.trim().parse::<u8>().unwrap())
        .collect()
}

mod part_1 {
    pub fn simulate_lanternfish(data: &Vec<u8>, days: u8) -> u64 {
        let mut population = data.to_vec();
        population.reserve(800_000);
        for _ in 0..days {
            let mut new_fish_count = 0;
            for fish in population.iter_mut() {
                if *fish == 0 {
                    new_fish_count += 1;
                    *fish = 6;
                    continue;
                }
                *fish -= 1;
            }
            for _ in 0..new_fish_count {
                population.push(8);
            }
        }
        population.len() as u64
    }
}

mod part_2 {
    use std::collections::HashMap;

    // the implementation on part_1 didn't scale, so this cluster-based
    // approach was necessary to cope with the 256 simulation period
    pub fn simulate_lanternfish(data: &Vec<u8>, days: u16) -> u64 {
        let mut population_map = HashMap::<u8, u64>::new();

        for p in data {
            *population_map.entry(*p).or_insert(0) += 1;
        }

        let mut population: [u64; 9] = [0; 9];
        for (p, count) in population_map {
            population[p as usize] = count;
        }

        for _ in 0..days {
            let p0 = population[0];
            for i in 1..9 {
                population[i-1] = population[i];
            }
            population[6] += p0;
            population[8] = p0;
        }

        population.iter().sum()    
    }
}

fn main() {
    let data = parse_input(&std::fs::read_to_string("data/day-06.txt").unwrap());
    println!("== PART 1");
    let population = part_1::simulate_lanternfish(&data, 80);
    println!("Population: {}", population);

    println!("== PART 2");
    let population = part_2::simulate_lanternfish(&data, 256);
    println!("Population: {}", population);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_population_should_be_parsed_from_csv_string() {
        let input = "5,2,4,6, 1,0";
        let data = parse_input(&input);
        assert_eq!(data.len(), 6);
    }

    #[test]
    fn new_fish_are_added_with_value_8_when_timer_reaches_0() {
        let data = vec![3, 4, 1];
        let population = part_1::simulate_lanternfish(&data, 5);
        assert_eq!(population, 6);
    }
}
