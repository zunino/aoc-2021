use std::collections::HashMap;
use std::fs::read_to_string;

type PolymerTemplate = Vec<char>;
type Pair = (char, char);
type InsertionRules = HashMap<Pair, char>;
type PairCounts = HashMap<Pair, usize>;
type SymbolCounts = HashMap<char, usize>;

fn parse_input(input: &str) -> (PolymerTemplate, InsertionRules) {
    let sections: Vec<&str> = input.split_terminator("\n\n").collect();
    let template: PolymerTemplate = sections[0].chars().collect();

    let rule_lines: Vec<&str> = sections[1].split_terminator("\n").collect();
    let mut rules = InsertionRules::new();
    for line in rule_lines {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let pair: Pair = (
            parts[0].chars().next().unwrap(),
            parts[0].chars().skip(1).next().unwrap(),
        );
        rules.insert(pair, parts[1].parse::<char>().unwrap());
    }

    (template, rules)
}

fn pair_counting_step(
    rules: &InsertionRules,
    pair_counts: &PairCounts,
) -> PairCounts {
    let mut new_pair_counts = PairCounts::new();

    for (pair, count) in pair_counts {
        let new_symbol: char = *(rules.get(pair).unwrap());

        let new_pair_1: Pair = (pair.0, new_symbol);
        let new_pair_2: Pair = (new_symbol, pair.1);

        *(new_pair_counts.entry(new_pair_1).or_insert(0)) += count;
        *(new_pair_counts.entry(new_pair_2).or_insert(0)) += count;
    }

    new_pair_counts
}

fn create_initial_pair_counts(
    template: &PolymerTemplate
) -> PairCounts {
    let mut pair_counts = PairCounts::new();
    for i in 0..template.len() - 1 {
        let symbols = &template[i..i + 2];
        let pair: Pair = (symbols[0], symbols[1]);
        *(pair_counts.entry(pair).or_insert(0)) += 1;
    }
    pair_counts
}

fn compute_symbol_counts(
    template: &PolymerTemplate,
    pair_counts: &PairCounts
) -> SymbolCounts {
    let mut symbol_counts = SymbolCounts::new();
    for (pair, count) in pair_counts {
        *(symbol_counts.entry(pair.0).or_insert(0)) += count;
        *(symbol_counts.entry(pair.1).or_insert(0)) += count;
    }
    let first_template_symbol = template.iter().next().unwrap();
    let last_template_symbol = template.iter().last().unwrap();
    *(symbol_counts.entry(*first_template_symbol).or_insert(0)) += 1;
    *(symbol_counts.entry(*last_template_symbol).or_insert(0)) += 1;
    symbol_counts
}

fn min_and_max_symbol_occurrences(symbol_counts: &SymbolCounts) -> (usize, usize) {
    let mut min: usize = usize::MAX;
    let mut max = usize::MIN;
    for (_, &count) in symbol_counts {
        if count < min {
            min = count;
        }
        if count > max {
            max = count;
        }
    }
    (min, max)
}

fn most_common_minus_least_common_after_n_steps(
    template: &PolymerTemplate,
    rules: &InsertionRules,
    steps: u32,
) -> usize {
    let mut pair_counts = create_initial_pair_counts(template);
    for _ in 0..steps {
        pair_counts = pair_counting_step(rules, &pair_counts);
    }
    let symbol_counts = compute_symbol_counts(template, &pair_counts);
    let (min, max) = min_and_max_symbol_occurrences(&symbol_counts);
    max/2 - min/2 // discount double counts
}

mod part_1 {
    use super::*;

    pub fn most_common_minus_least_common_after_10_steps(
        template: &PolymerTemplate,
        rules: &InsertionRules,
    ) -> usize {
        most_common_minus_least_common_after_n_steps(template, rules, 10)
    }
}

mod part_2 {
    use super::*;

    pub fn most_common_minus_least_common_after_40_steps(
        template: &PolymerTemplate,
        rules: &InsertionRules,
    ) -> usize {
        most_common_minus_least_common_after_n_steps(template, rules, 40)
    }
}

fn main() {
    let (template, rules) = parse_input(&read_to_string("data/day-14.txt").unwrap());

    println!("== PART 1");
    let diff_10 = part_1::most_common_minus_least_common_after_10_steps(&template, &rules);
    println!("Difference between most and least common element after 10 steps: {diff_10}");

    println!();

    println!("== PART 2");
    let diff_40 = part_2::most_common_minus_least_common_after_40_steps(&template, &rules);
    println!("Difference between most and least common element after 40 steps: {diff_40}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_TEMPLATE_STR: &str = "NNCB";
    const TEST_RULES: &str = "CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_input_parsing() {
        let test_input = format!("{TEST_TEMPLATE_STR}\n\n{TEST_RULES}");
        let (template, rules) = parse_input(&test_input);
        assert_eq!(template, vec!['N', 'N', 'C', 'B']);
        assert_eq!(rules.len(), 16);
        let cb_rule = rules.get(&('C', 'B'));
        assert_eq!(cb_rule, Some(&'H'));
    }

    #[test]
    fn test_create_initial_pair_counts() {
        let test_template: PolymerTemplate = TEST_TEMPLATE_STR.chars().collect();
        let pair_counts = create_initial_pair_counts(&test_template);
        
        assert_eq!(3, pair_counts.len());
        assert_eq!(Some(&1), pair_counts.get(&('N', 'N')));
        assert_eq!(Some(&1), pair_counts.get(&('N', 'C')));
        assert_eq!(Some(&1), pair_counts.get(&('C', 'B')));
    }

    #[test]
    fn test_min_and_max() {
        let symbol_counts = SymbolCounts::from([('A', 2), ('B', 5), ('C', 3), ('D', 1)]);
        let (min, max) = min_and_max_symbol_occurrences(&symbol_counts);
        assert_eq!(1, min);
        assert_eq!(5, max);
    }

    #[test]
    fn test_10_steps() {
        let test_input = format!("{TEST_TEMPLATE_STR}\n\n{TEST_RULES}");
        let (template, rules) = parse_input(&test_input);
        let diff = most_common_minus_least_common_after_n_steps(&template, &rules, 10);
        assert_eq!(1588, diff);
    }
}
