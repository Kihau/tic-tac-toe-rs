use std::{
    io::{stdin, stdout, Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

use crossterm::{self, cursor::MoveTo, event::*, execute, style::*, terminal::*, *};
use dns_lookup::lookup_host;

use crate::game::*;

// Starting sides should be switched after every game
// Add a button to concede (ex: Ecp -> "Are you sure you want to surrender?" )

pub fn take_turn(stream: &mut TcpStream, game: &mut Game, player: PlayerType, mut pos: Position) {
    loop {
        let action = game.do_action(&pos);

        // Handle server shutdown
        stream.write(&action.send_data()[..]).unwrap();
        match &action {
            GameAction::ChangePos(Position { x, y }) => {
                pos = Position::new(*x, *y);
                execute!(stdout(), MoveTo(pos.x as u16 * 6 + 3, pos.y as u16 * 2 + 1),).unwrap();
            }
            GameAction::MakeMove(Position { x, y }) => {
                execute!(
                    stdout(),
                    //MoveTo(pos.x as u16 * 6 + 3, pos.y as u16 * 2 + 1),
                    SetForegroundColor(player.move_color),
                    Print(player.move_char),
                    SetForegroundColor(Color::Reset),
                )
                .unwrap();

                pos = Position::new(*x, *y);
                game.board[pos.x][pos.y] = player.move_num;
                return;
            }
            GameAction::ResetGame => return,
            GameAction::ExitGame => {
                stream.shutdown(Shutdown::Both).unwrap();
                return;
            }
            GameAction::NoAction => {}
        }
    }
}

pub fn wait_turn(stream: &mut TcpStream, game: &mut Game, player: PlayerType) -> Option<Position> {
    loop {
        let mut buffer = [0u8; 3];
        stream.read(&mut buffer).unwrap();

        let action = GameAction::retrieve_data(&buffer);
        match action {
            GameAction::ChangePos(pos) => {
                execute!(stdout(), MoveTo(pos.x as u16 * 6 + 3, pos.y as u16 * 2 + 1),).unwrap();
            }
            GameAction::MakeMove(pos) => {
                execute!(
                    stdout(),
                    //MoveTo(pos.x as u16 * 6 + 3, pos.y as u16 * 2 + 1),
                    SetForegroundColor(player.move_color),
                    Print(player.move_char),
                    SetForegroundColor(Color::Reset),
                )
                .unwrap();

                game.board[pos.x][pos.y] = player.move_num;
                return Some(pos);
            }
            GameAction::ResetGame => return None,
            GameAction::ExitGame => return None,
            GameAction::NoAction => {}
        }
    }
}

pub fn online_host() {
    print!("Input board size: ");
    stdout().flush().unwrap();

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let size = buffer.trim().parse::<usize>().unwrap();

    print!("Input host ip: ");
    stdout().flush().unwrap();

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let addr = buffer.trim();

    print!("Input connection port: ");
    stdout().flush().unwrap();

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let port = buffer.trim();

    let server = TcpListener::bind(format!("{}:{}", addr, port));
    if let Err(e) = &server {
        eprintln!("Could not create the server: {}", e);
        eprintln!("Press any button to continue. . .");
        if let Event::Key(_) = read().unwrap() {
            return;
        }
    }

    println!("\nWaiting for the client to connect. . .");
    match server.unwrap().accept() {
        Ok((mut stream, addr)) => {
            stream.write(&[size as u8]).unwrap();

            let mut game = Game::new(size);
            let mut pos = Position::new(0, 0);

            execute!(
                stdout(),
                Clear(ClearType::All),
                crossterm::cursor::MoveTo(0, 0),
                SetForegroundColor(Color::Yellow),
                Print(&game.board_string),
                SetForegroundColor(Color::Reset),
                cursor::Show,
            )
            .unwrap();

            loop {
                take_turn(&mut stream, &mut game, CROSS, pos);
                pos = match wait_turn(&mut stream, &mut game, CIRCLE) {
                    Some(pos) => pos,
                    None => break,
                }
            }
        }
        Err(_e) => {}
    }
}

pub fn online_client() {
    print!("Input server ip: ");
    stdout().flush().unwrap();

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let addr = lookup_host(buffer.trim()).unwrap();

    print!("Input server port: ");
    stdout().flush().unwrap();

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let port = buffer.trim();

    println!("{}:{}", addr[0], port);
    let stream = TcpStream::connect(format!("{}:{}", addr[0], port));

    if let Err(e) = &stream {
        println!("Could not connect to the server: {}", e);
        eprintln!("Press any button to continue. . .");
        if let Event::Key(_) = read().unwrap() {
            return;
        }
    }

    let mut stream = stream.unwrap();
    let mut buf = [0u8; 1];
    stream.read(&mut buf).unwrap();

    let mut game = Game::new(buf[0] as usize);
    //let mut pos = Position::new(0, 0);

    execute!(
        stdout(),
        Clear(ClearType::All),
        crossterm::cursor::MoveTo(0, 0),
        SetForegroundColor(Color::Yellow),
        Print(&game.board_string),
        SetForegroundColor(Color::Reset),
        cursor::Show,
    )
    .unwrap();

    loop {
        let pos = match wait_turn(&mut stream, &mut game, CROSS) {
            Some(pos) => pos,
            None => break,
        };
        take_turn(&mut stream, &mut game, CIRCLE, pos);
    }
}
