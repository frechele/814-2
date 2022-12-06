use rand::{thread_rng, Rng};

pub const BOARD_WIDTH: usize = 14;
pub const BOARD_HEIGHT: usize = 8;

pub type Board = [[i32; BOARD_WIDTH]; BOARD_HEIGHT];
type BoardMarker = [[bool; BOARD_WIDTH]; BOARD_HEIGHT];

pub fn new_board() -> Board {
    let mut rng = thread_rng();
    let mut board = [[0; BOARD_WIDTH]; BOARD_HEIGHT];

    for row in 0..BOARD_HEIGHT {
        for col in 0..BOARD_WIDTH {
            board[row][col] = rng.gen_range(0..10);
        }
    }

    board
}

pub fn is_valid(board: &Board) -> bool {
    for row in board.iter() {
        for num in row.iter() {
            if *num < 0 || *num > 9 {
                return false;
            }
        }
    }
    true
}

pub fn set_board(board: &mut Board, x: usize, y: usize, val: i32) {
    board[y][x] = val;
}

pub fn print_board(board: &Board) {
    for row in board.iter() {
        for num in row.iter() {
            print!("{}", num);
        }
        println!();
    }
}
