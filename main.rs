use rand::Rng;

const WIDTH: usize = 8;
const HEIGHT: usize = 8;
const MINES: usize = 10;

const MINEINDEX: usize = 0;
const DISCOVERINDEX: usize = 1;
const FLAGINDEX: usize = 2;

fn main() {
    let mut board = generate_board();
    board[0][0][DISCOVERINDEX] = true;
    render_board(&board);
}


fn render_board(board: &[[[bool; 3]; WIDTH]; HEIGHT]) {
    let mut buffer = vec!["".to_string(); WIDTH];

    for row in 0..HEIGHT {
        for collumn in 0..WIDTH {
            match board[row][collumn][FLAGINDEX] {
                true => buffer[collumn] = "F".to_string(),

                false => match board[row][collumn][DISCOVERINDEX] {
                    false => buffer[collumn] = "#".to_string(),

                    true => match board[row][collumn][MINEINDEX] {
                        false => buffer[collumn] = near_mines(board, row, collumn),
                        true => buffer[collumn] = "X".to_string(),
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
