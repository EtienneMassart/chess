use crate::core_struct::{Color, Piece, Board};
use crate::game::GameState;

impl Board {
    fn generate_pawn_moves(&self, start: (usize, usize), color: Color) -> Vec<(usize, usize)> {
        let direction = if color == Color::White { -1 } else { 1 };

        // Potential moves in `isize`
        let potential_moves = vec![
            (start.0 as isize + direction, start.1 as isize), // Single forward
            (start.0 as isize + 2 * direction, start.1 as isize), // Double forward
            (start.0 as isize + direction, start.1 as isize - 1), // Diagonal capture left
            (start.0 as isize + direction, start.1 as isize + 1), // Diagonal capture right
        ];

        potential_moves.into_iter()
            .filter_map(|pos| is_valid_position(pos))
            .collect()
    }

    fn generate_knight_moves(&self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let potential_moves = vec![
            (start.0 as isize - 2, start.1 as isize - 1),
            (start.0 as isize - 2, start.1 as isize + 1),
            (start.0 as isize - 1, start.1 as isize - 2),
            (start.0 as isize - 1, start.1 as isize + 2),
            (start.0 as isize + 1, start.1 as isize - 2),
            (start.0 as isize + 1, start.1 as isize + 2),
            (start.0 as isize + 2, start.1 as isize - 1),
            (start.0 as isize + 2, start.1 as isize + 1),
        ];

        potential_moves.into_iter()
            .filter_map(|pos| is_valid_position(pos))
            .collect()
    }

    fn generate_bishop_moves(&self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();

        for direction in &[(1, 1), (1, -1), (-1, 1), (-1, -1)] {
            let mut pos = (start.0 as isize + direction.0, start.1 as isize + direction.1);
            while let Some(new_pos) = is_valid_position(pos) {
                moves.push(new_pos);
                if self.grid[new_pos.0][new_pos.1].is_some() {
                    break;
                }
                pos = (pos.0 + direction.0, pos.1 + direction.1);
            }
        }

        moves
    }

    fn generate_rook_moves(&self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();

        for direction in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let mut pos = (start.0 as isize + direction.0, start.1 as isize + direction.1);
            while let Some(new_pos) = is_valid_position(pos) {
                moves.push(new_pos);
                if self.grid[new_pos.0][new_pos.1].is_some() {
                    break;
                }
                pos = (pos.0 + direction.0, pos.1 + direction.1);
            }
        }

        moves
    }

    fn generate_queen_moves(&self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();

        for direction in &[(1, 1), (1, -1), (-1, 1), (-1, -1), (1, 0), (-1, 0), (0, 1), (0, -1)] {
            let mut pos = (start.0 as isize + direction.0, start.1 as isize + direction.1);
            while let Some(new_pos) = is_valid_position(pos) {
                moves.push(new_pos);
                if self.grid[new_pos.0][new_pos.1].is_some() {
                    break;
                }
                pos = (pos.0 + direction.0, pos.1 + direction.1);
            }
        }

        moves
    }

    fn generate_king_moves(&self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let potential_moves = vec![
            (start.0 as isize - 1, start.1 as isize - 1),
            (start.0 as isize - 1, start.1 as isize),
            (start.0 as isize - 1, start.1 as isize + 1),
            (start.0 as isize, start.1 as isize - 1),
            (start.0 as isize, start.1 as isize + 1),
            (start.0 as isize + 1, start.1 as isize - 1),
            (start.0 as isize + 1, start.1 as isize),
            (start.0 as isize + 1, start.1 as isize + 1),
        ];

        potential_moves.into_iter()
            .filter_map(|pos| is_valid_position(pos))
            .collect()
    }
    /// Only if we know there is a piece at start
    pub(crate) fn get_legal_moves(&mut self, start: (usize, usize), game_state: &GameState) -> Vec<(usize, usize)> {
        let piece = self.grid[start.0][start.1].unwrap();
        let color = piece.color();

        let potential_moves = match piece {
            Piece::Pawn(_) => self.generate_pawn_moves(start, color),
            Piece::Knight(_) => self.generate_knight_moves(start),
            Piece::Bishop(_) => self.generate_bishop_moves(start),
            Piece::Rook(_) => self.generate_rook_moves(start),
            Piece::Queen(_) => self.generate_queen_moves(start),
            Piece::King(_) => self.generate_king_moves(start),
        };

        potential_moves.into_iter()
            .filter(|&end| self.is_valid_move(start, end, &game_state).is_ok())
            .collect()
    }

    pub(crate) fn has_legal_moves(&mut self, game_state: &GameState) -> bool {
        let color = game_state.turn;
        for i in 0..8 {
            for j in 0..8 {
                if self.grid[i][j].is_some() && self.grid[i][j].unwrap().color() == color {
                    let start = (i, j);
                    if !self.get_legal_moves(start, game_state).is_empty() {
                        return true;
                    }
                }
            }
        }
        false
    }

}
        


fn is_valid_position(pos: (isize, isize)) -> Option<(usize, usize)> {
    if pos.0 >= 0 && pos.0 < 8 && pos.1 >= 0 && pos.1 < 8 {
        Some((pos.0 as usize, pos.1 as usize))
    } else {
        None
    }
}