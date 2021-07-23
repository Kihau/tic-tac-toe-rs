use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    style::Color,
};
use std::io::stdout;

// move_num should be dirrectly conneted to game turn
pub struct PlayerType {
    pub move_char: char,
    pub move_num: u8,
    pub move_color: Color,
}

pub const CROSS: PlayerType = PlayerType {
    move_char: 'X',
    move_num: 1,
    move_color: Color::Red,
};

pub const CIRCLE: PlayerType = PlayerType {
    move_char: 'O',
    move_num: 2,
    move_color: Color::Blue,
};

#[allow(dead_code)]
pub const TRRIANGLE: PlayerType = PlayerType {
    move_char: '▲',
    move_num: 2,
    move_color: Color::Green,
};

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub enum GameAction {
    ChangePos(Position),
    MakeMove(Position),
    ResetGame,
    ExitGame,
    NoAction,
}

impl GameAction {
    pub fn retrieve_data(data: &[u8]) -> Self {
        match data[0] {
            0 => Self::ChangePos(Position::new(data[1] as usize, data[2] as usize)),
            1 => Self::MakeMove(Position::new(data[1] as usize, data[2] as usize)),
            2 => Self::ResetGame,
            3 => Self::ExitGame,
            4 => Self::NoAction,
            _ => unreachable!("heck"),
        }
    }

    pub fn send_data(&self) -> Vec<u8> {
        match self {
            Self::ChangePos(Position { x, y }) => vec![0, *x as u8, *y as u8],
            Self::MakeMove(Position { x, y }) => vec![1, *x as u8, *y as u8],
            Self::ResetGame => vec![2, 0, 0],
            Self::ExitGame => vec![3, 0, 0],
            Self::NoAction => vec![4, 0, 0],
        }
    }
}

/* ---- Board Characters ----
    ╗ ╔ ╝ ╚
    ║ ═ ╠ ╬ ╣
    ╦ ╩

    Example board:
    ╔═════╦═════╦═════╗
    ║     ║     ║     ║
    ╠═════╬═════╬═════╣
    ║     ║     ║     ║
    ╚═════╩═════╩═════╝
*/

pub struct Game {
    pub board: Vec<Vec<u8>>, // Potentially change u8 to custom enum token (or just a char)
    pub board_size: usize,
    pub board_string: String,
}

// Also check if the game is over
impl Game {
    pub fn new(board_size: usize) -> Self {
        let board_string = Game::generate_board(board_size);
        let board = vec![vec![0; board_size]; board_size];
        Self {
            board,
            board_size,
            board_string,
        }
    }

    pub fn generate_board(size: usize) -> String {
        let mut top = String::from("╔");
        let mut middle = String::from("╠");
        let mut bottom = String::from("╚");
        let mut gap = String::from("║");

        for _ in 0..size - 1 {
            top.push_str("═════╦");
            middle.push_str("═════╬");
            bottom.push_str("═════╩");
            gap.push_str("     ║");
        }

        top.push_str("═════╗\n");
        middle.push_str("═════╣\n");
        bottom.push_str("═════╝");
        gap.push_str("     ║\n");

        let mut board = String::new();
        board.push_str(&top);

        for _ in 0..size - 1 {
            board.push_str(&gap);
            board.push_str(&middle);
        }

        board.push_str(&gap);
        board.push_str(&bottom);

        board
    }

    pub fn make_move(&self, mut pos: Position) -> Option<Position> {
        loop {
            execute!(
                stdout(),
                crossterm::cursor::Show,
                crossterm::cursor::MoveTo(pos.x as u16 * 6 + 3, pos.y as u16 * 2 + 1)
            )
            .unwrap();

            if let Event::Key(event) = read().unwrap() {
                match event.code {
                    KeyCode::Up | KeyCode::Char('w') if pos.y > 0 => pos.y -= 1,
                    KeyCode::Down | KeyCode::Char('s') if pos.y < self.board_size - 1 => pos.y += 1,
                    KeyCode::Left | KeyCode::Char('a') if pos.x > 0 => pos.x -= 1,
                    KeyCode::Right | KeyCode::Char('d') if pos.x < self.board_size - 1 => {
                        pos.x += 1
                    }
                    KeyCode::Enter | KeyCode::Char(' ') if self.board[pos.x][pos.y] == 0 => {
                        return Some(pos)
                    }
                    KeyCode::Esc => return None,
                    _ => {}
                }
            }
        }
    }

    pub fn do_action(&self, pos: &Position) -> GameAction {
        execute!(
            stdout(),
            crossterm::cursor::MoveTo(pos.x as u16 * 6 + 3, pos.y as u16 * 2 + 1)
        )
        .unwrap();

        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Up | KeyCode::Char('w') if pos.y > 0 => {
                    return GameAction::ChangePos(Position::new(pos.x, pos.y - 1))
                }
                KeyCode::Down | KeyCode::Char('s') if pos.y < self.board_size - 1 => {
                    return GameAction::ChangePos(Position::new(pos.x, pos.y + 1))
                }
                KeyCode::Left | KeyCode::Char('a') if pos.x > 0 => {
                    return GameAction::ChangePos(Position::new(pos.x - 1, pos.y))
                }
                KeyCode::Right | KeyCode::Char('d') if pos.x < self.board_size - 1 => {
                    return GameAction::ChangePos(Position::new(pos.x + 1, pos.y))
                }
                KeyCode::Enter | KeyCode::Char(' ') if self.board[pos.x][pos.y] == 0 => {
                    return GameAction::MakeMove(Position::new(pos.x, pos.y))
                }
                KeyCode::Char('r') => return GameAction::ResetGame,
                KeyCode::Esc => return GameAction::ExitGame,
                _ => {}
            }
        }

        GameAction::NoAction
    }

    // Check and return current state - Plyaer1(char), Player2(char), Draw, None
    #[allow(dead_code)]
    fn check_result() {}
}
