mod board;

use crate::board::{new_board, print_board};

fn main() {
    let bd = new_board();

    print_board(&bd);
}
