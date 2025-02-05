use std::cell::Cell;

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

use chess_core::{Color, Piece, EndgameStatus};
use chess_core::Game;
use crate::constants::{BOARD_SIZE, BORDER_SIZE, TILE_SIZE, PLAYABLE_SIZE};



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
    pub no_piece: Texture2D,
    pub is_piece: Texture2D,
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

    let no_piece = load_texture("assets/highlight_circle.png").await.map_err(|e| e.to_string())?;
    let is_piece = load_texture("assets/hollow_circle.png").await.map_err(|e| e.to_string())?;

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
        no_piece,
        is_piece,
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

pub fn show_legal_moves(game: &mut Game, selected: Option<(usize, usize)>, textures: &Textures) {
    if game.endgame_status() != EndgameStatus::Ongoing {return}

    if let Some((row, col)) = selected {
        if game.piece_at(row, col).is_some() {
            let legal_moves = game.get_legal_moves((row, col));
            for (r, c) in legal_moves {
                let x = BORDER_SIZE + c as f32 * TILE_SIZE;
                let y = BORDER_SIZE + (7 - r) as f32 * TILE_SIZE;
                if game.piece_at(r, c).is_some() {
                    draw_texture_ex(
                        textures.is_piece, 
                        x, 
                        y,
                        WHITE, 
                        DrawTextureParams {
                            dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                            ..Default::default()
                        }
                    );
                } else {
                    draw_texture_ex(
                        textures.no_piece, 
                        x, 
                        y,
                        WHITE, 
                        DrawTextureParams {
                            dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                            ..Default::default()
                        }
                    );
                }
            }
        }
    }
}

pub fn show_promotion_menu((column, color): (usize, Color), textures: &Textures) {
    let pieces = [
        Piece::Queen(color),
        Piece::Rook(color),
        Piece::Bishop(color),
        Piece::Knight(color),
    ];
    let range;
    // Draw white rectangle in squares (column, 7) to (column, 4) if color is White, else (column, 0) to (column, 3)
    // Draw the piece textures in the white rectangles

    if color == Color::White {
        range = [7, 6, 5, 4];
    }
    else {
        range = [0, 1, 2, 3];
    }
    for (row, piece) in range.iter().zip(pieces.iter()) {
        let x = BORDER_SIZE + column as f32 * TILE_SIZE;
        let y = BORDER_SIZE + (7 - *row) as f32 * TILE_SIZE;
        let texture = match piece { // Some of them are not needed but it's easier to keep them all
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
        draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, WHITE);
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

pub fn draw_game_over_box(should_quit: &Cell<bool>, game: &mut Game) {
    // Define the size of the box.
    let box_width = 300.0;
    let box_height = 190.0;
    
    // Center the box on the board.
    let box_x = (BOARD_SIZE - box_width) / 2.0;
    let box_y = (BOARD_SIZE - box_height) / 2.0;

    let winner_text = match game.endgame_status() {
        EndgameStatus::Stalemate => "It's a draw!",
        EndgameStatus::Checkmate(Color::White) => "Black wins!",
        EndgameStatus::Checkmate(Color::Black) => "White wins!",
        _ => "",
    };

    let text_size = measure_text(winner_text, None, 40, 1.0).width;
    let text_x = (box_width - text_size) / 2.0;

    let button1_size = measure_text("Restart", None, 30, 1.0).width + 40.0;
    let button1_x = (box_width - button1_size) / 2.0;

    let button2_size = measure_text("Quit", None, 30, 1.0).width + 40.0;
    let button2_x = (box_width - button2_size) / 2.0;


    // Draw a simple UI box (group) at the computed position and size.
    root_ui().window(hash!("game_over_box"), vec2(box_x, box_y), vec2(box_width, box_height), |ui| {
        // Display the winner text.
        ui.label(vec2(text_x, 15.0), winner_text);
        
        // You can add spacing here if desired, e.g., a separator:
        ui.separator();

        // Draw a "Restart" button.
        if ui.button(vec2(button1_x, 60.0), "Restart") {
            // Insert code here to restart the game.
            *game = Game::new();

        }
        
        // Draw a "Quit" button.
        if ui.button(vec2(button2_x, 120.0), "Quit") {
            // Insert code here to quit the game.
            should_quit.set(true);
        }
        
        
    });
}