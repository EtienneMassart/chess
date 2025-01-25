use crate::chess_logic::{self, Color};


#[derive(Debug)]
pub struct GameState {
    pub turn: chess_logic::Color,
    pub white_castle_king_side: bool,
    pub black_castle_king_side: bool,
    pub white_castle_queen_side: bool,
    pub black_castle_queen_side: bool,
    pub en_passant: Option<(usize, Color)>,
}

#[derive(Debug)]
pub struct Game {
    pub board: chess_logic::Board,
    pub game_state: GameState,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: chess_logic::Board::new(),
            game_state : GameState {
                turn: chess_logic::Color::White,
                white_castle_king_side: true,
                white_castle_queen_side: true,
                black_castle_king_side: true,
                black_castle_queen_side: true,
                en_passant: None,
            },
        }
    }
    

    pub fn make_move(&mut self, start: (usize, usize), end: (usize, usize)) -> Result<(), &'static str> {
        self.board.make_move(start, end, &mut self.game_state)?;
        Ok(())
    }
}