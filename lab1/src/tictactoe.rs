use std::io;

fn main() {
    let mut board = [[' '; 3]; 3];

    print_board(&board);
    game_loop(&mut board);
}

fn game_loop(board: &mut [[char; 3]; 3]) {
    let mut player_turn = 1;

    while !is_tie(*board) {
        let mut index: usize;
        loop {
            println!("Player {}, enter your move (1-9):", player_turn);
            index = read_player_input() as usize - '0' as usize;

            if (1..=9).contains(&index) && is_empty(board, index - 1) {
                break;
            }
            println!("Invalid input! Please try again.");
        }

        if player_turn == 1 {
            set_at_board(board, index - 1, 'O');
        } else {
            set_at_board(board, index - 1, 'X');
        }

        let (is_end, winning_player) = is_winning(*board);
        if is_end {
            println!(
                "Player {} wins the game!",
                if winning_player == 'O' { 1 } else { 2 }
            );
            print_board(board);
            break;
        }

        player_turn = player_turn % 2 + 1;
        print_board(board);
    }
}

fn is_tie(board: [[char; 3]; 3]) -> bool {
    for row in &board {
        for col in *row {
            if col == ' ' {
                return false;
            }
        }
    }

    true
}

fn is_winning(board: [[char; 3]; 3]) -> (bool, char) {
    let winning_conditions = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
        [1, 4, 7],
        [2, 5, 8],
        [3, 6, 9],
        [1, 5, 9],
        [3, 5, 7],
    ];

    for win_con in winning_conditions {
        let player: char = board[(win_con[0] - 1) / 3][(win_con[0] - 1) % 3];

        if player == ' ' {
            continue;
        }

        let mut is_win = true;
        for &index in &win_con[1..] {
            let i = (index - 1) / 3;
            let j = (index - 1) % 3;

            if player != board[i][j] {
                is_win = false;
                break;
            }
        }
        if is_win {
            return (true, player);
        }
    }

    (false, ' ')
}

fn read_player_input() -> char {
    let mut user_input = String::new();

    println!("Type your command:");
    let _ = io::stdin().read_line(&mut user_input); // get string from the user input
    user_input.chars().next().unwrap() // get the first char from the given string
}

fn print_board(board: &[[char; 3]; 3]) {
    for (i, row) in board.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == ' ' {
                print!(" {}", 3 * i + j + 1)
            } else {
                print!(" {}", cell);
            }

            if j < 2 {
                print!(" |");
            }
        }

        println!();

        if i < 2 {
            println!("------------");
        }
    }
}

fn is_empty(board: &[[char; 3]; 3], index: usize) -> bool {
    board[index / 3][index % 3] == ' '
}

fn set_at_board(board: &mut [[char; 3]; 3], index: usize, c: char) {
    board[index / 3][index % 3] = c;
}
