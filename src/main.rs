use std::io;

fn main() {
    let mut board = [[' '; 8]; 8];
    let mut turn = 'w';

    loop {
        // Print the board
        for row in board.iter() {
            for &col in row.iter() {
                print!("{} ", col);
            }
            println!();
        }

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