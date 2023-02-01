use std::collections::HashMap;
use std::convert::Infallible;
use std::ops::Sub;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Signal(Vec<char>);

#[derive(Debug, PartialEq, Eq)]
pub struct Digit(Vec<char>);

#[derive(Debug, PartialEq)]
pub struct Entry(Vec<Signal>, Vec<Digit>);

impl Signal {
    fn new(s: &str) -> Signal {
        let mut signal = Signal(Vec::from_iter(s.chars()));
        signal.0.sort();
        signal
    }
    fn as_string(&self) -> String {
        String::from_iter(self.0.iter())
    }
    fn includes(&self, rhs: &[char]) -> bool {
        rhs.iter().all(|v| self.0.contains(v))
    }
    fn intersect(&self, rhs: &[char]) -> Vec<char> {
        self.0.iter().filter(|v| rhs.contains(v)).map(|c| *c).collect()
    }
}

impl Sub for Signal {
    type Output = Vec<char>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec::from_iter(self.0.iter()
            .filter(|v| !rhs.0.contains(v))
            .map(|v| *v)
        )
    }
}

impl Digit {
    fn new(s: &str) -> Digit {
        let mut digit = Digit(Vec::from_iter(s.chars()));
        digit.0.sort();
        digit
    }
    fn as_string(&self) -> String {
        String::from_iter(self.0.iter())
    }
}

impl FromStr for Entry {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .trim()
            .split("|")
            .map(|p| String::from(p))
            .collect::<Vec<String>>();
        let signals = parts[0]
            .split_whitespace()
            .map(|p| Signal::new(p))
            .collect();
        let digits = parts[1]
            .split_whitespace()
            .map(|p| Digit::new(p))
            .collect();
        Ok(Entry(signals, digits))
    }
}

fn parse_input(input: &str) -> Vec<Entry> {
    input
        .trim()
        .split("\n")
        .map(|v| v.parse::<Entry>().unwrap())
        .collect()
}

mod part_1 {
    use super::*;

    pub fn count_digits_1_4_7_and_8(entries: &[Entry]) -> u16 {
        let mut count = 0;
        for entry in entries {
            let digits = &entry.1;
            for digit in digits {
                if [2, 3, 4, 7].contains(&digit.0.len()) {
                    count += 1;
                }
            }
        }
        count
    }
}

mod part_2 {
    use super::*;

    type SegmentsMap = HashMap<String, u8>;

    pub fn deduce_segment_map(signals: &[Signal]) -> SegmentsMap {
        let mut segments_map = SegmentsMap::new();

        // 1
        let signal_one = signals.iter().find(|s| s.0.len() == 2).unwrap();
        segments_map.insert(signal_one.as_string(), 1);

        // 7
        let signal_seven = signals.iter().find(|s| s.0.len() == 3).unwrap();
        segments_map.insert(signal_seven.as_string(), 7);

        // 4
        let signal_four = signals.iter().find(|s| s.0.len() == 4).unwrap();
        segments_map.insert(signal_four.as_string(), 4);

        // 8
        let signal_eight = signals.iter().find(|s| s.0.len() == 7).unwrap();
        segments_map.insert(signal_eight.as_string(), 8);

        // length 6: 6, 9 and 0
        let len_6: Vec<&Signal> = signals.iter()
            .filter(|s| s.0.len() == 6)
            .collect();

        for s in len_6 {
            let diff = signal_eight.clone() - s.clone();
            if signal_one.includes(&diff) {
                segments_map.insert(s.as_string(), 6);
            } else if signal_four.includes(&diff) {
                segments_map.insert(s.as_string(), 0);
            } else {
                segments_map.insert(s.as_string(), 9);
            }
        }

        // length 5: 2, 3 and 5
        let len_5: Vec<&Signal> = signals.iter()
            .filter(|s| s.0.len() == 5)
            .collect();

        for s in len_5 {
            let diff = signal_eight.clone() - s.clone();
            if signal_four.includes(&diff) {
                segments_map.insert(s.as_string(), 2);
            } else if signal_one.intersect(&diff).is_empty() {
                segments_map.insert(s.as_string(), 3);
            } else {
                segments_map.insert(s.as_string(), 5);
            }
        }

        segments_map
    }

    pub fn get_decimal_value(digits: &[Digit], segments_map: &SegmentsMap) -> u32 {
        let mut value = 0u32;
        for d in digits {
            value += *segments_map.get(&d.as_string()).unwrap() as u32;
            value *= 10;
        }
        value / 10
    }

    pub fn find_digit_values_and_add_them(entries: &[Entry]) -> u32 {
        let mut total_sum = 0u32;
        for entry in entries {
            let signals = &entry.0;
            let digits = &entry.1;
            let segments_map = deduce_segment_map(&signals);
            total_sum += get_decimal_value(&digits, &segments_map);
        }
        total_sum
    }
}

fn main() {
    let entries = parse_input(&std::fs::read_to_string("data/day-08.txt").unwrap());

    println!("== PART 1");
    let count = part_1::count_digits_1_4_7_and_8(&entries);
    println!("Count of digits 1, 4, 7 and 8: {count}");

    println!("== PART 2");
    let total_sum = part_2::find_digit_values_and_add_them(&entries);
    println!("Total sum of digits in all entries: {total_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal_creation_from_string() {
        let s1 = Signal::new("abc");
        assert_eq!(s1.0.len(), 3);
    }

    #[test]
    fn signal_creation_from_string_sorts_components() {
        let s1 = Signal::new("caeb");
        assert_eq!(s1.0.len(), 4);
        assert_eq!(s1.0, ['a', 'b', 'c', 'e']);
    }

    #[test]
    fn signal_components_as_string() {
        let s1 = Signal::new("aebcf");
        assert_eq!(s1.0.len(), 5);
        assert_eq!(s1.as_string(), "abcef");
    }

    #[test]
    fn signal_subtraction() {
        let s1 = Signal::new("abcde");
        let s2 = Signal::new("dfbc");
        let res = s1 - s2;
        assert_eq!(res, ['a', 'e']);
    }

    #[test]
    fn signal_includes_subset() {
        let s1 = Signal::new("abcdef");
        assert!(s1.includes(&['a']));
        assert!(s1.includes(&['a', 'd']));
        assert!(s1.includes(&['a', 'd', 'f']));
        assert!(!s1.includes(&['a', 'x']));
    }

    #[test]
    fn signal_intersect() {
        let s1 = Signal::new("abcdef");
        assert_eq!(s1.intersect(&['b', 'x', 'd']), ['b', 'd']);
        assert_eq!(s1.intersect(&['x', 'y']), []);
    }

    #[test]
    fn parsing_of_entries_with_signals_and_digits() {
        let input = "   abc  def  foobar |  xyzh nmo  qwerty    
   asdfg  nb  qqq | zxcv lkopj eaw
";
        let entries = parse_input(input);
        assert_eq!(entries.len(), 2);
        let signals_1 = &entries[0].0;
        assert_eq!(
            signals_1,
            &vec![
                Signal::new("abc"),
                Signal::new("def"),
                Signal::new("foobar")
            ]
        );
        let digits_2 = &entries[1].1;
        assert_eq!(
            digits_2,
            &vec![
                Digit::new("zxcv"),
                Digit::new("lkopj"),
                Digit::new("eaw")
            ]
        );
    }

    mod part_1 {
        use super::super::*;

        #[test]
        fn digits_1_4_7_and_8_can_be_deduced_by_n_of_segments() {
            let entries = [
                Entry(
                    vec![],
                    vec![
                        Digit::new("ab"),
                        Digit::new("abc"),
                        Digit::new("x"),
                        Digit::new("abcdefg"),
                    ],
                ),
                Entry(
                    vec![],
                    vec![
                        Digit::new("w"),
                        Digit::new("loop"),
                        Digit::new("antiq"),
                        Digit::new("xy"),
                    ],
                ),
            ];
            let count = part_1::count_digits_1_4_7_and_8(&entries);
            assert_eq!(count, 5);
        }
    }

    mod part_2 {
        use super::super::*;

        #[test]
        fn get_decimal_value_of_set_of_digits() {
            let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
            let entry = &parse_input(input)[0];
            let signals = &entry.0;
            let digits = &entry.1;
            let segments_map = part_2::deduce_segment_map(&signals);

            assert_eq!(part_2::get_decimal_value(&digits, &segments_map), 5353);
        }

    }
}
