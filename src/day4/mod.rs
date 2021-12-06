use std::fs;

pub fn solve() -> (usize, usize) {
    let contents = fs::read_to_string("src/day4/input.txt").expect("Something went wrong reading the file");
    (part_1(&contents), part_2(&contents))
}

fn part_1(contents: &str) -> usize {
    let first = contents.lines().next().unwrap();

    let drawn_numbers = parse_drawn_numbers(first);
    let boards = parse_game_boards(contents);

    let mut idx_of_board = 0;
    let mut lowest_bingo = 999999999;
    
    for (idx, board) in boards.iter().enumerate() {
        let bingo_score = find_bingo(board, &drawn_numbers);

        match bingo_score {
            None => {},
            Some(score) => {
                if score < lowest_bingo {
                    lowest_bingo = score;
                    idx_of_board = idx;
                }
            }
        }
    }

    calculate_board_score(&boards[idx_of_board], &drawn_numbers, lowest_bingo)
}

fn part_2(contents: &str) -> usize {
    let first = contents.lines().next().unwrap();

    let drawn_numbers = parse_drawn_numbers(first);
    let boards = parse_game_boards(contents);

    let mut idx_of_board = 0;
    let mut lowest_bingo = 0;
    
    for (idx, board) in boards.iter().enumerate() {
        let bingo_score = find_bingo(board, &drawn_numbers);

        match bingo_score {
            None => {},
            Some(score) => {
                if score > lowest_bingo {
                    lowest_bingo = score;
                    idx_of_board = idx;
                }
            }
        }
    }

    calculate_board_score(&boards[idx_of_board], &drawn_numbers, lowest_bingo)
}

fn parse_drawn_numbers(raw_input: &str) -> Vec<usize> {
    raw_input.split(',').map(|s| s.parse::<usize>().unwrap()).collect()
}

fn parse_game_boards(raw_input: &str) -> Vec<Vec<Vec<usize>>> {
    let mut boards: Vec<Vec<Vec<usize>>> = vec![vec![]];
    let mut idx = 0;

    for (i, line) in raw_input.lines().enumerate() {
        if i == 0 || i == 1 {
            continue;
        }
        if line.is_empty() {
            idx += 1;
            boards.push(vec![]);
            continue;
        }

       let v: Vec<usize> = line.split(' ').filter(|s| s != &"").map(|s| s.parse::<usize>().unwrap()).collect();
       boards[idx].push(v);
    }

    boards
}

fn find_bingo(board: &Vec<Vec<usize>>, drawn_numbers: &Vec<usize>) -> Option<usize> {
    let mut bingo_idx: Option<usize> = None;

    for row in board.iter() {
        let mut row_bingo_idx = 0;
        let mut bingo_found = true;

        for item in row.iter() {
            match drawn_numbers.iter().position(|x| x == item) {
                None => { bingo_found = false },
                Some(idx) => {
                    if idx > row_bingo_idx {
                        row_bingo_idx = idx;
                    }
                }
            }
        }

        if bingo_found {
            match bingo_idx {
                None => bingo_idx = Some(row_bingo_idx),
                Some(idx) => {
                    if row_bingo_idx < idx {
                        bingo_idx = Some(row_bingo_idx);
                    }
                }
            }
        }
    }

    for col_idx in 0..5 {
        let mut col_bingo_idx = 0;
        let mut bingo_found = true;

        for row_idx in 0..5 {
            match drawn_numbers.iter().position(|&x| x == board[row_idx][col_idx]) {
                None => { bingo_found = false },
                Some(idx) => {
                    if idx > col_bingo_idx {
                        col_bingo_idx = idx;
                    }
                }
            }
        }

        if bingo_found {
            match bingo_idx {
                None => bingo_idx = Some(col_bingo_idx),
                Some(idx) => {
                    if col_bingo_idx < idx {
                        bingo_idx = Some(col_bingo_idx);
                    }
                }
            }
        }
    }

    bingo_idx
}

fn calculate_board_score(board: &Vec<Vec<usize>>, drawn_numbers: &Vec<usize>, idx_of_bingo: usize) -> usize {
    let mut unmarked_score = 0;

    for row in board {
        for item in row {
            match drawn_numbers.iter().position(|x| x == item) {
                None => { unmarked_score += item },
                Some(idx) => {
                    if idx > idx_of_bingo {
                        unmarked_score += item;
                    }
                }
            }
        }
    }

    unmarked_score * drawn_numbers[idx_of_bingo]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_works() {
        assert_eq!(solve(), (58412, 10030));
    }

    #[test]
    fn examples_works() {
        let test_str = String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7");
        assert_eq!(part_1(&test_str), 4512);
        assert_eq!(part_2(&test_str), 1924);
    }
}
