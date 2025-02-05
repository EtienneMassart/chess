use chess_core::{Color, PromotionPiece};
use crate::constants::{BOARD_SIZE, BORDER_SIZE, TILE_SIZE, PLAYABLE_SIZE};
use macroquad::prelude::*;

pub fn select_promotion_piece(square: (usize, usize),column: usize, color: Color) -> Option<PromotionPiece> {
    if square.1 != column {
        return None;
    }
    if color==Color::White {
        match square.0 {
            7 => Some(PromotionPiece::Queen),
            6 => Some(PromotionPiece::Rook),
            5 => Some(PromotionPiece::Bishop),
            4 => Some(PromotionPiece::Knight),
            _ => None,
        }
    } else {
        match square.0 {
            0 => Some(PromotionPiece::Queen),
            1 => Some(PromotionPiece::Rook),
            2 => Some(PromotionPiece::Bishop),
            3 => Some(PromotionPiece::Knight),
            _ => None,
        }
    }
}

/// Select a square on the board, updating the selected square and the previously selected square
pub fn select_square() -> Option<(usize, usize)> {
    let mouse_pos = mouse_position();
    if mouse_pos.0 >= BORDER_SIZE
        && mouse_pos.0 <= BORDER_SIZE + PLAYABLE_SIZE
        && mouse_pos.1 >= BORDER_SIZE
        && mouse_pos.1 <= BORDER_SIZE + PLAYABLE_SIZE
    {
        
        let col = ((mouse_pos.0 - BORDER_SIZE) / TILE_SIZE) as usize;
        let row = 7 - ((mouse_pos.1 - BORDER_SIZE) / TILE_SIZE) as usize;
        return Some((row, col));
    }
    None
}