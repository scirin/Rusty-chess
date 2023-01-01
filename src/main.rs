use std::io;

fn main() {
    let mut board = [
        ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
        ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
        ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
    ];
    let mut turn = 'w';

    loop {
        // Print the board
        print_board(&board);

        // Get the move from the user
        println!("Enter move for {} (e.g. a2 a4):", turn);
        let mut move_str = String::new();
        io::stdin().read_line(&mut move_str).expect("Failed to read line");
        let move_str = move_str.trim();

        // Split the move into two parts (origin and destination)
        let parts: Vec<&str> = move_str.split(' ').collect();
        if parts.len() != 2 {
            println!("Invalid move format");
            continue;
        }
        let from = parts[0];
        let to = parts[1];

        // Convert the strings to coordinates
        let from_x = from.chars().nth(0).unwrap() as u8 - b'a';
        let from_y = from.chars().nth(1).unwrap() as u8 - b'1';
        let to_x = to.chars().nth(0).unwrap() as u8 - b'a';
        let to_y = to.chars().nth(1).unwrap() as u8 - b'1';

        // Make the move
        board[from_y as usize][from_x as usize] = ' ';
        board[to_y as usize][to_x as usize] = turn;

        // Switch turns
        if turn == 'w' {
            turn = 'b';
        } else {
            turn = 'w';
        }
    }
}

fn print_board(board: &[[char; 8]; 8]) {
    println!("  a b c d e f g h");
    for (i, row) in board.iter().enumerate() {
        print!("{} ", i + 1);
        for &col in row.iter() {
            print!("{} ", col);
        }
        println!();
    }
}