
use std::collections::{HashMap, HashSet};
use crate::core_struct::{Color, Piece, Board}; 
use crate::game::GameState;



#[test]
fn test_pawn_move() {
    // Initial board setup
    let mut grid: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
    grid[1][0] = Some(Piece::Pawn(Color::White)); // White pawn in a2 that can move 1 or 2 squares but can't take
    grid[4][1] = Some(Piece::Pawn(Color::Black)); // Black pawn in b4 for en passant
    grid[4][2] = Some(Piece::Pawn(Color::White)); // White pawn in c4 taking en passant
    grid[1][3] = Some(Piece::Pawn(Color::White)); // White pawn at d2 that can't move forward but can take
    grid[2][3] = Some(Piece::Pawn(Color::Black)); // Black pawn at d3 that blocks white pawn
    grid[2][4] = Some(Piece::Pawn(Color::Black)); // Black pawn at e3 that will be taken

    let board = Board { grid, pieces: Default::default() };

    // Initial GameState
    let mut state = GameState {
        turn: Color::White,
        white_castle_king_side: true,
        black_castle_king_side: true,
        white_castle_queen_side: true,
        black_castle_queen_side: true,
        en_passant: Some((2, Color::Black)), // Black pawn made a 2-square move to (4, 3)
    };

    assert!(board.is_valid_pawn_move((1, 0), (2, 0), &state),"White pawn moves 1 square forward");
    assert!(board.is_valid_pawn_move((1, 0), (3, 0), &state), "White pawn moves 2 squares forward");
    assert!(!board.is_valid_pawn_move((1, 0), (2, 1), &state), "White pawn can't take");
    assert!(!board.is_valid_pawn_move((1, 0), (4, 4), &state), "White pawn can't make random move");
    assert!(!board.is_valid_pawn_move((4, 2), (5, 1), &state), "White pawn can take en passant");
    assert!(!board.is_valid_pawn_move((1, 3), (2, 3), &state), "White pawn can't move forward"); // 
    assert!(!board.is_valid_pawn_move((1, 3), (3, 3), &state), "White pawn can't move 2 squares forward"); // 
    assert!(board.is_valid_pawn_move((1, 3), (2, 4), &state), "White pawn can take"); // 


    
    state.turn = Color::Black;
    state.en_passant = None;

    assert!(!board.is_valid_pawn_move((4, 1), (5, 1), &state)); // Black pawn can't move backward
    assert!(board.is_valid_pawn_move((4, 1), (3, 1), &state)); // Black pawn can move 1 square forward
}

#[test]
fn test_knight_move() {
    let mut grid: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
    grid[3][3] = Some(Piece::Knight(Color::White)); // White knight at d4
    grid[5][4] = Some(Piece::Knight(Color::Black)); // Black knight at e6

    let board = Board { grid, pieces: Default::default() };

    assert!(board.is_valid_knight_move((3, 3), (5, 4)), "White knight moves 2 squares forward and 1 square right");
    assert!(board.is_valid_knight_move((3, 3), (5, 2)), "White knight moves 2 squares forward and 1 square left");
    assert!(board.is_valid_knight_move((3, 3), (4, 5)), "White knight moves 1 square forward and 2 squares right");
    assert!(board.is_valid_knight_move((3, 3), (4, 1)), "White knight moves 1 square forward and 2 squares left");
    assert!(board.is_valid_knight_move((3, 3), (2, 5)), "White knight moves 1 square backward and 2 squares right");
    assert!(board.is_valid_knight_move((3, 3), (2, 1)), "White knight moves 1 square backward and 2 squares left");
    assert!(board.is_valid_knight_move((3, 3), (1, 4)), "White knight moves 2 squares backward and 1 square right");
    assert!(board.is_valid_knight_move((3, 3), (1, 2)), "White knight moves 2 squares backward and 1 square left");

    assert!(!board.is_valid_knight_move((3, 3), (4, 3)), "White knight can't move 1 square forward");
    assert!(!board.is_valid_knight_move((3, 3), (3, 5)), "White knight can't move 2 squares right");
    assert!(!board.is_valid_knight_move((3, 3), (5, 3)), "White knight can't move 2 squares forward");
    assert!(!board.is_valid_knight_move((3, 3), (4, 2)), "White knight can't move 1 square diagonally");
    assert!(!board.is_valid_knight_move((3, 3), (5, 1)), "White knight can't move 2 squares diagonally");

    assert!(board.is_valid_knight_move((5, 4), (3, 3),), "Black knight moves 2 squares backward and 1 square left");
    assert!(board.is_valid_knight_move((5, 4), (7, 5),), "Black knight moves 2 squares forward and 1 square right");
    assert!(board.is_valid_knight_move((5, 4), (6, 6),), "Black knight moves 1 square forward and 2 squares right");

    
}

#[test]
fn test_bishop_move() {
    let mut grid: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
    grid[3][3] = Some(Piece::Bishop(Color::White)); // White bishop at d4
    grid[5][5] = Some(Piece::Bishop(Color::Black)); // Black bishop at f6
    grid[0][7] = Some(Piece::Bishop(Color::White)); // White bishop at h1

    let board = Board { grid, pieces: Default::default() };

    assert!(board.is_valid_bishop_move((3, 3), (1, 1)), "White bishop moves 2 squares backward and 2 squares left");
    assert!(board.is_valid_bishop_move((3, 3), (1, 5)), "White bishop moves 2 squares backward and 2 squares right");
    assert!(board.is_valid_bishop_move((3, 3), (5, 1)), "White bishop moves 2 squares forward and 2 squares left");
    assert!(board.is_valid_bishop_move((3, 3), (5, 5)), "White bishop moves 2 squares forward and 2 squares right");
    assert!(board.is_valid_bishop_move((3, 3), (2, 2)), "White bishop moves 1 square backward and 1 square left");
    assert!(board.is_valid_bishop_move((3, 3), (2, 4)), "White bishop moves 1 square backward and 1 square right");
    assert!(board.is_valid_bishop_move((3, 3), (4, 2)), "White bishop moves 1 square forward and 1 square left");
    assert!(board.is_valid_bishop_move((3, 3), (4, 4)), "White bishop moves 1 square forward and 1 square right");
    assert!(board.is_valid_bishop_move((3, 3), (6, 0)), "White bishop moves 3 squares forward and 3 squares left");
    assert!(board.is_valid_bishop_move((3, 3), (0, 6)), "White bishop moves 3 squares backward and 3 squares right");
    assert!(board.is_valid_bishop_move((0, 7), (7, 0)), "White bishop moves 7 squares forward and 7 squares left");



    assert!(!board.is_valid_bishop_move((3, 3), (6, 6)), "White bishop can't move through the black bishop");
    assert!(!board.is_valid_bishop_move((3, 3), (7, 7)), "White bishop can't move through the black bishop");
    assert!(!board.is_valid_bishop_move((5, 5), (2, 2)), "Black bishop can't move through the white bishop");
    assert!(!board.is_valid_bishop_move((5, 5), (0, 0)), "Black bishop can't move through the white bishop");
    assert!(!board.is_valid_bishop_move((3, 3), (3, 4)), "White bishop can't move 1 square forward");
    assert!(!board.is_valid_bishop_move((3, 3), (4, 3)), "White bishop can't move 1 square right");


}

#[test]
fn test_rook_move() {
    let mut grid: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
    grid[3][3] = Some(Piece::Rook(Color::White)); // White rook at d4
    grid[5][3] = Some(Piece::Rook(Color::Black)); // Black rook at d6
    grid[0][0] = Some(Piece::Rook(Color::White)); // White rook at a1

    let board = Board { grid, pieces: Default::default() };

    assert!(board.is_valid_rook_move((3, 3), (0, 3)), "White rook moves 3 squares backward");
    assert!(board.is_valid_rook_move((3, 3), (3, 0)), "White rook moves 3 squares left");
    assert!(board.is_valid_rook_move((3, 3), (3, 6)), "White rook moves 3 squares right");
    assert!(board.is_valid_rook_move((3, 3), (2, 3)), "White rook moves 1 square backward");
    assert!(board.is_valid_rook_move((3, 3), (3, 2)), "White rook moves 1 square left");
    assert!(board.is_valid_rook_move((3, 3), (4, 3)), "White rook moves 1 square forward");
    assert!(board.is_valid_rook_move((3, 3), (3, 4)), "White rook moves 1 square right");
    assert!(board.is_valid_rook_move((0, 0), (7, 0)), "White rook moves 7 squares forward");
    assert!(board.is_valid_rook_move((0, 0), (0, 7)), "White rook moves 7 squares right");


    assert!(!board.is_valid_rook_move((3, 3), (6, 3)), "White rook can't move through black rook");
    assert!(!board.is_valid_rook_move((3, 3), (4, 2)), "White rook can't move diagonally");
    assert!(!board.is_valid_rook_move((3, 3), (6, 5)), "White rook can't move to random square");
}

#[test]
fn test_queen_move() {
    // Should it be really tested? It's just a combination of rook and bishop moves

}

#[test]
fn test_king_move(){ 
    let mut grid: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
    grid[3][3] = Some(Piece::King(Color::White)); // White king at d4
    grid[5][3] = Some(Piece::King(Color::Black)); // Black king at d6

    let mut board = Board { grid, pieces: Default::default() };

    let game_state = GameState {
        turn: Color::White,
        white_castle_king_side: true,
        black_castle_king_side: true,
        white_castle_queen_side: true,
        black_castle_queen_side: true,
        en_passant: None,
    };

    assert!(board.is_valid_king_move((3, 3), (2, 2), &game_state), "White king moves 1 square backward and 1 square left");
    assert!(board.is_valid_king_move((3, 3), (2, 3), &game_state), "White king moves 1 square backward");
    assert!(board.is_valid_king_move((3, 3), (2, 4), &game_state), "White king moves 1 square backward and 1 square right");
    assert!(board.is_valid_king_move((3, 3), (3, 4), &game_state), "White king moves 1 square right");
    assert!(board.is_valid_king_move((3, 3), (4, 4), &game_state), "White king moves 1 square forward and 1 square right");
    assert!(board.is_valid_king_move((3, 3), (4, 3), &game_state), "White king moves 1 square forward");
    assert!(board.is_valid_king_move((3, 3), (4, 2), &game_state), "White king moves 1 square forward and 1 square left");
    assert!(board.is_valid_king_move((3, 3), (3, 2), &game_state), "White king moves 1 square left");

    assert!(!board.is_valid_king_move((3, 3), (5, 3), &game_state), "White king can't move 2 squares forward");
    assert!(!board.is_valid_king_move((3, 3), (5, 5), &game_state), "White king can't move 2 squares diagonally");
    assert!(!board.is_valid_king_move((3, 3), (3, 5), &game_state), "White king can't move 2 squares right");

    // Test for castling
    let mut grid: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
    grid[0][4] = Some(Piece::King(Color::White)); // White king at e1
    grid[0][0] = Some(Piece::Rook(Color::White)); // White rook at a1
    grid[0][7] = Some(Piece::Rook(Color::White)); // White rook at h1
    grid[7][4] = Some(Piece::King(Color::Black)); // Black king at e8
    grid[7][0] = Some(Piece::Rook(Color::Black)); // Black rook at a8
    grid[7][7] = Some(Piece::Rook(Color::Black)); // Black rook at h8

    let pieces = HashMap::from([
        (Piece::King(Color::White), HashSet::from([(0, 4)])),
        (Piece::Rook(Color::White), HashSet::from([(0, 0), (0, 7)])),
        (Piece::King(Color::Black), HashSet::from([(7, 4)])),
        (Piece::Rook(Color::Black), HashSet::from([(7, 0), (7, 7)]))
    ]);

    let mut board = Board { grid, pieces };

    let mut game_state = GameState {
        turn: Color::White,
        white_castle_king_side: true,
        black_castle_king_side: true,
        white_castle_queen_side: true,
        black_castle_queen_side: true,
        en_passant: None,
    };

    assert!(board.is_valid_king_move((0, 4), (0, 6), &game_state), "White king castles kingside");
    assert!(board.is_valid_king_move((0, 4), (0, 2), &game_state), "White king castles queenside");

    board.grid[2][4] = Some(Piece::Rook(Color::Black)); // Black rook at e3 giving check
    board.pieces.get_mut(&Piece::Rook(Color::Black)).unwrap().insert((2, 4));

    assert!(!board.is_valid_king_move((0, 4), (0, 6), &game_state), "White king can't castle kingside because of check");
    assert!(!board.is_valid_king_move((0, 4), (0, 2), &game_state), "White king can't castle queenside because of check");

    board.execute_move((2, 4), (2, 5)); // Move the black rook to f3 to cut castling on king side


    assert!(!board.is_valid_king_move((0, 4), (0, 6), &game_state), "White king can't castle kingside because of the rook at f3");
    assert!(board.is_valid_king_move((0, 4), (0, 2), &game_state), "White king can castle queenside because the rook is not blocking");

    board.execute_move((2, 5), (2, 6)); // Move the black rook to g3 it should not cut castling anymore

    assert!(board.is_valid_king_move((0, 4), (0, 6), &game_state), "White king can castle kingside because the rook is not blocking");

    board.execute_move((2, 6), (2, 3)); // Move the black rook to d3 to cut castling on queen side

    assert!(!board.is_valid_king_move((0, 4), (0, 2), &game_state), "White king can't castle queenside because of the rook at d3");


    game_state.turn = Color::Black;

    assert!(board.is_valid_king_move((7, 4), (7, 6), &game_state), "Black king castles kingside");
    assert!(board.is_valid_king_move((7, 4), (7, 2), &game_state), "Black king castles queenside");

}

