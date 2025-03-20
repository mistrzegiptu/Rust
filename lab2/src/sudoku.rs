fn main() {
    let board = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    println!("{}", check_sudoku_board(board));
}

fn check_sudoku_board(board: [[u8; 9]; 9]) -> bool {
    let magic_number: u32 = 15; // 1 ^ 2 ^ 3 ^ 4 ^ 5 ^ 6 ^ 7 ^ 8 ^ 9

    check_number_correctness(board)
        && check_rows(board, magic_number)
        && check_cols(board, magic_number)
        && check_diagonals(board, magic_number)
        && check_small_squares(board, magic_number)
}

fn check_number_correctness(board: [[u8; 9]; 9]) -> bool {
    for (i, row) in board.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if board[i][j] > 9 {
                return false;
            }
        }
    }

    true
}

fn check_rows(board: [[u8; 9]; 9], magic_num: u32) -> bool {
    for (i, row) in board.iter().enumerate() {
        let mut row_value: u32 = 0;
        for (j, _) in row.iter().enumerate() {
            row_value ^= board[i][j] as u32;
        }

        if magic_num != row_value {
            return false;
        }
    }

    true
}

fn check_cols(board: [[u8; 9]; 9], magic_num: u32) -> bool {
    for (i, row) in board.iter().enumerate() {
        let mut col_value: u32 = 0;
        for (j, _) in row.iter().enumerate() {
            col_value ^= board[j][i] as u32;
        }

        if magic_num != col_value {
            return false;
        }
    }

    true
}

fn check_diagonals(board: [[u8; 9]; 9], magic_num: u32) -> bool {
    let mut left_diagonal_value: u32 = 0;
    let mut right_diagonal_value: u32 = 0;

    for (i, _) in board.iter().enumerate() {
        left_diagonal_value ^= board[i][i] as u32;
        right_diagonal_value ^= board[i][9 - i] as u32;
    }

    left_diagonal_value == magic_num && right_diagonal_value == magic_num
}

fn check_small_squares(board: [[u8; 9]; 9], magic_num: u32) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            let mut square_value: u32 = 0;
            for x in 0..3 {
                for y in 0..3 {
                    square_value ^= board[i * 3 + x][j * 3 + y] as u32;
                }
            }
            if square_value != magic_num {
                return false;
            }
        }
    }

    true
}