use crate::{core_struct::{self, Color, Piece}, rules::EndgameStatus};

#[derive(Debug)]
pub enum PromotionPiece {
    Queen,
    Rook,
    Bishop,
    Knight,
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
        }
    }
}

#[derive(Debug)]
pub struct Game {
    board: core_struct::Board,
    game_state: GameState,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: core_struct::Board::new(),
            game_state : GameState {
                turn: core_struct::Color::White,
                white_castle_king_side: true,
                white_castle_queen_side: true,
                black_castle_king_side: true,
                black_castle_queen_side: true,
                en_passant: None,
                promotion_pending: None,
                endgame_status: EndgameStatus::Ongoing,
            },
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

        // update game state
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

        if self.board.grid[start.0][start.1] == Some(Piece::Pawn(Color::White)) && start.0 == 1 && end.0 == 3 {
            self.game_state.en_passant = Some((start.1, Color::White));
        } else if self.board.grid[start.0][start.1] == Some(Piece::Pawn(Color::Black)) && start.0 == 6 && end.0 == 4 {
            self.game_state.en_passant = Some((start.1, Color::Black));
        } else {
            self.game_state.en_passant = None;
        }

        if self.board.grid[start.0][start.1] == Some(Piece::Pawn(Color::White)) && end.0 == 7 {
            self.game_state.promotion_pending = Some((end.1, Color::White));
        } else if self.board.grid[start.0][start.1] == Some(Piece::Pawn(Color::Black)) && end.0 == 0 {
            self.game_state.promotion_pending = Some((end.1, Color::Black));
        }
        else {
            // Only switch turns if the move is not a promotion
            self.game_state.turn = self.game_state.turn.opposite();
        }


        self.board.execute_move(start, end);

        Ok(())
    }

    /// Promote a pawn to a piece
    pub fn promote_pawn(&mut self, promotion_piece: PromotionPiece) -> Result<(), &'static str> {
        let (y, color) = self.game_state.promotion_pending.ok_or("No promotion pending")?;
        if color != self.game_state.turn {
            return Err("Not your turn");
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
        Ok(())
    }


    /// Returns the endgame status of the game: ongoing, checkmate or stalemate
    pub fn evaluate_endgame(&mut self) -> EndgameStatus {
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
