impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut rook = (9, 9);
        'l: for i in 0..8 {
            let row = &board[i];
            for j in 0..8 {
                if row[j] == 'R' {
                    rook = (i, j);
                    break 'l;
                }
            }
        }
        let mut captures = 0;
        fn is_captured(c: char) -> Option<bool> {
            match c {
                'B' => Some(false),
                'p' => Some(true),
                _ => None
            }
        }
        for j in (0..rook.1).rev() {
            if let Some(state) = is_captured(board[rook.0][j]) {
                if state {
                    captures += 1;
                }
                break;
            }
        }
        for j in rook.1 + 1..8 {
            if let Some(state) = is_captured(board[rook.0][j]) {
                if state {
                    captures += 1;
                }
                break;
            }
        }
        for i in (0..rook.0).rev() {
            if let Some(state) = is_captured(board[i][rook.1]) {
                if state {
                    captures += 1;
                }
                break;
            }
        }
        for i in rook.0 + 1..8 {
            if let Some(state) = is_captured(board[i][rook.1]) {
                if state {
                    captures += 1;
                }
                break;
            }
        }
        captures
    }
}
