use std::{fmt::Display, fs::read_to_string};

#[derive(Debug, PartialEq)]
enum Token {
    OpenDelimiter(char),
    CloseDelimiter(char),
}

#[derive(Clone)]
pub struct ChunkLine(String);

#[derive(Debug, PartialEq)]
enum ChunkParsingError {
    CorruptedLine {
        expected: char,
        found: char,
        stack: Vec<Token>,
    },
    _IncompleteLine {
        stack: Vec<Token>,
    },
}

static OPEN_DELIMS: [char; 4] = ['(', '[', '{', '<'];
static CLOSE_DELIMS: [char; 4] = [')', ']', '}', '>'];

impl Token {
    fn new(c: char) -> Token {
        if OPEN_DELIMS.contains(&c) {
            Token::OpenDelimiter(c)
        } else if CLOSE_DELIMS.contains(&c) {
            Token::CloseDelimiter(c)
        } else {
            panic!(
                "Invalid delimiter; expected one of () [] {{}} <>; got {}",
                c
            );
        }
    }

    fn char(&self) -> char {
        match *self {
            Token::OpenDelimiter(c) => c,
            Token::CloseDelimiter(c) => c,
        }
    }
}

impl ChunkLine {
    pub fn new(s: &str) -> ChunkLine {
        ChunkLine(s.to_string())
    }
}

impl Display for ChunkParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::CorruptedLine {
                expected, found, ..
            } => {
                write!(f, "Expected {}; found {}", expected, found)
            }
            &Self::_IncompleteLine { .. } => write!(f, "Incomplete chunk line"),
        }
    }
}

fn get_matching_close_delim(c: char) -> char {
    let index = OPEN_DELIMS.iter().position(|d| *d == c).unwrap();
    CLOSE_DELIMS[index]
}

fn parse_input(input: &str) -> Vec<ChunkLine> {
    input
        .split_terminator("\n")
        .map(|line| ChunkLine(line.to_string()))
        .collect()
}

fn parse_chunk_line(chunk_line: &ChunkLine) -> Result<Vec<Token>, ChunkParsingError> {
    let mut stack: Vec<Token> = Vec::new();

    for c in chunk_line.0.chars() {
        let token = Token::new(c);
        match token {
            Token::OpenDelimiter(_) => stack.push(token),
            Token::CloseDelimiter(c) => {
                let top_token = stack.last().unwrap();
                let open_delim = top_token.char();
                let matching_close_delim = get_matching_close_delim(open_delim);
                if c != matching_close_delim {
                    return Err(ChunkParsingError::CorruptedLine {
                        expected: matching_close_delim,
                        found: c,
                        stack,
                    });
                }
                stack.pop();
            }
        }
    }

    Ok(stack)
}

mod part_1 {
    use super::*;

    pub fn calculate_syntax_error_score_for_corrupted_lines(chunk_lines: &[ChunkLine]) -> u64 {
        let mut score = 0u64;

        for line in chunk_lines {
            let result = parse_chunk_line(line);
            match result {
                Err(ChunkParsingError::CorruptedLine { found, .. }) => match found {
                    ')' => score += 3,
                    ']' => score += 57,
                    '}' => score += 1197,
                    '>' => score += 25137,
                    _ => continue,
                },
                _ => continue,
            }
        }

        score
    }
}

mod part_2 {
    use super::*;

    pub fn calculate_completion_score_for_incomplete_lines(chunk_lines: &[ChunkLine]) -> u64 {
        let mut line_scores: Vec<u64> = Vec::new();

        for line in chunk_lines {
            let result = parse_chunk_line(line);
            match result {
                Ok(stack) => {
                    let missing_delims = stack.iter()
                        .rev()
                        .map(|t| get_matching_close_delim(t.char()))
                        .collect::<Vec<char>>();
                    line_scores.push(calculate_line_score(&missing_delims));
                },
                _ => continue
            }
        }

        line_scores.sort();
        line_scores[line_scores.len() / 2]
    }

    fn calculate_line_score(missing_delims: &[char]) -> u64 {
        let mut score = 0u64;

        for c in missing_delims {
            score *= 5;
            score += get_close_delim_score(*c) as u64;
        }

        score
    }

    // ')': 1 point
    // ']': 2 points
    // '}': 3 points
    // '>': 4 points
    fn get_close_delim_score(c: char) -> u8 {
        let index = CLOSE_DELIMS.iter().position(|d| *d == c).unwrap();
        (index + 1) as u8
    }
}

fn main() {
    let chunk_lines = parse_input(&read_to_string("data/day-10.txt").unwrap());

    println!("== PART 1");
    let score = part_1::calculate_syntax_error_score_for_corrupted_lines(&chunk_lines);
    println!("Syntax error score for corrupted lines: {score}");

    println!("== PART 2");
    let score = part_2::calculate_completion_score_for_incomplete_lines(&chunk_lines);
    println!("Completion score for incomplete lines: {score}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_creation_with_opening_delimiter() {
        let token = Token::new('(');
        assert_eq!(token, Token::OpenDelimiter('('));
    }

    #[test]
    fn token_creation_with_closing_delimiter() {
        let token = Token::new('>');
        assert_eq!(token, Token::CloseDelimiter('>'));
    }

    #[test]
    #[should_panic]
    fn token_creation_with_invalid_delimiter() {
        Token::new('$');
    }

    #[test]
    fn chunk_line_parsing() {
        let chunk_line = ChunkLine::new("(([<>]))");
        let _ = parse_chunk_line(&chunk_line);
    }

    #[test]
    fn chunk_line_parsing_with_error() {
        let chunk_line = ChunkLine::new("(([<]]))");
        let result = parse_chunk_line(&chunk_line);
        assert_eq!(
            result,
            Err(ChunkParsingError::CorruptedLine {
                expected: '>',
                found: ']',
                stack: vec![
                    Token::OpenDelimiter('('),
                    Token::OpenDelimiter('('),
                    Token::OpenDelimiter('['),
                    Token::OpenDelimiter('<'),
                ]
            })
        );
    }

    #[test]
    fn successful_chunk_line_parsing_should_return_empty_stack() {
        let chunk_line = ChunkLine::new("(([<>]))");
        let result = parse_chunk_line(&chunk_line);
        if let Ok(stack) = result {
            assert!(stack.is_empty());
        }
    }

    #[test]
    fn unsuccessful_chunk_line_parsing_should_return_non_empty_stack() {
        let chunk_line = ChunkLine::new("[(([<>])");
        let result = parse_chunk_line(&chunk_line);
        if let Ok(stack) = result {
            assert_eq!(
                stack,
                vec![
                    Token::OpenDelimiter('['),
                    Token::OpenDelimiter('('),
                ]
            )
        }
    }

    #[test]
    fn teststete() {
        const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
(((({<>}<{<{<>}{[]{[]{}
{<[[]]>}<{[{[{[]{()[[[]
<{([{{}}[<[[[<>{}]]]>[]]";

        let chunk_lines = parse_input(INPUT);
        let score = part_2::calculate_completion_score_for_incomplete_lines(&chunk_lines);
        assert_eq!(score, 288957);
    }

}
