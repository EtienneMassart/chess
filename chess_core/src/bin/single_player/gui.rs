use macroquad::prelude::*;
use chess_core::core_struct::{Color, Piece};
use chess_core::game::Game;
use crate::constants::{BORDER_SIZE, TILE_SIZE, PLAYABLE_SIZE};


pub struct Textures {
    pub board: Texture2D,
    pub white_pawn: Texture2D,
    pub white_knight: Texture2D,
    pub white_bishop: Texture2D,
    pub white_rook: Texture2D,
    pub white_queen: Texture2D,
    pub white_king: Texture2D,
    pub black_pawn: Texture2D,
    pub black_knight: Texture2D,
    pub black_bishop: Texture2D,
    pub black_rook: Texture2D,
    pub black_queen: Texture2D,
    pub black_king: Texture2D,
}

pub async fn load_textures() -> Result<Textures, String> {
    // Use .await? if you want to propagate errors, here I'm using unwrap-style for brevity
    let board = load_texture("assets/8x8-board.png").await.map_err(|e| e.to_string())?;

    let white_pawn = load_texture("assets/white-pawn.png").await.map_err(|e| e.to_string())?;
    let white_knight = load_texture("assets/white-knight.png").await.map_err(|e| e.to_string())?;
    let white_bishop = load_texture("assets/white-bishop.png").await.map_err(|e| e.to_string())?;
    let white_rook = load_texture("assets/white-rook.png").await.map_err(|e| e.to_string())?;
    let white_queen = load_texture("assets/white-queen.png").await.map_err(|e| e.to_string())?;
    let white_king = load_texture("assets/white-king.png").await.map_err(|e| e.to_string())?;

    let black_pawn = load_texture("assets/black-pawn.png").await.map_err(|e| e.to_string())?;
    let black_knight = load_texture("assets/black-knight.png").await.map_err(|e| e.to_string())?;
    let black_bishop = load_texture("assets/black-bishop.png").await.map_err(|e| e.to_string())?;
    let black_rook = load_texture("assets/black-rook.png").await.map_err(|e| e.to_string())?;
    let black_queen = load_texture("assets/black-queen.png").await.map_err(|e| e.to_string())?;
    let black_king = load_texture("assets/black-king.png").await.map_err(|e| e.to_string())?;

    Ok(Textures {
        board,
        white_pawn,
        white_knight,
        white_bishop,
        white_rook,
        white_queen,
        white_king,
        black_pawn,
        black_knight,
        black_bishop,
        black_rook,
        black_queen,
        black_king,
    })
}

pub fn draw_board(game: &Game, textures: &Textures) {
    // Draw the chessboard background
    draw_texture(textures.board, 0.0, 0.0, WHITE);

    // Iterate over the 8x8 grid and draw pieces
    for i in 0..8 {
        for j in 0..8 {
            if let Some(piece) = game.piece_at(i, j) {
                let texture = match piece {
                    Piece::Pawn(Color::White)   => textures.white_pawn,
                    Piece::Knight(Color::White) => textures.white_knight,
                    Piece::Bishop(Color::White) => textures.white_bishop,
                    Piece::Rook(Color::White)   => textures.white_rook,
                    Piece::Queen(Color::White)  => textures.white_queen,
                    Piece::King(Color::White)   => textures.white_king,
                    Piece::Pawn(Color::Black)   => textures.black_pawn,
                    Piece::Knight(Color::Black) => textures.black_knight,
                    Piece::Bishop(Color::Black) => textures.black_bishop,
                    Piece::Rook(Color::Black)   => textures.black_rook,
                    Piece::Queen(Color::Black)  => textures.black_queen,
                    Piece::King(Color::Black)   => textures.black_king,
                };

                // Calculate drawing positions
                let x = BORDER_SIZE + j as f32 * TILE_SIZE;
                let y = BORDER_SIZE + (7 - i) as f32 * TILE_SIZE;

                // Draw the piece texture
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
    }
}

/// Select a square on the board, updating the selected square and the previously selected square
pub fn select_square(previous_selected: &mut Option<(usize, usize)>, selected: &mut Option<(usize, usize)>) {
    let mouse_pos = mouse_position();
    if mouse_pos.0 >= BORDER_SIZE
        && mouse_pos.0 <= BORDER_SIZE + PLAYABLE_SIZE
        && mouse_pos.1 >= BORDER_SIZE
        && mouse_pos.1 <= BORDER_SIZE + PLAYABLE_SIZE
    {
        *previous_selected = *selected;
        let col = ((mouse_pos.0 - BORDER_SIZE) / TILE_SIZE) as usize;
        let row = 7 - ((mouse_pos.1 - BORDER_SIZE) / TILE_SIZE) as usize;
        *selected = Some((row, col));
    }
}