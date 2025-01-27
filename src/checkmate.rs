use crate::chess_logic::{Color, Piece, Board};

impl Board {
    pub fn check_pawn_check(&self, start: (usize, usize), end: (usize, usize), color: Color) -> bool {
        // We only have to check normal take, a pawn can't en passant the king
        if color == Color::White {
            return start.0 + 1 == end.0
                && (start.1 == end.1 + 1 || start.1 + 1 == end.1)
        } else {
            return start.0 == end.0 + 1
                && (start.1 == end.1 + 1 || start.1 + 1 == end.1)
        }      
    }

    pub fn check_king_check(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        // Check if the move is only one square away, we don't need to check for castling because you can't take by castling
        let x_diff = (start.0 as i8 - end.0 as i8).abs();
        let y_diff = (start.1 as i8 - end.1 as i8).abs();
        return x_diff <= 1 && y_diff <= 1;
    }

    pub fn  verify_check(&self, color: Color) -> Result<bool, &'static str> {
        let king_positions = self.pieces.get(&(Piece::King(color))).ok_or("King not found")?;
        if king_positions.len() != 1 {
            return Err("Multiple or no kings found");
        }
        let king_pos = *king_positions.iter().next().unwrap(); // We know there's only one king so this is safe

        for (piece, positions) in &self.pieces {
            for pos in positions {
                if piece.color() != color {
                    match piece { 
                        Piece::Pawn(_) => {
                            if self.check_pawn_check(*pos, king_pos, piece.color()) {
                                return Ok(true);
                            }
                        }

                        Piece::Knight(_) => {
                            if self.check_knight_move(*pos, king_pos) {
                                return Ok(true);
                            }
                        }

                        Piece::Bishop(_) => {
                            if self.check_bishop_move(*pos, king_pos) {
                                return Ok(true);
                            }
                        }

                        Piece::Rook(_) => {
                            if self.check_rook_move(*pos, king_pos) {
                                return Ok(true);
                            }
                        }

                        Piece::Queen(_) => {
                            if self.check_queen_move(*pos, king_pos) {
                                return Ok(true);
                            }
                        }

                        Piece::King(_) => {
                            if self.check_king_check(*pos, king_pos) {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }

        return Ok(false)
    }
}