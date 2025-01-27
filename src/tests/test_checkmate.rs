use std::collections::{HashMap, HashSet};
use crate::chess_logic::{Color, Piece, Board}; 

#[test]
fn test_verify_check() {
    let mut grid = [[None; 8]; 8];
    grid[3][3] = Some(Piece::King(Color::White)); // White king at d4
    grid[5][3] = Some(Piece::Queen(Color::White)); // White queen at d6

    let pieces = HashMap::from([
        (Piece::King(Color::White), HashSet::from([(3, 3)])),
        (Piece::Queen(Color::White), HashSet::from([(5, 3)]))
    ]);
    let mut board = Board { grid, pieces };

    assert!(!board.verify_check(Color::White).unwrap_or(true), "White king is not in check by white queen");

    board.grid[5][3] = Some(Piece::Queen(Color::Black)); // Black queen at d6
    board.pieces.get_mut(&Piece::Queen(Color::White)).unwrap().remove(&(5, 3));
    board.pieces.insert(Piece::Queen(Color::Black), HashSet::from([(5, 3)]));

    assert!(board.verify_check(Color::White).unwrap_or(false), "White king is in check by black queen");

    board.grid[5][3] = Some(Piece::Rook(Color::Black)); // Black rook at d6
    board.pieces.get_mut(&Piece::Queen(Color::Black)).unwrap().remove(&(5, 3));
    board.pieces.insert(Piece::Rook(Color::Black), HashSet::from([(5, 3)]));

    assert!(board.verify_check(Color::White).unwrap_or(false), "White king is in check by black rook");
    

    board.grid[4][3] = Some(Piece::Pawn(Color::Black)); // Black pawn at d5
    board.pieces.insert(Piece::Pawn(Color::White), HashSet::from([(5, 4)]));

    assert!(!board.verify_check(Color::White).unwrap_or(true), "Check by rook is blocked by pawn");

    board.grid[4][4] = Some(Piece::Pawn(Color::Black)); // Black pawn at e5
    board.pieces.insert(Piece::Pawn(Color::Black), HashSet::from([(4, 4)]));

    assert!(board.verify_check(Color::White).unwrap_or(false), "White king is in check by black pawn");

    board.grid[4][4] = Some(Piece::Pawn(Color::White)); // White pawn at e5
    board.pieces.insert(Piece::Pawn(Color::White), HashSet::from([(4, 4)]));
    board.pieces.get_mut(&Piece::Pawn(Color::Black)).unwrap().remove(&(4, 4));

    assert!(!board.verify_check(Color::White).unwrap_or(true), "Check by black rook is blocked by black pawn, white pawn doesn't check");

    board.grid[5][5] = Some(Piece::King(Color::Black)); // Black king at f6
    board.pieces.insert(Piece::King(Color::Black), HashSet::from([(5, 5)]));

    assert!(board.verify_check(Color::Black).unwrap_or(false), "Black king is in check by white pawn");


    board.grid = [[None; 8]; 8];
    board.grid[3][3] = Some(Piece::King(Color::White)); // White king at d4
    board.grid[6][6] = Some(Piece::Bishop(Color::Black)); // Black bishop at g7
    board.pieces = HashMap::from([
        (Piece::King(Color::White), HashSet::from([(3, 3)])),
        (Piece::Bishop(Color::Black), HashSet::from([(6, 6)]))
    ]);

    assert!(board.verify_check(Color::White).unwrap_or(false), "White king is in check by black bishop");

    board.grid[5][5] = Some(Piece::Bishop(Color::White)); // White bishop at f6
    board.pieces.insert(Piece::Bishop(Color::White), HashSet::from([(5, 5)]));

    assert!(!board.verify_check(Color::White).unwrap_or(true), "Check by black bishop is blocked by white bishop");

    board.grid[5][4] = Some(Piece::Knight(Color::Black)); // Black knight at f6
    board.pieces.insert(Piece::Knight(Color::Black), HashSet::from([(5, 4)]));

    assert!(board.verify_check(Color::White).unwrap_or(false), "White king is in check by black knight");

    board.grid = [[None; 8]; 8];
    board.pieces = Default::default(); // Empty hashmap

    assert!(board.verify_check(Color::White).is_err(), "King not found");

    board.grid[3][3] = Some(Piece::King(Color::White)); // White king at d4
    board.grid[4][3] = Some(Piece::King(Color::White)); // White king at d5
    board.pieces.insert(Piece::King(Color::White), HashSet::from([(3, 3), (4, 3)]));


    assert!(board.verify_check(Color::White).is_err(), "Multiple kings found");

    board.grid[4][3] = Some(Piece::King(Color::Black)); // Black king at d5
    board.pieces.insert(Piece::King(Color::Black), HashSet::from([(4, 3)]));
    board.pieces.get_mut(&Piece::King(Color::White)).unwrap().remove(&(4, 3));

    assert!(board.verify_check(Color::White).unwrap_or(false), "White king is in check by black king");
    assert!(board.verify_check(Color::Black).unwrap_or(false), "Black king is in check by white king");

    
}
