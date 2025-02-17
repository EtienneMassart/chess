use super::EndgameStatus;
use crate::core_struct::{Board, Color, Piece};
use crate::game::GameState;
use crate::{DrawReason, WinReason};
use std::collections::{HashMap, HashSet};

#[test]
fn test_is_king_in_check() {
    let mut grid = [[None; 8]; 8];
    grid[3][3] = Some(Piece::King(Color::White)); // White king at d4
    grid[5][3] = Some(Piece::Queen(Color::White)); // White queen at d6

    let pieces = HashMap::from([
        (Piece::King(Color::White), HashSet::from([(3, 3)])),
        (Piece::Queen(Color::White), HashSet::from([(5, 3)])),
    ]);
    let mut board = Board { grid, pieces };

    assert!(
        !board.is_king_in_check(Color::White).unwrap_or(true),
        "White king is not in check by white queen"
    );

    board.grid[5][3] = Some(Piece::Queen(Color::Black)); // Black queen at d6
    board
        .pieces
        .get_mut(&Piece::Queen(Color::White))
        .unwrap()
        .remove(&(5, 3));
    board
        .pieces
        .insert(Piece::Queen(Color::Black), HashSet::from([(5, 3)]));

    assert!(
        board.is_king_in_check(Color::White).unwrap_or(false),
        "White king is in check by black queen"
    );

    board.grid[5][3] = Some(Piece::Rook(Color::Black)); // Black rook at d6
    board
        .pieces
        .get_mut(&Piece::Queen(Color::Black))
        .unwrap()
        .remove(&(5, 3));
    board
        .pieces
        .insert(Piece::Rook(Color::Black), HashSet::from([(5, 3)]));

    assert!(
        board.is_king_in_check(Color::White).unwrap_or(false),
        "White king is in check by black rook"
    );

    board.grid[4][3] = Some(Piece::Pawn(Color::Black)); // Black pawn at d5
    board
        .pieces
        .insert(Piece::Pawn(Color::White), HashSet::from([(5, 4)]));

    assert!(
        !board.is_king_in_check(Color::White).unwrap_or(true),
        "Check by rook is blocked by pawn"
    );

    board.grid[4][4] = Some(Piece::Pawn(Color::Black)); // Black pawn at e5
    board
        .pieces
        .insert(Piece::Pawn(Color::Black), HashSet::from([(4, 4)]));

    assert!(
        board.is_king_in_check(Color::White).unwrap_or(false),
        "White king is in check by black pawn"
    );

    board.grid[4][4] = Some(Piece::Pawn(Color::White)); // White pawn at e5
    board
        .pieces
        .insert(Piece::Pawn(Color::White), HashSet::from([(4, 4)]));
    board
        .pieces
        .get_mut(&Piece::Pawn(Color::Black))
        .unwrap()
        .remove(&(4, 4));

    assert!(
        !board.is_king_in_check(Color::White).unwrap_or(true),
        "Check by black rook is blocked by black pawn, white pawn doesn't check"
    );

    board.grid[5][5] = Some(Piece::King(Color::Black)); // Black king at f6
    board
        .pieces
        .insert(Piece::King(Color::Black), HashSet::from([(5, 5)]));

    assert!(
        board.is_king_in_check(Color::Black).unwrap_or(false),
        "Black king is in check by white pawn"
    );

    board.grid = [[None; 8]; 8];
    board.grid[3][3] = Some(Piece::King(Color::White)); // White king at d4
    board.grid[6][6] = Some(Piece::Bishop(Color::Black)); // Black bishop at g7
    board.pieces = HashMap::from([
        (Piece::King(Color::White), HashSet::from([(3, 3)])),
        (Piece::Bishop(Color::Black), HashSet::from([(6, 6)])),
    ]);

    assert!(
        board.is_king_in_check(Color::White).unwrap_or(false),
        "White king is in check by black bishop"
    );

    board.grid[5][5] = Some(Piece::Bishop(Color::White)); // White bishop at f6
    board
        .pieces
        .insert(Piece::Bishop(Color::White), HashSet::from([(5, 5)]));

    assert!(
        !board.is_king_in_check(Color::White).unwrap_or(true),
        "Check by black bishop is blocked by white bishop"
    );

    board.grid[5][4] = Some(Piece::Knight(Color::Black)); // Black knight at f6
    board
        .pieces
        .insert(Piece::Knight(Color::Black), HashSet::from([(5, 4)]));

    assert!(
        board.is_king_in_check(Color::White).unwrap_or(false),
        "White king is in check by black knight"
    );

    board.grid = [[None; 8]; 8];
    board.pieces = Default::default(); // Empty hashmap

    assert!(
        board.is_king_in_check(Color::White).is_err(),
        "King not found"
    );

    board.grid[3][3] = Some(Piece::King(Color::White)); // White king at d4
    board.grid[4][3] = Some(Piece::King(Color::White)); // White king at d5
    board
        .pieces
        .insert(Piece::King(Color::White), HashSet::from([(3, 3), (4, 3)]));

    assert!(
        board.is_king_in_check(Color::White).is_err(),
        "Multiple kings found"
    );

    board.grid[4][3] = Some(Piece::King(Color::Black)); // Black king at d5
    board
        .pieces
        .insert(Piece::King(Color::Black), HashSet::from([(4, 3)]));
    board
        .pieces
        .get_mut(&Piece::King(Color::White))
        .unwrap()
        .remove(&(4, 3));

    assert!(
        board.is_king_in_check(Color::White).unwrap_or(false),
        "White king is in check by black king"
    );
    assert!(
        board.is_king_in_check(Color::Black).unwrap_or(false),
        "Black king is in check by white king"
    );
}

#[test]
fn test_evaluate_endgame() {
    let game_state = GameState {
        turn: Color::Black,
        white_castle_king_side: false,
        white_castle_queen_side: false,
        black_castle_king_side: false,
        black_castle_queen_side: false,
        ..Default::default()
    };

    let mut grid = [[None; 8]; 8];
    grid[7][3] = Some(Piece::King(Color::Black)); // Black king at d8
    grid[6][3] = Some(Piece::Queen(Color::White)); // White queen at d7
    grid[5][3] = Some(Piece::King(Color::White)); // White king at d6

    let pieces = HashMap::from([
        (Piece::King(Color::Black), HashSet::from([(7, 3)])),
        (Piece::Queen(Color::White), HashSet::from([(6, 3)])),
        (Piece::King(Color::White), HashSet::from([(5, 3)])),    
    ]);

    let mut board = Board { grid, pieces };

    assert_eq!(
        board.evaluate_endgame(&game_state),
        EndgameStatus::Win(Color::White, WinReason::Checkmate),
        "White wins by checkmate with queen"
    );

    let mut grid = [[None; 8]; 8];
    grid[7][3] = Some(Piece::King(Color::Black)); // Black king at d8
    grid[6][4] = Some(Piece::Queen(Color::White)); // White queen at d7
    grid[5][3] = Some(Piece::King(Color::White)); // White king at d6

    let pieces = HashMap::from([
        (Piece::King(Color::Black), HashSet::from([(7, 3)])),
        (Piece::Queen(Color::White), HashSet::from([(6, 4)])),
        (Piece::King(Color::White), HashSet::from([(5, 3)])),    
    ]);

    let mut board = Board { grid, pieces };

    assert_eq!(
        board.evaluate_endgame(&game_state),
        EndgameStatus::Ongoing,
        "No checkmate"
    );

    let mut grid = [[None; 8]; 8];
    grid[7][4] = Some(Piece::King(Color::Black)); // Black king on e8
    grid[6][4] = Some(Piece::Queen(Color::White)); // White queen on e7
    grid[5][3] = Some(Piece::Bishop(Color::White)); // White bishop on d6
    grid[6][3] = Some(Piece::King(Color::White)); // White king on d7

    let pieces = HashMap::from([
        (Piece::King(Color::Black), HashSet::from([(7, 4)])),
        (Piece::Queen(Color::White), HashSet::from([(6, 4)])),
        (Piece::Bishop(Color::White), HashSet::from([(5, 3)])),
        (Piece::King(Color::White), HashSet::from([(6, 3)])),
    ]);

    let mut board = Board { grid, pieces };

    assert_eq!(
        board.evaluate_endgame(&game_state),
        EndgameStatus::Win(Color::White, WinReason::Checkmate),
        "White wins by checkmate with queen and bishop"
    );

    let mut grid = [[None; 8]; 8];
    grid[7][4] = Some(Piece::King(Color::Black)); // Black king on e8
    grid[7][5] = Some(Piece::Knight(Color::Black)); // Black knight on f8
    grid[6][4] = Some(Piece::Queen(Color::White)); // White queen on e7
    grid[6][3] = Some(Piece::King(Color::White)); // White king on d7

    let pieces = HashMap::from([
        (Piece::King(Color::Black), HashSet::from([(7, 4)])),
        (Piece::Knight(Color::Black), HashSet::from([(7, 5)])),
        (Piece::Queen(Color::White), HashSet::from([(6, 4)])),
        (Piece::King(Color::White), HashSet::from([(6, 3)])),
    ]);

    let mut board = Board { grid, pieces };

    assert_eq!(
        board.evaluate_endgame(&game_state),
        EndgameStatus::Win(Color::White, WinReason::Checkmate),
        "White wins by smothered checkmate"
    );

    let mut grid = [[None; 8]; 8];
    grid[7][1] = Some(Piece::King(Color::Black)); // Black king on b8
    grid[6][0] = Some(Piece::Pawn(Color::Black)); // Black pawn on a7
    grid[6][1] = Some(Piece::Pawn(Color::Black)); // Black pawn on b7
    grid[6][2] = Some(Piece::Pawn(Color::Black)); // Black pawn on c7
    grid[7][4] = Some(Piece::Rook(Color::White)); // White rook on e8
    grid[0][4] = Some(Piece::King(Color::White)); // White king on e1

    let pieces = HashMap::from([
        (Piece::King(Color::Black), HashSet::from([(7, 1)])),
        (Piece::Pawn(Color::Black), HashSet::from([(6, 0), (6, 1), (6, 2)])),
        (Piece::Rook(Color::White), HashSet::from([(7, 4)])),
        (Piece::King(Color::White), HashSet::from([(0, 4)])),
    ]);

    let mut board = Board { grid, pieces };

    assert_eq!(
        board.evaluate_endgame(&game_state),
        EndgameStatus::Win(Color::White, WinReason::Checkmate),
        "White wins by back rank checkmate"
    );


    let mut grid = [[None; 8]; 8];
    grid[7][0] = Some(Piece::King(Color::Black)); // Black king on a8
    grid[6][0] = Some(Piece::Pawn(Color::Black)); // Black pawn on a7
    grid[5][0] = Some(Piece::Pawn(Color::White)); // White pawn on a6
    grid[0][4] = Some(Piece::King(Color::White)); // White king on e1
    grid[0][1] = Some(Piece::Rook(Color::White)); // White rook on b1

    let pieces = HashMap::from([
        (Piece::King(Color::Black), HashSet::from([(7, 0)])),
        (Piece::Pawn(Color::Black), HashSet::from([(6, 0)])),
        (Piece::Pawn(Color::White), HashSet::from([(5, 0)])),
        (Piece::King(Color::White), HashSet::from([(0, 4)])),
        (Piece::Rook(Color::White), HashSet::from([(0, 1)])),
    ]);

    let mut board = Board { grid, pieces };

    assert_eq!(
        board.evaluate_endgame(&game_state),
        EndgameStatus::Draw(DrawReason::Stalemate),
        "Stalemate—Black has no legal moves"
    );



}
