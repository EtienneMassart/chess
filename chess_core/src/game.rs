use std::{collections::HashMap, hash::Hash};
use serde::{Serialize, Deserialize};

use crate::{core_struct::{self, Color, Piece}, rules::{EndgameStatus, WinReason, DrawReason}};

#[derive(Debug, Serialize, Deserialize)]
pub enum PromotionPiece {
    Queen,
    Rook,
    Bishop,
    Knight,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position { //For 3 fold repetition
    turn: Color,
    grid: [[Option<Piece>; 8]; 8],
    white_castle_king_side: bool,
    white_castle_queen_side: bool,
    black_castle_king_side: bool,
    black_castle_queen_side: bool,
}

#[derive(Debug)]
pub(crate) struct GameState {
    pub turn: core_struct::Color,
    pub white_castle_king_side: bool,
    pub black_castle_king_side: bool,
    pub white_castle_queen_side: bool,
    pub black_castle_queen_side: bool,
    pub en_passant: Option<(usize, Color)>,
    pub promotion_pending: Option<(usize, Color)>,
    pub endgame_status: EndgameStatus,
    pub how_many_moves: u32,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            turn: Color::White,
            white_castle_king_side: true,
            white_castle_queen_side: true,
            black_castle_king_side: true,
            black_castle_queen_side: true,
            en_passant: None,
            promotion_pending: None,
            endgame_status: EndgameStatus::Ongoing,
            how_many_moves: 0,
        }
    }
}

#[derive(Debug)]
pub struct Game {
    board: core_struct::Board,
    game_state: GameState,
    positions: HashMap<Position, u32>, //For 3 fold repetition
}

impl Game {
    pub fn new() -> Game {
        let board = core_struct::Board::new();
        let mut positions = HashMap::new();
        positions.insert(Position {
            turn: Color::White,
            grid: board.grid,
            white_castle_king_side: true,
            white_castle_queen_side: true,
            black_castle_king_side: true,
            black_castle_queen_side: true,
        }, 1);
        Game {
            board,
            game_state : GameState {
                turn: core_struct::Color::White,
                white_castle_king_side: true,
                white_castle_queen_side: true,
                black_castle_king_side: true,
                black_castle_queen_side: true,
                en_passant: None,
                promotion_pending: None,
                endgame_status: EndgameStatus::Ongoing,
                how_many_moves: 0,
            },
            positions
        }
    }

    /// Returns the piece at the given position
    pub fn piece_at(&self, i: usize, j: usize) -> Option<Piece> {
        self.board.grid[i][j]
    }
    
    /// Evaluate if a move is valid and execute it if it is
    pub fn play_move(&mut self, start: (usize, usize), end: (usize, usize)) -> Result<(), &'static str> {
        if self.game_state.promotion_pending.is_some() {
            return Err("Promotion pending");
        }
        if self.game_state.endgame_status != EndgameStatus::Ongoing {
            return Err("Game is over");
        }

        self.board.is_valid_move(start, end, &self.game_state)?;

        // Can't castle if the king or rook has moved
        if self.board.grid[start.0][start.1] == Some(Piece::King(Color::White)) {
            self.game_state.white_castle_king_side = false;
            self.game_state.white_castle_queen_side = false;
        } else if self.board.grid[start.0][start.1] == Some(Piece::Rook(Color::White)) {
            if start == (0, 0) {
                self.game_state.white_castle_queen_side = false;
            } else if start == (0, 7) {
                self.game_state.white_castle_king_side = false;
            }
        } else if self.board.grid[start.0][start.1] == Some(Piece::King(Color::Black)) {
            self.game_state.black_castle_king_side = false;
            self.game_state.black_castle_queen_side = false;
        } else if self.board.grid[start.0][start.1] == Some(Piece::Rook(Color::Black)) {
            if start == (7, 0) {
                self.game_state.black_castle_queen_side = false;
            } else if start == (7, 7) {
                self.game_state.black_castle_king_side = false;
            }
        }

        // If a pawn moves two squares, it can be captured en passant
        if self.board.grid[start.0][start.1] == Some(Piece::Pawn(Color::White)) && start.0 == 1 && end.0 == 3 {
            self.game_state.en_passant = Some((start.1, Color::White));
        } else if self.board.grid[start.0][start.1] == Some(Piece::Pawn(Color::Black)) && start.0 == 6 && end.0 == 4 {
            self.game_state.en_passant = Some((start.1, Color::Black));
        } else {
            self.game_state.en_passant = None;
        }

        // If a pawn reaches the end of the board, a promotion is pending
        if self.board.grid[start.0][start.1] == Some(Piece::Pawn(Color::White)) && end.0 == 7 {
            self.game_state.promotion_pending = Some((end.1, Color::White));
        } else if self.board.grid[start.0][start.1] == Some(Piece::Pawn(Color::Black)) && end.0 == 0 {
            self.game_state.promotion_pending = Some((end.1, Color::Black));
        }
        else {
            // Only switch turns if the move is not a promotion
            self.game_state.turn = self.game_state.turn.opposite();
        }

        let is_take_or_pawn_move = self.board.grid[end.0][end.1].is_some() || self.board.grid[start.0][start.1] == Some(Piece::Pawn(Color::White)) || self.board.grid[start.0][start.1] == Some(Piece::Pawn(Color::Black));

        self.board.execute_move(start, end);

        let position = Position {
            turn: self.game_state.turn,
            grid: self.board.grid,
            white_castle_king_side: self.game_state.white_castle_king_side,
            white_castle_queen_side: self.game_state.white_castle_queen_side,
            black_castle_king_side: self.game_state.black_castle_king_side,
            black_castle_queen_side: self.game_state.black_castle_queen_side,
        };

        // Increment the number of moves if the move is not a capture or a pawn move
        // Also add a position to the positions hashmap for 3 fold repetition
        // The only move where the taken piece is not on the end square is en passant, which is already pawn move

        if !is_take_or_pawn_move {
            self.game_state.how_many_moves += 1;
            *self.positions.entry(position).or_insert(0) += 1;
        } else {
            self.game_state.how_many_moves = 0;
            self.positions.clear();
            // Should not insert the position if a promotion is pending or if it is a 2 square pawn move
            if !self.game_state.promotion_pending.is_some() && self.game_state.en_passant.is_none() {
                self.positions.insert(position, 1);
            }   
        }

        Ok(())
    }

    /// Promote a pawn to a piece
    pub fn promote_pawn(&mut self, promotion_piece: PromotionPiece) -> Result<(), &'static str> {
        let (y, color) = self.game_state.promotion_pending.ok_or("No promotion pending")?;
        if color != self.game_state.turn {
            return Err("Wrong turn for promotion");
        } 
        let piece = match promotion_piece {
            PromotionPiece::Queen => Piece::Queen(color),
            PromotionPiece::Rook => Piece::Rook(color),
            PromotionPiece::Bishop => Piece::Bishop(color),
            PromotionPiece::Knight => Piece::Knight(color),
        };

        let x = match color {
            Color::White => 7,
            Color::Black => 0,
        };

        self.board.grid[x][y] = Some(piece);
        self.board.pieces.get_mut(&Piece::Pawn(color)).unwrap().remove(&(x, y));
        self.board.pieces.get_mut(&piece).unwrap().insert((x, y));
        self.game_state.promotion_pending = None;
        self.game_state.turn = self.game_state.turn.opposite();


        let position = Position {
            turn: self.game_state.turn,
            grid: self.board.grid,
            white_castle_king_side: self.game_state.white_castle_king_side,
            white_castle_queen_side: self.game_state.white_castle_queen_side,
            black_castle_king_side: self.game_state.black_castle_king_side,
            black_castle_queen_side: self.game_state.black_castle_queen_side,
        };

        self.positions.insert(position, 1);

        Ok(())
    }


    /// Returns the endgame status of the game: ongoing, checkmate or stalemate
    pub fn evaluate_endgame(&mut self) -> EndgameStatus {
        if self.game_state.how_many_moves >= 100 {
            self.game_state.endgame_status = EndgameStatus::Draw(DrawReason::FiftyMoveRule);
            return EndgameStatus::Draw(DrawReason::FiftyMoveRule);
        }
        if self.positions.values().any(|&v| v >= 3) {
            self.game_state.endgame_status = EndgameStatus::Draw(DrawReason::ThreefoldRepetition);
            return EndgameStatus::Draw(DrawReason::ThreefoldRepetition);
        }

        if self.board.is_insufficient_material() {
            self.game_state.endgame_status = EndgameStatus::Draw(DrawReason::InsufficientMaterial);
            return EndgameStatus::Draw(DrawReason::InsufficientMaterial);
        }

        let endgame_status = self.board.evaluate_endgame(&self.game_state);
        self.game_state.endgame_status = endgame_status;
        endgame_status
    }

    /// Panics if there is no piece at start
    pub fn get_legal_moves(&mut self, start: (usize, usize)) -> Vec<(usize, usize)> {
        self.board.get_legal_moves(start, &self.game_state)
    }

    pub fn is_promotion_pending(&self) -> Option<(usize, Color)> {
        self.game_state.promotion_pending
    }

    pub fn endgame_status(&self) -> EndgameStatus {
        self.game_state.endgame_status
    }
}
