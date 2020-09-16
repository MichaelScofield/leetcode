impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        assert!(board.len() == 9 && board.iter().filter(|row| row.len() != 9).count() == 0);
        let num_board = board.iter()
            .map(|row| row.iter()
                .map(|&c| c.to_digit(10).unwrap_or(0))
                .collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>();
        let mut num_board_clone = num_board.clone();
        fn next_empty_cell(board: &Vec<Vec<u32>>, (i, j): (usize, usize)) -> Option<(usize, usize)> {
            if i == 0 && j == 0 && board[i][j] == 0 {
                return Some((i, j));
            }
            let (mut i, mut j) = (i, j);
            loop {
                if i == 8 && j == 8 {
                    return None;
                }
                if j == 8 {
                    i += 1;
                    j = 0;
                } else {
                    j += 1;
                }
                if board[i][j] == 0 {
                    break;
                }
            }
            Some((i, j))
        }
        fn can_place(board: &Vec<Vec<u32>>, (i, j): (usize, usize), d: u32) -> bool {
            for x in (0..i).chain(i + 1..9) {
                if board[x][j] == d {
                    return false;
                }
            }
            for y in (0..j).chain(j + 1..9) {
                if board[i][y] == d {
                    return false;
                }
            }
            let (sub_board_start_i, sub_board_start_j) = (i / 3 * 3, j / 3 * 3);
            for x in sub_board_start_i..sub_board_start_i + 3 {
                for y in sub_board_start_j..sub_board_start_j + 3 {
                    if board[x][y] == d {
                        return false;
                    }
                }
            }
            true
        }
        fn solve_sudoku(board: &Vec<Vec<u32>>, board_clone: &mut Vec<Vec<u32>>,
                        (i, j): (usize, usize)) -> bool {
            if let Some((i, j)) = next_empty_cell(board_clone, (i, j)) {
                for d in 1..=9 {
                    if !can_place(board_clone, (i, j), d) {
                        continue;
                    }
                    board_clone[i][j] = d;
                    if solve_sudoku(board, board_clone, (i, j)) {
                        return true;
                    }
                    board_clone[i][j] = 0;
                }
                false
            } else {
                true
            }
        }
        solve_sudoku(&num_board, &mut num_board_clone, (0, 0));
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    board[i][j] = std::char::from_digit(num_board_clone[i][j], 10).unwrap();
                }
            }
        }
    }
}
