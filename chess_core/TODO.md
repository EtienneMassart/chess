
FOR CHESS LOGIC

FOR GUI
- Add a menu to resign, offer draw, etc.
- Add sound

FOR MULTIPLAYER
- Implement server and client architecture

Unit tests


STEPS TO MAKE A MOVE:

- Get the move from the server

- Check that the move is possible
  - not out of bound
  - there is a piece at start
  - no piece of the same color at end
  - the piece can go to end
  - nothing blocks

- Check that the position after move is not check for the playing side

- Check if the move is a promotion
    - If it is, ask the player what piece he wants to promote to
    - 

- Modify the game state (board position, variables)

- Check if there is no stalemate or checkmate

- Send the move to the server



STEPS TO VERIFY STALEMATE:

For every piece of color:
    For every square of the board:
        Verify if the move is possible
        verify_check(color) after that move


If no move is possible then it's stalemate
If no move is possible AND you're in check it's checkmate

