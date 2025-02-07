mod constants;
mod gui;
mod utils;

use std::cell::Cell;

//use utils::parse_move;
use chess_core::Game;
use macroquad::{prelude::*, ui::{root_ui, Skin}};

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

    let should_quit = Cell::new(false);

    let button_style = root_ui()
        .style_builder()
        .color(GREEN)
        .margin(RectOffset::new(20.0, 20.0, 10.0, 10.0)) 
        .font_size(30)
        .build();

    let label_style = root_ui()
        .style_builder()
        .font_size(40)
        .build();

    let ui_skin = Skin {
        button_style,
        label_style,
        ..root_ui().default_skin()
    };

    root_ui().push_skin(&ui_skin);


    loop {
        clear_background(WHITE);

        gui::draw_board(&game, &textures);

        // Handle input and show promotion menu

        if let Some((column, color)) = game.is_promotion_pending() {
            gui::show_promotion_menu((column, color), &textures);
            if is_mouse_button_pressed(MouseButton::Left) {
                if let Some(square) = utils::select_square() {
                    if let Some(promotion_piece) =
                        utils::select_promotion_piece(square, column, color)
                    {
                        game.promote_pawn(promotion_piece).unwrap();
                    }
                }
            }
        } else if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(square) = utils::select_square() {
                if let Some((row, col)) = selected {
                    if square == (row, col) {
                        selected = None;
                    } else {
                        previous_selected = selected;
                        selected = Some(square);
                    }
                } else {
                    selected = Some(square);
                }
            }

            if let (Some(start), Some(end)) = (previous_selected, selected) {
                if let Err(_e) = game.play_move(start, end) {
                    //println!("{}", _e);
                } else {
                    previous_selected = None;
                    selected = None;

                    game.evaluate_endgame(); // Check if the game is over, will set endgame_status
                }
            }            
        }

        if game.endgame_status() != chess_core::EndgameStatus::Ongoing {
            gui::draw_game_over_box(&should_quit, &mut game);
        }

        if should_quit.get() {
            break;
        }
    
        gui::show_legal_moves(&mut game, selected, &textures);

        next_frame().await;
    }
}
