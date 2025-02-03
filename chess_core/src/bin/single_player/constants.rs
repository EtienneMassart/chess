pub const BOARD_SIZE: f32 = 784.0; // Full board size including borders
pub const BORDER_SIZE: f32 = 8.0; // Size of the border on each side
pub const PLAYABLE_SIZE: f32 = BOARD_SIZE - BORDER_SIZE * 2.0; // Playable area size
pub const TILE_SIZE: f32 = PLAYABLE_SIZE / 8.0; // Size of each square (96 pixels)