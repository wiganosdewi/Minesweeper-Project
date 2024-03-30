use std::io;
use std::process;
//use std::collections::HashMap;
use rand::Rng;

const WIDTH: usize = 8;
const HEIGHT: usize = 8;
const MINES: usize = 10;

const ALPHABET: [&str; 26] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K",
    "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];

const MINEINDEX: usize = 0;
const DISCOVERINDEX: usize = 1;
const FLAGINDEX: usize = 2;


fn main() {
    // Make the gameplay loop
    let mut board = generate_board();
    loop {
        let mut player_choice = String::new();

        render_board(&board);
        println!("What do you want to do?");
        println!("[c]lear  [f]lag  [q]uit");        

        io::stdin()
            .read_line(&mut player_choice)
            .expect("Failed to read line");

        let player_choice = player_choice.trim().to_string();

        match player_choice.as_str() {
            "c" => {
                println!("which cell do you want to clear?");

                let (number, letter) = capture_user_input();

                board[number - 1][letter][DISCOVERINDEX] = true;
                read_board(&board, letter, number - 1)
            }

            "f" => {
                println!("which cell do you want to flag?");

                let (number, letter) = capture_user_input();

                board[number - 1][letter][FLAGINDEX] = true;
            }

            "q" => process::exit(0),
            _ => continue,
        }
    }
}


fn render_board(board: &[[[bool; 3]; WIDTH]; HEIGHT]) {
    let mut buffer = vec![String::new(); WIDTH];
    print!("  ");

    for letter in 0..WIDTH {
        print!("  {:?}", ALPHABET[letter]);
    }

    println!("");

    for row in 0..HEIGHT {
        print!(" {} ", row + 1);

        for collumn in 0..WIDTH {
            match board[row][collumn][FLAGINDEX] {
                true => buffer[collumn] = String::from("F"),

                false => match board[row][collumn][DISCOVERINDEX] {
                    false => buffer[collumn] = String::from("#"),

                    true => match board[row][collumn][MINEINDEX] {
                        false => buffer[collumn] = near_mines(board, row, collumn),
                        true => buffer[collumn] = String::from("X"),
                    }
                }
            }
        }
        println!("{:?}", &buffer);
    }
}


fn near_mines(board: &[[[bool; 3]; WIDTH]; HEIGHT], row: usize, collumn: usize) -> String {
    let mut near_mines: u32 = 0;

    for v_offset in -1..2 {
        for h_offset in -1..2 {
            let new_row = row as isize + v_offset;
            let new_col = collumn as isize + h_offset;

            if new_row >= 0 && new_row < HEIGHT as isize && 
            new_col >= 0 && new_col < WIDTH as isize {
                if board[new_row as usize][new_col as usize][MINEINDEX] == true {
                    near_mines += 1;
                }
            }
        }
    }
    let near_mines = near_mines.to_string();
    return near_mines;
}


fn generate_board() -> [[[bool; 3]; WIDTH]; HEIGHT] {
    let mut cells: u32 = WIDTH as u32 * HEIGHT as u32;
    let mut mines: u32 = MINES as u32;

    let mut array = [[[false; 3]; WIDTH]; HEIGHT];

    for row in 0..HEIGHT {
        for collumn in 0..WIDTH {
            if mines >= rand::thread_rng().gen_range(1..=cells) {
                array[row][collumn][MINEINDEX] = true;
                mines -= 1;
            }
            cells -= 1;
        }
    }
    return array;
}


fn clear_around(board: &mut [[[bool; 3]; WIDTH]; HEIGHT], row: usize, collumn: usize) {
    for v_offset in -1..2 {
        for h_offset in -1..2 {
            let new_row = row as isize + v_offset;
            let new_col = collumn as isize + h_offset;

            if new_row >= 0 && new_row < HEIGHT as isize && 
            new_col >= 0 && new_col < WIDTH as isize {
                if board[new_row as usize][new_col as usize][FLAGINDEX] == false {
                    board[new_row as usize][new_col as usize][DISCOVERINDEX] = true;
                }
            }
        }
    }
}


fn capture_user_input() -> (usize, usize) {
    let mut input = String::from("");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let split_index = input.chars().position(|c| c.is_digit(10)).unwrap_or(input.len());

    let (letter, number) = input.split_at(split_index);

    let number: usize = number.trim()
        .parse()
        .expect("Failed to parse coordinate number");

    let letter = ALPHABET.iter()
        .position(|&i| i == letter)
        .unwrap();

    return (number, letter);
}


fn read_board(board: &[[[bool; 3]; WIDTH]; HEIGHT], row: usize, collumn: usize) {
    
}
