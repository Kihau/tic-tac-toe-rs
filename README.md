# TicTacToe game written in rust
## Libraries used:
  - [Crossterm](https://github.com/crossterm-rs/crossterm) - extended console control
  - [Dns-lookup](https://github.com/keeperofdakeys/dns-lookup/) - resolving dns ips
## Consepts used in the game: 
  - MinMax algorithm
  - Neural Network
  - Simple networking
## TO DO:
  - Game:
    - [ ] Check for win, lose or draw
    - [ ] Implement gui input box
    - [ ] Add rematch option
    - [ ] Keep track of games won/lost
  - Multiplayer:
    - [ ] Fix bug where idle player can set his cursor position beforehand
    - [ ] Implement proper exception handling for client and server function
    - [x] Send packets whenever cursor is moved instead of when the move is made
  - AI:
    - [ ] Implement Min Max algorithm
    - [ ] Implement Neural Network algorithm