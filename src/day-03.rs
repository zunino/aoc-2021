fn read_values_stored_as_bit_str(path: &str) -> Result<Vec<u64>, std::io::Error> {
    Ok(std::fs::read_to_string(path)?
        .split("\n")
        .filter_map(|line| {
            if line.len() == 0 {
                return None;
            }
            return Some(aoc_2021::bit_str_to_u64(line));
        })
        .collect())
}

// Given a slice of numbers, calculates and return counts of bits for the first
// 12 bit positions across all numbers, i.e. checks all the 0-bit across all
// numbers, keeping count of 0s and 1s; then repeat for the 1-bit and so on up
// to the 11-bit. The counts for each bit index are returned as 12 2-element
// arrays in a containing array.

type Counts = [[u32; 2]; 12];

fn bit_counts_12(values: &[u64]) -> Counts {
    let mut counts: Counts = [[0, 0]; 12];

    for v in values {
        let mut w = *v;
        for b in 0..12 {
            let bit: usize = (w & 1) as usize;
            w >>= 1;
            counts[b][bit] += 1;
        }
    }

    counts
}

mod part_1 {
    use super::bit_counts_12;

    const TWELVE_BITS: u64 = 2u64.pow(12) - 1;

    pub fn run(values: &[u64]) {
        let counts = bit_counts_12(values);
        let mut gamma = 0u64;

        for b in (0..12).rev() {
            let count = &counts[b];
            if count[1] > count[0] {
                gamma |= 1;
            }
            gamma <<= 1;
        }
        gamma >>= 1;
        let epsilon: u64 = !gamma & TWELVE_BITS;

        println!("Gamma: {}", gamma);
        println!("Epsilon: {}", epsilon);
        println!("Product: {}", gamma * epsilon);
    }
}

mod part_2 {
    use super::{Counts, bit_counts_12};

    pub fn run(values: &[u64]) {
        let oxygen_rating = calculate_rating(values, relevant_bit_oxygen);
        let co2_rating = calculate_rating(values, relevant_bit_co2);

        println!("Oxygen generator rating: {}", oxygen_rating);
        println!("CO2 scrubber rating: {}", co2_rating);
        println!("Life support rating: {}", oxygen_rating * co2_rating);
    }

    pub fn remove_values_mismatching_specified_bit(
        values: &[u64],
        bit_index: usize,
        bit_value: u64,
    ) -> Vec<u64> {
        let bit_mask: u64 = 2u64.pow(bit_index as u32);
        values
            .iter()
            .filter(|v| (*v & bit_mask) >> bit_index == bit_value)
            .map(|v| *v)
            .collect()
    }

    fn relevant_bit_oxygen(counts: &Counts, bit_index: usize) -> u64 {
        let mut bit: u64 = 1;
        if counts[bit_index][0] > counts[bit_index][1] {
            bit = 0;
        }
        bit
    }

    fn relevant_bit_co2(counts: &Counts, bit_index: usize) -> u64 {
        let mut bit: u64 = 0;
        if counts[bit_index][1] < counts[bit_index][0] {
            bit = 1;
        }
        bit
    }

    fn calculate_rating(
        values: &[u64],
        relevant_bit_fn: fn(counts: &Counts, bit_index: usize) -> u64,
    ) -> u64 {
        let mut remaining_values: Vec<u64> = values.to_vec();
        let mut rating: Option<u64> = None;
        for bit_index in (0..12).rev() {
            let counts = bit_counts_12(&remaining_values);
            let bit = relevant_bit_fn(&counts, bit_index);
            if rating.is_none() {
                remaining_values =
                    remove_values_mismatching_specified_bit(&remaining_values, bit_index, bit);
                if remaining_values.len() == 1 {
                    rating = Some(remaining_values[0]);
                }
            }
        }
        rating.unwrap()
    }
}

fn main() {
    let values = read_values_stored_as_bit_str("data/day-03.txt").unwrap();
    println!("== PART 1");
    part_1::run(&values);
    println!("== PART 2");
    part_2::run(&values);
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use super::super::*;

        #[test]
        fn bit_counts_12_test_case_1() {
            let values: [u64; 3] = [0b0000, 0b1010, 0b1111];
            let expected_counts: [[u32; 2]; 12] = [
                [2, 1],
                [1, 2],
                [2, 1],
                [1, 2],
                [3, 0],
                [3, 0],
                [3, 0],
                [3, 0],
                [3, 0],
                [3, 0],
                [3, 0],
                [3, 0],
            ];
            let counts = bit_counts_12(&values);
            assert_eq!(counts, expected_counts);
        }

        #[test]
        fn bit_counts_12_test_case_2() {
            let values: [u64; 4] = [0b10110111, 0b01011100, 0b10001101, 0b00100110];
            let expected_counts: [[u32; 2]; 12] = [
                [2, 2],
                [2, 2],
                [0, 4],
                [2, 2],
                [2, 2],
                [2, 2],
                [3, 1],
                [2, 2],
                [4, 0],
                [4, 0],
                [4, 0],
                [4, 0],
            ];
            let counts = bit_counts_12(&values);
            assert_eq!(counts, expected_counts);
        }

        #[test]
        fn bit_counts_12_test_case_3() {
            let values: [u64; 4] = [
                0b101101110110,
                0b010111001100,
                0b100011011011,
                0b001001100010,
            ];
            let expected_counts: [[u32; 2]; 12] = [
                [3, 1],
                [1, 3],
                [2, 2],
                [2, 2],
                [2, 2],
                [2, 2],
                [0, 4],
                [2, 2],
                [2, 2],
                [2, 2],
                [3, 1],
                [2, 2],
            ];
            let counts = bit_counts_12(&values);
            assert_eq!(counts, expected_counts);
        }
    }

    mod part_2 {
        use super::super::*;

        #[test]
        fn test_elements_whose_bit_0_is_not_1_should_be_removed() {
            let values: &[u64] = &[0b0000, 0b1111, 0b0011];
            let clean = part_2::remove_values_mismatching_specified_bit(values, 0, 1);
            assert_eq!(clean.len(), 2);
            assert!(clean.contains(&0b1111u64));
            assert!(clean.contains(&0b1111u64));
        }

        #[test]
        fn test_elements_whose_bit_2_is_not_0_should_be_removed() {
            let values: &[u64] = &[0b0000, 0b1111, 0b0011];
            let clean = part_2::remove_values_mismatching_specified_bit(values, 2, 0);
            assert_eq!(clean.len(), 2);
            assert!(clean.contains(&0b0000u64));
            assert!(clean.contains(&0b0011u64));
        }

        #[test]
        fn test_elements_whose_bit_11_is_not_1_should_be_removed() {
            let values: &[u64] = &[
                0b101101110110,
                0b010111001100,
                0b100011011011,
                0b001001100010,
            ];
            let clean = part_2::remove_values_mismatching_specified_bit(values, 11, 1);
            assert_eq!(clean.len(), 2);
            assert!(clean.contains(&0b101101110110));
            assert!(clean.contains(&0b100011011011));
        }

        #[test]
        fn test_elements_whose_bit_11_is_not_0_should_be_removed() {
            let values: &[u64] = &[
                0b101101110110,
                0b010111001100,
                0b100011011011,
                0b001001100010,
                0b000101011101,
                0b110100001011,
                0b101110111011,
            ];
            let clean = part_2::remove_values_mismatching_specified_bit(values, 11, 0);
            assert_eq!(clean.len(), 3);
            assert!(clean.contains(&0b010111001100));
            assert!(clean.contains(&0b001001100010));
            assert!(clean.contains(&0b000101011101));
        }

        #[test]
        fn test_elements_whose_bit_0_is_not_1_should_be_removed_debugging() {
            let values: &[u64] = &[0b10110, 0b10111];
            let clean = part_2::remove_values_mismatching_specified_bit(values, 0, 1);
            assert_eq!(clean.len(), 1);
            assert!(clean.contains(&0b10111));
        }
    }
}
