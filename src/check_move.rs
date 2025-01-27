use crate::{
    chess_logic::{Board, Color},
    game,
    utils::BiRange
};

impl Board {
    // For all these check functions, we already know that the start and end are valid and that the piece at the end is not the same color as the piece at the start
    pub fn check_pawn_move(
        &self,
        start: (usize, usize),
        end: (usize, usize),
        game_state: &game::GameState,
    ) -> bool {
        let color = game_state.turn;

        if color == Color::White {
            // check if it's a normal take
            if start.0 + 1 == end.0
                && (start.1 == end.1 + 1 || start.1 + 1 == end.1)
                && self.grid[end.0][end.1].is_some() // we already know that if there's a piece at the end it's not the same color as the pawn
            {
                return true;
            }

            // check if it's en passant
            if let Some((file, en_passant_color)) = game_state.en_passant {
                if start.0 + 1 == end.0
                    && start.0 == 4
                    && (start.1 == end.1 + 1 || start.1 + 1 == end.1)
                    && end.1 == file
                    && en_passant_color == Color::Black
                {
                    return true;
                }
            }

            // otherwise pawns can only move forward
            if end.1 != start.1 {
                return false;
            }

            // check if it's a double move
            if start.0 == 1
                && end.0 == 3
                && self.grid[2][start.1].is_none()
                && self.grid[3][start.1].is_none()
            {
                return true;
            }

            // check if it's a single move
            if start.0 + 1 == end.0 && self.grid[end.0][end.1].is_none() {
                return true;
            }

            return false;
        } else {
            // check if it's a normal take
            if start.0 == end.0 + 1
                && (start.1 == end.1 + 1 || start.1 + 1 == end.1)
                && self.grid[end.0][end.1].is_some()
            {
                return true;
            }

            // check if it's en passant
            if let Some((file, en_passant_color)) = game_state.en_passant {
                if start.0 == end.0 + 1
                    && start.0 == 3
                    && (start.1 == end.1 + 1 || start.1 + 1 == end.1)
                    && end.1 == file
                    && en_passant_color == Color::White
                {
                    return true;
                }
            }

            // otherwise pawns can only move forward
            if end.1 != start.1 {
                return false;
            }

            // check if it's a double move
            if start.0 == 6
                && end.0 == 4
                && self.grid[5][start.1].is_none()
                && self.grid[4][start.1].is_none()
            {
                return true;
            }

            // check if it's a single move
            if start.0 == end.0 + 1 && self.grid[end.0][end.1].is_none() {
                return true;
            }

            return false;
        }
    }

    pub fn check_knight_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        // For this we only need to check that the end square is in a reachable square for a knight
        let x_diff = (start.0 as i8 - end.0 as i8).abs();
        let y_diff = (start.1 as i8 - end.1 as i8).abs();
        return (x_diff == 1 && y_diff == 2) || (x_diff == 2 && y_diff == 1);
    }

    pub fn check_bishop_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        // Check if the move is diagonal
        let x_diff = (start.0 as i8 - end.0 as i8).abs();
        let y_diff = (start.1 as i8 - end.1 as i8).abs();
        if x_diff != y_diff {
            return false;
        }

        // Check if there are any pieces in the way
        let x_range = BiRange::new(start.0, end.0);
        let y_range = BiRange::new(start.1, end.1);
            

        for (x, y) in x_range.zip(y_range) {
            if self.grid[x as usize][y as usize].is_some() {
                return false;
            }
        }
        return true;
    }

    pub fn check_rook_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        // Check if the move is vertical or horizontal
        if start.0 != end.0 && start.1 != end.1 {
            return false;
        }

        // Check if there are any pieces in the way
        if start.0 == end.0 {
            let range = BiRange::new(start.1, end.1);
            for y in range {
                if self.grid[start.0][y].is_some() {
                    return false;
                }
            }
        } else {
            let range = BiRange::new(start.0, end.0);
            for x in range {
                if self.grid[x][start.1].is_some() {
                    return false;
                }
            }
        }

        return true;
         
    }

    pub fn check_queen_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        // You only need to check if the move is a valid bishop or rook move
        return self.check_bishop_move(start, end) || self.check_rook_move(start, end);
        
    }

    pub fn check_king_move(&mut self, start: (usize, usize), end: (usize, usize), game_state: &game::GameState) -> bool {
        // Check if the move is only one square away
        let x_diff = (start.0 as i8 - end.0 as i8).abs();
        let y_diff = (start.1 as i8 - end.1 as i8).abs();
        if x_diff <= 1 && y_diff <= 1 {
            return true;
        }
        
        // Check if it's a castling move
        // If it is we need to check if the king is not in check, if the squares between the king and the rook are empty and if the rook and king haven't moved yet
        // As well as if the king doesn't move through check 

        let color = game_state.turn;
        if self.verify_check(color).unwrap_or(true) {return false;}
        if color == Color::White && start == (0, 4) {
            if  end == (0, 6) && game_state.white_castle_king_side {
                if self.grid[0][5].is_none() && self.grid[0][6].is_none() {
                    let (taken_piece, promotion) = self.move_piece(start, (0,5));
                    let is_check = self.verify_check(color).unwrap_or(true);
                    self.revert_move(start, (0, 5), taken_piece, promotion);
                    return !is_check;
                }
            } else if end == (0, 2) && game_state.white_castle_queen_side {
                if self.grid[0][3].is_none() && self.grid[0][2].is_none() && self.grid[0][1].is_none() { 
                    let (taken_piece, promotion) = self.move_piece(start, (0,3));
                    let is_check = self.verify_check(color).unwrap_or(true);
                    self.revert_move(start, (0, 3), taken_piece, promotion);
                    return !is_check;
                }
            }
        }
        else if color == Color::Black && start == (7, 4) {
            if  end == (7, 6) && game_state.black_castle_king_side {
                if self.grid[7][5].is_none() && self.grid[7][6].is_none() {
                    let (taken_piece, promotion) = self.move_piece(start, (7,5));
                    let is_check = self.verify_check(color).unwrap_or(true);
                    self.revert_move(start, (7, 5), taken_piece, promotion);
                    return !is_check;
                }
            } else if end == (7, 2) && game_state.black_castle_queen_side {
                if self.grid[7][3].is_none() && self.grid[7][2].is_none() && self.grid[7][1].is_none() { 
                    let (taken_piece, promotion) = self.move_piece(start, (7,3));
                    let is_check = self.verify_check(color).unwrap_or(true);
                    self.revert_move(start, (7, 3), taken_piece, promotion);
                    return !is_check;
                }
            }
            
        }
        return false;
    }
}

