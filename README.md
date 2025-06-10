This is a **Work In Progress**.

## Organisation

`chess_core` is a library that implements the core logic of chess, including move generation, validation, and game state management. 
You can try it out as a single-player game against yourself (no AI was implemented) with `cargo run --bin single_player`

`chess_client` provides a client interface for playing chess games. It is still under development and not yet functional.
The goal is to allow users to connect to a chess server, join games, and play against other players, but currently it is mostly a copy of the user interface from the single-player game in chess_core.

`chess_server` is a server application that allows multiple players to connect and play chess games. It should manage connections, game sessions, and player interactions, with verification of game rules and state.
It is also under development and not yet functional.

`chess_network` is a library containing common networking structures such as messages used by both chess_client and chess_server.
