mod gui;
mod constants;

//use utils::parse_move;
use chess_core::core_struct::Color;
use chess_core::rules::EndgameStatus;
use chess_core::game::Game;
use macroquad::prelude::*;




// Define the window configuration
fn window_conf() -> Conf {
    Conf {
        window_title: "Chess Game".to_string(),
        window_width: 784, // Match the board size including borders
        window_height: 784,
        fullscreen: false, // Disable fullscreen (optional)
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let textures = gui::load_textures().await.unwrap();

    let mut game = Game::new();

    let mut previous_selected: Option<(usize, usize)> = None;
    let mut selected: Option<(usize, usize)> = None;

    loop {
        clear_background(WHITE);

        gui::draw_board(&game, &textures);

        // Handle input
        if is_mouse_button_pressed(MouseButton::Left) {
            gui::select_square(&mut previous_selected, &mut selected);

            if let (Some((start_y, start_x)), Some((end_y, end_x))) = (previous_selected, selected)
            {
                let start = (start_y, start_x);
                let end = (end_y, end_x);
                if let Err(e) = game.play_move(start, end) {
                    println!("{}", e);
                } else {
                    previous_selected = None;
                    selected = None;

                    // Check if the game is over TODO: Add a popup and a button to restart
                    match game.evaluate_endgame() {
                        EndgameStatus::Ongoing => {}
                        EndgameStatus::Checkmate(Color::White) => {
                            println!("Checkmate! Black wins!");
                        }
                        EndgameStatus::Checkmate(Color::Black) => {
                            println!("Checkmate! White wins!");
                        }
                        EndgameStatus::Stalemate => {
                            println!("Stalemate!");
                        }
                    }

                }
            }
        }

        gui::show_legal_moves(&mut game, selected, &textures);

        next_frame().await;
    }
}
