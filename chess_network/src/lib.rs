use chess_core::{PromotionPiece, EndgameStatus};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    SendMove((usize, usize), (usize, usize), Option<PromotionPiece>),
    Resign,
    OfferDraw,
    AcceptDraw,
    DeclineDraw,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    MoveAccepted,
    MoveRejected,
    OpponentMove((usize, usize), (usize, usize), Option<PromotionPiece>),
    OfferDraw,
    AcceptDraw,
    DeclineDraw,
    GameOver(EndgameStatus)
}