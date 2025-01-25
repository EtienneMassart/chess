use utils::parse_move;

pub mod chess_logic;
pub mod game;
pub mod utils;

#[cfg(test)]
mod tests {
    mod utils_tests; // Include the test module
}

fn main() {
    let mut game = game::Game::new();
    println!("{}", game.board);
    loop {

        println!("Enter your move for {}: ", game.game_state.turn);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input == "exit\n" {
            break;
        }

        // input should be of the form "a2 a4"
        match parse_move(&input) {
            Ok((start, end)) => {
                match game.make_move(start, end) {
                    Ok(_) => {
                        println!("{}", game.board);
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }



    }

}