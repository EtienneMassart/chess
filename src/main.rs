pub mod check_and_mate;
pub mod core_struct;
pub mod game;
pub mod move_execution;
pub mod move_validation;
pub mod move_generation;
pub mod utils;

#[cfg(test)]
mod tests {
    mod test_check_and_mate;
    mod test_move_validation; // Include the test module
    mod test_utils;
}

//use utils::parse_move;
use core_struct::{Color, Piece};
use check_and_mate::EndgameStatus;
use macroquad::prelude::*;

const BOARD_SIZE: f32 = 784.0; // Full board size including borders
const BORDER_SIZE: f32 = 8.0; // Size of the border on each side
const PLAYABLE_SIZE: f32 = BOARD_SIZE - BORDER_SIZE * 2.0; // Playable area size
const TILE_SIZE: f32 = PLAYABLE_SIZE / 8.0; // Size of each square (96 pixels)

// Define the window configuration
fn window_conf() -> Conf {
    Conf {
        window_title: "Chess Game".to_string(),
        window_width: 784, // Match the board size including borders
        window_height: 784,
        fullscreen: false, // Disable fullscreen (optional)
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Load the chessboard texture
    let board_texture = load_texture("assets/8x8-board.png").await.unwrap();

    // Load piece textures
    let white_pawn_texture = load_texture("assets/white-pawn.png").await.unwrap();
    let white_knight_texture = load_texture("assets/white-knight.png").await.unwrap();
    let white_bishop_texture = load_texture("assets/white-bishop.png").await.unwrap();
    let white_rook_texture = load_texture("assets/white-rook.png").await.unwrap();
    let white_queen_texture = load_texture("assets/white-queen.png").await.unwrap();
    let white_king_texture = load_texture("assets/white-king.png").await.unwrap();

    let black_pawn_texture = load_texture("assets/black-pawn.png").await.unwrap();
    let black_knight_texture = load_texture("assets/black-knight.png").await.unwrap();
    let black_bishop_texture = load_texture("assets/black-bishop.png").await.unwrap();
    let black_rook_texture = load_texture("assets/black-rook.png").await.unwrap();
    let black_queen_texture = load_texture("assets/black-queen.png").await.unwrap();
    let black_king_texture = load_texture("assets/black-king.png").await.unwrap();

    let mut game = game::Game::new();

    let mut previous_selected: Option<(usize, usize)> = None;
    let mut selected: Option<(usize, usize)> = None;

    loop {
        clear_background(WHITE);

        // Draw the chessboard
        draw_texture(board_texture, 0.0, 0.0, WHITE);

        // Draw pieces

        for i in 0..8 {
            for j in 0..8 {
                let piece = game.board.grid[i][j];
                let texture = match piece {
                    Some(Piece::Pawn(Color::White)) => white_pawn_texture,
                    Some(Piece::Knight(Color::White)) => white_knight_texture,
                    Some(Piece::Bishop(Color::White)) => white_bishop_texture,
                    Some(Piece::Rook(Color::White)) => white_rook_texture,
                    Some(Piece::Queen(Color::White)) => white_queen_texture,
                    Some(Piece::King(Color::White)) => white_king_texture,
                    Some(Piece::Pawn(Color::Black)) => black_pawn_texture,
                    Some(Piece::Knight(Color::Black)) => black_knight_texture,
                    Some(Piece::Bishop(Color::Black)) => black_bishop_texture,
                    Some(Piece::Rook(Color::Black)) => black_rook_texture,
                    Some(Piece::Queen(Color::Black)) => black_queen_texture,
                    Some(Piece::King(Color::Black)) => black_king_texture,
                    None => continue,
                };
                let x = BORDER_SIZE + j as f32 * TILE_SIZE;
                let y = BORDER_SIZE + (7 - i) as f32 * TILE_SIZE;
                draw_texture_ex(
                    texture,
                    x,
                    y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                );
            }
        }

        // Handle input
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            if mouse_pos.0 >= BORDER_SIZE
                && mouse_pos.0 <= BORDER_SIZE + PLAYABLE_SIZE
                && mouse_pos.1 >= BORDER_SIZE
                && mouse_pos.1 <= BORDER_SIZE + PLAYABLE_SIZE
            {
                previous_selected = selected;
                let col = ((mouse_pos.0 - BORDER_SIZE) / TILE_SIZE) as usize;
                let row = 7 - ((mouse_pos.1 - BORDER_SIZE) / TILE_SIZE) as usize;
                selected = Some((row, col));
            }

            if let (Some((start_y, start_x)), Some((end_y, end_x))) = (previous_selected, selected)
            {
                let start = (start_y, start_x);
                let end = (end_y, end_x);
                if let Err(e) = game.play_move(start, end) {
                    println!("{}", e);
                } else {
                    previous_selected = None;
                    selected = None;

                    // Check if the game is over TODO: Add a popup and a button to restart
                    match game.board.evaluate_endgame(game.game_state.turn, &game.game_state) {
                        EndgameStatus::Ongoing => {}
                        EndgameStatus::Checkmate(Color::White) => {
                            println!("Checkmate! Black wins!");
                        }
                        EndgameStatus::Checkmate(Color::Black) => {
                            println!("Checkmate! White wins!");
                        }
                        EndgameStatus::Stalemate => {
                            println!("Stalemate!");
                        }
                    }

                }
            }

            
        }

        next_frame().await;
    }
}
