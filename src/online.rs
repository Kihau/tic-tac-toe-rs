use std::{
    io::{stdin, stdout, Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

use crossterm::{self, cursor::MoveTo, event::*, execute, style::*, terminal::*, *};
use dns_lookup::lookup_host;

use crate::game::*;

// Starting sides should be switched after every game
// Add a button to concede (ex: Ecp -> "Are you sure you want to surrender?" )
fn start_game(stream: &mut TcpStream, size: usize, mut turn: bool) {
    let mut game = Game::new(size);
    let mut pos = Position::new(0usize, 0usize);

    execute!(
        stdout(),
        Clear(ClearType::All),
        crossterm::cursor::MoveTo(0, 0),
        SetForegroundColor(Color::Yellow),
        Print(&game.board_string),
        SetForegroundColor(Color::Reset),
        MoveTo(3, 1),
        cursor::Show,
    )
    .unwrap();

    let mut current;
    loop {
        let action = if turn {
            let action = game.do_action(&pos);
            // Handle server shutdown
            stream.write_all(&action.send_data()[..]).unwrap();
            current = CROSS;
            action
        } else {
            let mut buffer = [0u8; 3];
            stream.read_exact(&mut buffer).unwrap();
            current = CIRCLE;
            GameAction::retrieve_data(&buffer)
        };

        match action {
            GameAction::ChangePos(new_pos) => {
                pos = new_pos;
                execute!(stdout(), MoveTo(pos.x as u16 * 6 + 3, pos.y as u16 * 2 + 1)).unwrap();
            }
            GameAction::MakeMove(pos) => {
                execute!(
                    stdout(),
                    SetForegroundColor(current.move_color),
                    Print(current.move_char),
                    SetForegroundColor(Color::Reset),
                )
                .unwrap();

                game.board[pos.x][pos.y] = current.move_num;
                turn = !turn;
            }
            GameAction::ResetGame => break,
            GameAction::ExitGame => break,
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
            stream.write_all(&[size as u8]).unwrap();
            println!("Connection established with: {}", addr);
            start_game(&mut stream, size, true);
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
    stream.read_exact(&mut buf).unwrap();
    start_game(&mut stream, buf[0] as usize, false);
}
