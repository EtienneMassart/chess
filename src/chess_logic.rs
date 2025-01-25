use std::{collections::{HashMap, HashSet}, fmt};
use crate::game;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = match self {
            Color::White => "white",
            Color::Black => "black",
        };
        write!(f, "{}", color)
    }
    
}

 

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}
impl Piece {
    pub fn color(&self) -> Color {
        match self {
            Piece::Pawn(color)
            | Piece::King(color)
            | Piece::Queen(color)
            | Piece::Rook(color)
            | Piece::Bishop(color)
            | Piece::Knight(color) => *color,
        }
    }
}


#[derive(Debug, Clone)]
pub struct Board {
    pub grid: [[Option<Piece>; 8]; 8],
    pub pieces: HashMap<Piece, HashSet<(usize, usize)>>
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.grid.iter().rev() {
            for cell in row.iter() {
                let symbol = match cell {
                    Some(Piece::Pawn(Color::Black)) => "♙",
                    Some(Piece::Knight(Color::Black)) => "♘",
                    Some(Piece::Bishop(Color::Black)) => "♗",
                    Some(Piece::Rook(Color::Black)) => "♖",
                    Some(Piece::Queen(Color::Black)) => "♕",
                    Some(Piece::King(Color::Black)) => "♔",
                    Some(Piece::Pawn(Color::White)) => "♟",
                    Some(Piece::Knight(Color::White)) => "♞",
                    Some(Piece::Bishop(Color::White)) => "♝",
                    Some(Piece::Rook(Color::White)) => "♜",
                    Some(Piece::Queen(Color::White)) => "♛",
                    Some(Piece::King(Color::White)) => "♚",                    
                    None => " ",
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            grid: [
                [
                    Some(Piece::Rook(Color::White)),
                    Some(Piece::Knight(Color::White)),
                    Some(Piece::Bishop(Color::White)),
                    Some(Piece::Queen(Color::White)),
                    Some(Piece::King(Color::White)),
                    Some(Piece::Bishop(Color::White)),
                    Some(Piece::Knight(Color::White)),
                    Some(Piece::Rook(Color::White)),
                ],
                [ 
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                ],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                ],
                [
                    Some(Piece::Rook(Color::Black)),
                    Some(Piece::Knight(Color::Black)),
                    Some(Piece::Bishop(Color::Black)),
                    Some(Piece::Queen(Color::Black)),
                    Some(Piece::King(Color::Black)),
                    Some(Piece::Bishop(Color::Black)),
                    Some(Piece::Knight(Color::Black)),
                    Some(Piece::Rook(Color::Black)),
                ]
            ],
            pieces: HashMap::from([
                (Piece::Rook(Color::White), HashSet::from([(0, 0), (0, 7)])),
                (Piece::Knight(Color::White), HashSet::from([(0, 1), (0, 6)])),
                (Piece::Bishop(Color::White), HashSet::from([(0, 2), (0, 5)])),
                (Piece::Queen(Color::White), HashSet::from([(0, 3)])),
                (Piece::King(Color::White), HashSet::from([(0, 4)])),
                (Piece::Pawn(Color::White), HashSet::from([
                    (1, 0), (1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7)
                ])),
                (Piece::Rook(Color::Black), HashSet::from([(7, 0), (7, 7)])),
                (Piece::Knight(Color::Black), HashSet::from([(7, 1), (7, 6)])),
                (Piece::Bishop(Color::Black), HashSet::from([(7, 2), (7, 5)])),
                (Piece::Queen(Color::Black), HashSet::from([(7, 3)])),
                (Piece::King(Color::Black), HashSet::from([(7, 4)])),
                (Piece::Pawn(Color::Black), HashSet::from([
                    (6, 0), (6, 1), (6, 2), (6, 3), (6, 4), (6, 5), (6, 6), (6, 7)
                ])),
            ]),
            

        }
    }


    pub fn check_move(&self, start: (usize, usize), end: (usize, usize), game_state: &game::GameState) -> Result<(), &'static str> {
        if start.0 > 7 || start.1 > 7  {
            return Err("Start square out of bounds");
        }
        if end.0 > 7 || end.1 > 7 {
            return Err("End square out of bounds");
        }

        let piece = self.grid[start.0][start.1].ok_or("No piece at start square")?;


        // Check if the start square is occupied by a piece of the correct color
        if piece.color() != game_state.turn {
            return Err("Start square occupied by piece of wrong color");
        }

        // Check if the end square is occupied by a piece of the same color
        if let Some(end_piece) = self.grid[end.0][end.1] {
            if end_piece.color() == piece.color() {
                return Err("End square occupied by piece of same color");
            }
        }

        let is_valid_move = match piece {
            Piece::Pawn(_) => {
                self.check_pawn_move(start, end, game_state)
            }

            Piece::Knight(_) => {
                self.check_knight_move(start, end)
            }

            Piece::Bishop(_) => {
                self.check_bishop_move(start, end)
            }

            Piece::Rook(_) => {
                self.check_rook_move(start, end)
            }

            Piece::Queen(_) => {
                self.check_queen_move(start, end)
            }

            Piece::King(_) => {
                self.check_king_move(start, end, game_state)
            }
            
        };

        if !is_valid_move {
            return Err("Invalid move");
        }

        let mut temp_board = self.clone();

        temp_board.move_piece(start, end); // ok because temp_board is a clone and we know there is a piece at start

        if temp_board.verify_check(game_state.turn)? {
            return Err("Move puts own king in check");
        }

        Ok(())



        

        
    }
        
    /// Should only be used if we know there is a piece at start
    pub fn move_piece (&mut self, start: (usize, usize), end: (usize, usize)) {

        let piece = self.grid[start.0][start.1].unwrap();
        let taken_piece = self.grid[end.0][end.1];

        if taken_piece.is_some() {
            let taken_piece = taken_piece.unwrap();
            self.pieces.get_mut(&taken_piece).unwrap().remove(&end);
        }

        // promotion of pawn, for now always promote to queen TODO: let player choose

        if piece == Piece::Pawn(Color::White) && end.0 == 7 {
            self.grid[start.0][start.1] = None;
            self.grid[end.0][end.1] = Some(Piece::Queen(Color::White));
            self.pieces.get_mut(&Piece::Pawn(Color::White)).unwrap().remove(&start);
            self.pieces.get_mut(&Piece::Queen(Color::White)).unwrap().insert(end);
            return;
        } else if piece == Piece::Pawn(Color::Black) && end.0 == 0 {
            self.grid[start.0][start.1] = None;
            self.grid[end.0][end.1] = Some(Piece::Queen(Color::Black));
            self.pieces.get_mut(&Piece::Pawn(Color::Black)).unwrap().remove(&start);
            self.pieces.get_mut(&Piece::Queen(Color::Black)).unwrap().insert(end);
            return;
        } else {
            self.grid[start.0][start.1] = None;
            self.grid[end.0][end.1] = Some(piece);
            self.pieces.get_mut(&piece).unwrap().remove(&start);
            self.pieces.get_mut(&piece).unwrap().insert(end);
        }

        // move the rook in case of castling
        if piece == Piece::King(Color::White) && start == (0, 4) {
            let rook = Piece::Rook(Color::White);
            if end == (0, 6) {
                self.grid[0][7] = None;
                self.grid[0][5] = Some(rook);
                self.pieces.get_mut(&rook).unwrap().remove(&(0, 7));
                self.pieces.get_mut(&rook).unwrap().insert((0, 5));
            } if end == (0, 2) {
                self.grid[0][0] = None;
                self.grid[0][3] = Some(rook);
                self.pieces.get_mut(&rook).unwrap().remove(&(0, 0));
                self.pieces.get_mut(&rook).unwrap().insert((0, 3));
            }
        } else if piece == Piece::King(Color::Black) && start == (7, 4) {
            let rook = Piece::Rook(Color::Black);
            if end == (7, 6) {
                self.grid[7][7] = None;
                self.grid[7][5] = Some(rook);
                self.pieces.get_mut(&rook).unwrap().remove(&(7, 7));
                self.pieces.get_mut(&rook).unwrap().insert((7, 5));
            } if end == (7, 2) {
                self.grid[7][0] = None;
                self.grid[7][3] = Some(rook);
                self.pieces.get_mut(&rook).unwrap().remove(&(7, 0));
                self.pieces.get_mut(&rook).unwrap().insert((7, 3));
            }
        }

        // take the pawn in case of en passant
        if piece == Piece::Pawn(Color::White) && start.1 != end.1 && taken_piece.is_none(){
            self.grid[start.0][end.1] = None;
            self.pieces.get_mut(&Piece::Pawn(Color::Black)).unwrap().remove(&(start.0, end.1));
        } else if piece == Piece::Pawn(Color::Black) && start.1 != end.1 && taken_piece.is_none(){
            self.grid[start.0][end.1] = None;
            self.pieces.get_mut(&Piece::Pawn(Color::White)).unwrap().remove(&(start.0, end.1));
        }
            
    }

    pub fn make_move(&mut self, start: (usize, usize), end: (usize, usize), game_state: &mut game::GameState)  -> Result<(), &'static str> {
        self.check_move(start, end, game_state)?;

        // update game state
        if self.grid[start.0][start.1] == Some(Piece::King(Color::White)) {
            game_state.white_castle_king_side = false;
            game_state.white_castle_queen_side = false;
        } else if self.grid[start.0][start.1] == Some(Piece::Rook(Color::White)) {
            if start == (0, 0) {
                game_state.white_castle_queen_side = false;
            } else if start == (0, 7) {
                game_state.white_castle_king_side = false;
            }
        } else if self.grid[start.0][start.1] == Some(Piece::King(Color::Black)) {
            game_state.black_castle_king_side = false;
            game_state.black_castle_queen_side = false;
        } else if self.grid[start.0][start.1] == Some(Piece::Rook(Color::Black)) {
            if start == (7, 0) {
                game_state.black_castle_queen_side = false;
            } else if start == (7, 7) {
                game_state.black_castle_king_side = false;
            }
        }

        if self.grid[start.0][start.1] == Some(Piece::Pawn(Color::White)) && start.0 == 1 && end.0 == 3 {
            game_state.en_passant = Some((start.1, Color::White));
        } else if self.grid[start.0][start.1] == Some(Piece::Pawn(Color::Black)) && start.0 == 6 && end.0 == 4 {
            game_state.en_passant = Some((start.1, Color::Black));
        } else {
            game_state.en_passant = None;
        }

        game_state.turn = match game_state.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };


        self.move_piece(start, end);

        Ok(())



    }

}