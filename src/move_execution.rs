use crate::core_struct::{Color, Piece, Board};

impl Board {
    /// Should only be used if we know there is a piece at start
    pub fn execute_move(
        &mut self,
        start: (usize, usize),
        end: (usize, usize),
    ) -> (Option<(Piece, (usize, usize))>, bool) {
        let mut result: (Option<(Piece, (usize, usize))>, bool) = (None, false); // (Option<taken_piece, taken_position>, promotion)

        let piece = self.grid[start.0][start.1].unwrap();
        let taken_piece = self.grid[end.0][end.1];

        if taken_piece.is_some() {
            let taken_piece = taken_piece.unwrap();
            self.pieces.get_mut(&taken_piece).unwrap().remove(&end);
            result.0 = Some((taken_piece, end));
        }

        // promotion of pawn, for now always promote to queen TODO: let player choose

        if piece == Piece::Pawn(Color::White) && end.0 == 7 {
            self.grid[start.0][start.1] = None;
            self.grid[end.0][end.1] = Some(Piece::Queen(Color::White));
            self.pieces
                .get_mut(&Piece::Pawn(Color::White))
                .unwrap()
                .remove(&start);
            self.pieces
                .get_mut(&Piece::Queen(Color::White))
                .unwrap()
                .insert(end);
            result.1 = true;
        } else if piece == Piece::Pawn(Color::Black) && end.0 == 0 {
            self.grid[start.0][start.1] = None;
            self.grid[end.0][end.1] = Some(Piece::Queen(Color::Black));
            self.pieces
                .get_mut(&Piece::Pawn(Color::Black))
                .unwrap()
                .remove(&start);
            self.pieces
                .get_mut(&Piece::Queen(Color::Black))
                .unwrap()
                .insert(end);
            result.1 = true;
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
            }
            if end == (0, 2) {
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
            }
            if end == (7, 2) {
                self.grid[7][0] = None;
                self.grid[7][3] = Some(rook);
                self.pieces.get_mut(&rook).unwrap().remove(&(7, 0));
                self.pieces.get_mut(&rook).unwrap().insert((7, 3));
            }
        }

        // take the pawn in case of en passant
        if piece == Piece::Pawn(Color::White) && start.1 != end.1 && taken_piece.is_none() {
            self.grid[start.0][end.1] = None;
            self.pieces
                .get_mut(&Piece::Pawn(Color::Black))
                .unwrap()
                .remove(&(start.0, end.1));
            result.0 = Some((Piece::Pawn(Color::Black), (start.0, end.1)));
        } else if piece == Piece::Pawn(Color::Black) && start.1 != end.1 && taken_piece.is_none() {
            self.grid[start.0][end.1] = None;
            self.pieces
                .get_mut(&Piece::Pawn(Color::White))
                .unwrap()
                .remove(&(start.0, end.1));
            result.0 = Some((Piece::Pawn(Color::White), (start.0, end.1)));
        }

        result
    }

    pub fn undo_move(
        &mut self,
        start: (usize, usize),
        end: (usize, usize),
        taken_piece: Option<(Piece, (usize, usize))>,
        promotion: bool,
    ) {
        // taken_piece is the piece that was taken in the move (we add position for en passant)

        let piece = if promotion {
            Piece::Pawn(self.grid[end.0][end.1].unwrap().color())
        } else {
            self.grid[end.0][end.1].unwrap()
        };

        self.grid[start.0][start.1] = Some(piece);
        self.grid[end.0][end.1] = None;
        self.pieces.get_mut(&piece).unwrap().remove(&end);
        self.pieces.get_mut(&piece).unwrap().insert(start);

        if taken_piece.is_some() {
            let (taken_piece, taken_position) = taken_piece.unwrap();
            self.grid[taken_position.0][taken_position.1] = Some(taken_piece);
            self.pieces
                .get_mut(&taken_piece)
                .unwrap()
                .insert(taken_position);
        }

        // move the rook back in case of castling
        if piece == Piece::King(Color::White) && start == (0, 4) {
            let rook = Piece::Rook(Color::White);
            if end == (0, 6) {
                self.grid[0][7] = Some(rook);
                self.grid[0][5] = None;
                self.pieces.get_mut(&rook).unwrap().insert((0, 7));
                self.pieces.get_mut(&rook).unwrap().remove(&(0, 5));
            }
            if end == (0, 2) {
                self.grid[0][0] = Some(rook);
                self.grid[0][3] = None;
                self.pieces.get_mut(&rook).unwrap().insert((0, 0));
                self.pieces.get_mut(&rook).unwrap().remove(&(0, 3));
            }
        } else if piece == Piece::King(Color::Black) && start == (7, 4) {
            let rook = Piece::Rook(Color::Black);
            if end == (7, 6) {
                self.grid[7][7] = Some(rook);
                self.grid[7][5] = None;
                self.pieces.get_mut(&rook).unwrap().insert((7, 7));
                self.pieces.get_mut(&rook).unwrap().remove(&(7, 5));
            }
            if end == (7, 2) {
                self.grid[7][0] = Some(rook);
                self.grid[7][3] = None;
                self.pieces.get_mut(&rook).unwrap().insert((7, 0));
                self.pieces.get_mut(&rook).unwrap().remove(&(7, 3));
            }
        }
    }
}
