pub mod check_move;
pub mod checkmate;
pub mod chess_logic;
pub mod game;
pub mod utils;

#[cfg(test)]
mod tests {
    mod test_check_move; // Include the test module
    mod test_utils;
    mod test_checkmate;
}

//use utils::parse_move;
use chess_logic::{Color, Piece};
use macroquad::prelude::*;

const BOARD_SIZE: f32 = 784.0; // Full board size including borders
const BORDER_SIZE: f32 = 8.0; // Size of the border on each side
const PLAYABLE_SIZE: f32 = BOARD_SIZE - BORDER_SIZE * 2.0; // Playable area size
const TILE_SIZE: f32 = PLAYABLE_SIZE / 8.0; // Size of each square (96 pixels)

#[macroquad::main("Chess Game")]
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
                if let Err(e) = game.make_move(start, end) {
                    println!("{}", e);
                }

                previous_selected = None;
                selected = None;
            }
        }

        next_frame().await;
    }
}
