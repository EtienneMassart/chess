use crate::core_struct::{self, Color, Piece};


#[derive(Debug)]
pub struct GameState {
    pub turn: core_struct::Color,
    pub white_castle_king_side: bool,
    pub black_castle_king_side: bool,
    pub white_castle_queen_side: bool,
    pub black_castle_queen_side: bool,
    pub en_passant: Option<(usize, Color)>,
}

#[derive(Debug)]
pub struct Game {
    pub board: core_struct::Board,
    pub game_state: GameState,
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
            },
        }
    }
    

    pub fn play_move(&mut self, start: (usize, usize), end: (usize, usize)) -> Result<(), &'static str> {
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

        self.game_state.turn = match self.game_state.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };


        self.board.execute_move(start, end);

        Ok(())
    }
}