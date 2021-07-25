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
    - [ ] Implement input box gui
    - [ ] Implement gui for displaying text (ex. errors)
    - [ ] Add rematch option
    - [ ] Keep track of games won/lost
    - [x] Check for win, lose or draw
  - Multiplayer:
    - [ ] Fix bug where idle player can set his cursor position beforehand
    - [x] Implement proper exception handling for client and server function
    - [x] Send packets whenever cursor is moved instead of when the move is made
  - AI:
    - [ ] Implement Min Max algorithm
    - [ ] Implement Neural Network algorithm