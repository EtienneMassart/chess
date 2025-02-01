use crate::core_struct::{Board, Color, Piece};
use crate::game::GameState;

impl Board {
    fn can_pawn_take_king(
        &self,
        start: (usize, usize),
        end: (usize, usize),
        color: Color,
    ) -> bool {
        // We only have to check normal take, a pawn can't en passant the king
        if color == Color::White {
            return start.0 + 1 == end.0 && (start.1 == end.1 + 1 || start.1 + 1 == end.1);
        } else {
            return start.0 == end.0 + 1 && (start.1 == end.1 + 1 || start.1 + 1 == end.1);
        }
    }

    fn can_king_take_king(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        // Check if the move is only one square away, we don't need to check for castling because you can't take by castling
        let x_diff = (start.0 as i8 - end.0 as i8).abs();
        let y_diff = (start.1 as i8 - end.1 as i8).abs();
        return x_diff <= 1 && y_diff <= 1;
    }

    pub(super) fn is_king_in_check(&self, color: Color) -> Result<bool, &'static str> {
        let king_positions = self
            .pieces
            .get(&(Piece::King(color)))
            .ok_or("King not found")?;
        if king_positions.len() != 1 {
            return Err("Multiple or no kings found");
        }
        let king_pos = *king_positions.iter().next().unwrap(); // We know there's only one king so this is safe

        for (piece, positions) in &self.pieces {
            for pos in positions {
                if piece.color() != color {
                    match piece {
                        Piece::Pawn(_) => {
                            if self.can_pawn_take_king(*pos, king_pos, piece.color()) {
                                return Ok(true);
                            }
                        }

                        Piece::Knight(_) => {
                            if self.is_valid_knight_move(*pos, king_pos) {
                                return Ok(true);
                            }
                        }

                        Piece::Bishop(_) => {
                            if self.is_valid_bishop_move(*pos, king_pos) {
                                return Ok(true);
                            }
                        }

                        Piece::Rook(_) => {
                            if self.is_valid_rook_move(*pos, king_pos) {
                                return Ok(true);
                            }
                        }

                        Piece::Queen(_) => {
                            if self.is_valid_queen_move(*pos, king_pos) {
                                return Ok(true);
                            }
                        }

                        Piece::King(_) => {
                            if self.can_king_take_king(*pos, king_pos) {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }

        return Ok(false);
    }

    /// See if the color king is in checkmate or
    pub(crate) fn evaluate_endgame(&mut self, game_state: &GameState) -> EndgameStatus {
        let color = game_state.turn;
        if !self.has_legal_moves(game_state) {
            if self.is_king_in_check(color).unwrap() {
                return EndgameStatus::Checkmate(color); // Checkmate
            }
            return EndgameStatus::Stalemate; // Stalemate
        }
        EndgameStatus::Ongoing // The game can continue
    }
}

/// The status of the endgame. The color in the checkmate variant is the color that is checkmated and lost.
#[derive(Debug, PartialEq)]
pub enum EndgameStatus {
    Checkmate(Color), // The color that is checkmated
    Stalemate,
    Ongoing,
}

impl EndgameStatus {
    pub fn is_ongoing(&self) -> bool {
        match self {
            EndgameStatus::Ongoing => true,
            _ => false,
        }
    }
}

#[cfg(test)]
#[path = "../tests/test_check_and_mate.rs"]
mod test_check_and_mate;