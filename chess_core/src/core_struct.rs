use std::{collections::{HashMap, HashSet}, fmt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
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
pub(crate) struct Board {
    pub(crate) grid: [[Option<Piece>; 8]; 8],
    pub(crate) pieces: HashMap<Piece, HashSet<(usize, usize)>>
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

}