use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy, Debug)]
struct BoardCell {
    number: u32,
    marked: bool,
}

type BoardCells = [[BoardCell; 5]; 5];

pub struct Board {
    cells: BoardCells,
    complete: bool,
}

#[derive(PartialEq, Debug)]
pub struct NumberLocation {
    board_idx: usize,
    row: usize,
    col: usize,
}

impl BoardCell {
    fn new(number: u32) -> BoardCell {
        BoardCell {
            number,
            marked: false,
        }
    }
}

impl Board {
    fn new(numbers: [[u32; 5]; 5]) -> Board {
        let mut board_cells = [[BoardCell::new(0); 5]; 5];
        for r in 0..5 {
            for c in 0..5 {
                board_cells[r][c].number = numbers[r][c];
            }
        }
        Board { cells: board_cells, complete: false }
    }

    fn mark(&mut self, row: usize, col: usize) {
        self.cells[row][col].marked = true;
        if !self.complete {
            if self.is_bingo_row(row) || self.is_bingo_col(col) {
                self.complete = true;
            }
        }
    }

    fn is_complete(&self) -> bool {
        self.complete
    }

    fn is_bingo_row(&self, row: usize) -> bool {
        self.cells[row].iter().all(|c| c.marked)
    }

    fn is_bingo_col(&self, col: usize) -> bool {
        for r in 0..5 {
            if !self.cells[r][col].marked {
                return false;
            }
        }
        true
    }

    fn calculate_score(&self) -> u32 {
        let mut score = 0;
        for r in 0..5 {
            for c in 0..5 {
                let cell = &self.cells[r][c];
                if !cell.marked {
                    score += cell.number;
                }
            }
        }
        score
    }
}

impl std::convert::TryFrom<&str> for Board {
    type Error = &'static str;

    fn try_from(block: &str) -> Result<Self, Self::Error> {
        let mut values: [[u32; 5]; 5] = [[0; 5]; 5];
        let rows: Vec<&str> = block.split("\n").collect();

        for (i, r) in rows.iter().enumerate() {
            let cols: Vec<&str> = r.split_whitespace().collect();
            for (j, c) in cols.iter().enumerate() {
                values[i][j] = c.parse::<u32>().unwrap();
            }
        }
        Ok(Board::new(values))
    }
}

fn parse_numbers_drawn(data: &str) -> Vec<u32> {
    data.split("\n")
        .take(1)
        .collect::<String>()
        .split(",")
        .map(|e| e.parse::<u32>().unwrap())
        .collect()
}

fn parse_boards(data: &str) -> Vec<Board> {
    data.split("\n\n")
        .skip(1)
        .map(|block| Board::try_from(block.trim()).unwrap())
        .collect::<Vec<Board>>()
}

// goes through all boards and sets up a reverse map where numbers map to the
// locations where they appear across the boards
fn make_bingo_map(boards: &Vec<Board>) -> HashMap<u32, Vec<NumberLocation>> {
    let mut bingo_map: HashMap<u32, Vec<NumberLocation>> = HashMap::new();
    for (b, board) in boards.iter().enumerate() {
        for r in 0..5 {
            for c in 0..5 {
                let number = board.cells[r][c].number;
                if let None = bingo_map.get(&number) {
                    bingo_map.insert(number, Vec::<NumberLocation>::new());
                }
                let locations = bingo_map.get_mut(&number).unwrap();
                locations.push(NumberLocation {
                    board_idx: b,
                    row: r,
                    col: c,
                });
            }
        }
    }
    bingo_map
}

mod part_1 {
    use super::*;

    pub fn play_bingo(
        boards: &mut Vec<Board>,
        numbers_drawn: &Vec<u32>,
        bingo_map: &HashMap<u32, Vec<NumberLocation>>,
    ) -> u32 {
        for number in numbers_drawn {
            let locations = bingo_map.get(number);
            if let None = locations {
                continue;
            }
            for location in locations.unwrap() {
                let board = &mut boards[location.board_idx];
                board.mark(location.row, location.col);
                if board.is_complete() {
                    let score = board.calculate_score();
                    println!("Last number called: {}", number);
                    println!("Board score: {}", score);
                    return score * number;
                }
            }
        }
        panic!("There should've been a winning board");
    }
}

mod part_2 {
    use super::*;

    pub fn play_bingo(
        boards: &mut Vec<Board>,
        numbers_drawn: &Vec<u32>,
        bingo_map: &HashMap<u32, Vec<NumberLocation>>,
    ) -> u32 {
        let mut boards_left = boards.len();
        for number in numbers_drawn {
            let locations = bingo_map.get(number);
            if let None = locations {
                continue;
            }
            for location in locations.unwrap() {
                let board = &mut boards[location.board_idx];
                if board.is_complete() {
                    continue;
                }
                board.mark(location.row, location.col);
                if board.is_bingo_row(location.row) || board.is_bingo_col(location.col) {
                    boards_left -= 1;
                    if boards_left == 0 {
                        let score = board.calculate_score();
                        println!("Last number called: {}", number);
                        println!("Board score: {}", score);
                        return score * number;
                    }
                }
            }
        }
        panic!("There should've been a winning board");
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day-04.txt").unwrap();
    let numbers_drawn = parse_numbers_drawn(&data);
    let mut boards = parse_boards(&data);
    let bingo_map = make_bingo_map(&boards);
    println!("== PART 1");
    let score = part_1::play_bingo(&mut boards, &numbers_drawn, &bingo_map);
    println!("Winning score: {}", score);
    println!("== PART 2");
    let mut boards = parse_boards(&data);
    let score = part_2::play_bingo(&mut boards, &numbers_drawn, &bingo_map);
    println!("Last board's score: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "30,9,31,32,15,33,22,34,8,10,6,7,23

 1  2  3  4  5
 6  7  8  9 10
11 12 13 14 15
16 17 18 19 20
21 22 23 24 25

10 20 30 40 50
11 21 31 41 51
12 22 32 42 52
13 23 33 43 53
14 24 34 44 54
";

    #[test]
    fn test_parse_numbers_drawn() {
        let drawn_numbers = parse_numbers_drawn(&DATA);
        assert_eq!(drawn_numbers.len(), 13);
        assert_eq!(
            drawn_numbers,
            [30, 9, 31, 32, 15, 33, 22, 34, 8, 10, 6, 7, 23]
        );
    }

    #[test]
    fn test_parse_2_boards() {
        let boards = parse_boards(&DATA);
        assert_eq!(boards.len(), 2);
        assert_eq!(
            boards[0].cells[0][0],
            BoardCell {
                number: 1,
                marked: false
            }
        );
        assert_eq!(
            boards[1].cells[0][0],
            BoardCell {
                number: 10,
                marked: false
            }
        );
    }

    #[test]
    fn test_bingo_row_should_return_false_if_no_row_cells_are_marked() {
        let board = &parse_boards(&DATA)[0];
        assert_eq!(board.is_bingo_row(0), false);
    }

    #[test]
    fn test_bingo_row_should_return_true_if_all_row_cells_are_marked() {
        let board = &mut parse_boards(&DATA)[0];
        for c in 0..5 {
            board.mark(0, c);
        }
        assert_eq!(board.is_bingo_row(0), true);
    }

    #[test]
    fn test_bingo_col_should_return_false_if_no_col_cells_are_marked() {
        let board = &parse_boards(&DATA)[0];
        assert_eq!(board.is_bingo_col(0), false);
    }

    #[test]
    fn test_bingo_col_should_return_true_if_all_col_cells_are_marked() {
        let board = &mut parse_boards(&DATA)[0];
        for r in 0..5 {
            board.mark(r, 0);
        }
        assert_eq!(board.is_bingo_col(0), true);
    }

    #[test]
    fn test_bingo_map_contains_all_numbers_across_all_boards() {
        let boards = parse_boards(&DATA);
        let bingo_map = make_bingo_map(&boards);
        assert_eq!(bingo_map.len(), 40);
    }

    #[test]
    fn test_bingo_map_contains_value_10_with_2_locations() {
        let boards = parse_boards(&DATA);
        let bingo_map = make_bingo_map(&boards);
        let locations = bingo_map.get(&10).unwrap();
        assert_eq!(
            locations[0],
            NumberLocation {
                board_idx: 0,
                row: 1,
                col: 4
            }
        );
        assert_eq!(
            locations[1],
            NumberLocation {
                board_idx: 1,
                row: 0,
                col: 0
            }
        );
    }

    #[test]
    fn test_play_bingo_returns_winning_board_score_times_last_number_called() {
        let drawn_numbers = parse_numbers_drawn(&DATA);
        let mut boards = parse_boards(&DATA);
        let bingo_map = make_bingo_map(&boards);
        let score = part_1::play_bingo(&mut boards, &drawn_numbers, &bingo_map);
        assert_eq!(score, 21012);
    }

    #[test]
    fn test_part_2_returns_score_based_on_last_completed_board() {
        let drawn_numbers = parse_numbers_drawn(&DATA);
        let mut boards = parse_boards(&DATA);
        let bingo_map = make_bingo_map(&boards);
        let score = part_2::play_bingo(&mut boards, &drawn_numbers, &bingo_map);
        assert_eq!(score, 1736);
    }
}

// last attemp 10812
