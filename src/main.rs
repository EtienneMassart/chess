pub mod chess_logic;
pub mod game;
pub mod utils;

#[cfg(test)]
mod tests {
    mod utils_tests; // Include the test module
}

fn main() {
    let board = chess_logic::Board::new();
    board.print();
}