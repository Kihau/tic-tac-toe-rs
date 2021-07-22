use std::{
    io::{stdin, stdout, Read, Write},
    net::{TcpListener, TcpStream},
};

use crossterm::{
    event::{read, Event},
    execute,
    style::*,
    terminal::*,
};
use dns_lookup::lookup_host;

use crate::game;

// Starting sides should be switched after every game
// Add a button to concede (ex: Ecp -> "Are you sure you want to surrender?" )

pub fn online_host() {
    // Input the board size
    // Establish connection
    // Send information about board size to the client

    // Create game and print a board

    // --- Repeat until game is over ---
    // Wait for the opponent move
    // Check if game is over
    // Make a move

    // Ask for rematch
    // Play another game / Exit to menu
    print!("Input board size: ");
    stdout().flush().unwrap();

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let size = buffer.trim().parse::<usize>().unwrap();

    print!("Input connection port: ");
    stdout().flush().unwrap();

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let port = buffer.trim();

    let server = TcpListener::bind(format!("192.168.1.32:{}", port));
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

            let mut game = game::Game::new(size);
            let pm1 = game::CROSS;
            let pm2 = game::CIRCLE;
            let mut pos = game::Position::new(0, 0);

            execute!(
                stdout(),
                Clear(ClearType::All),
                crossterm::cursor::MoveTo(0, 0),
                SetForegroundColor(Color::Yellow),
                Print(&game.board_string),
                SetForegroundColor(Color::Reset),
            )
            .unwrap();

            loop {
                pos = game.make_move(pos).unwrap();
                stream.write(&[pos.x as u8, pos.y as u8]).unwrap();
                game.board[pos.x][pos.y] = pm1.move_num;
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(pos.x as u16 * 6 + 3, pos.y as u16 * 2 + 1),
                    SetForegroundColor(pm1.move_color),
                    Print(pm1.move_char),
                    SetForegroundColor(Color::Reset),
                    crossterm::cursor::Hide,
                )
                .unwrap();

                let mut buf = [0u8; 2];
                stream.read(&mut buf).unwrap();
                pos.x = buf[0] as usize;
                pos.y = buf[1] as usize;
                game.board[pos.x][pos.y] = pm2.move_num;
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(pos.x as u16 * 6 + 3, pos.y as u16 * 2 + 1),
                    SetForegroundColor(pm2.move_color),
                    Print(pm2.move_char),
                    SetForegroundColor(Color::Reset),
                    crossterm::cursor::Show,
                )
                .unwrap();
            }
        }
        Err(_e) => {}
    }
}

pub fn online_client() {
    // Input ip adress and establish connection
    // Recieve information about the board size

    // Create game and print a board

    // --- Repeat until game is over ---
    // Make a move
    // Check if game is over
    // Wait for the opponent move

    // Ask for rematch
    // Play another game / Exit to menu

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

    let mut game = game::Game::new(buf[0] as usize);
    let pm1 = game::CROSS;
    let pm2 = game::CIRCLE;
    let mut pos = game::Position::new(0, 0);

    execute!(
        stdout(),
        Clear(ClearType::All),
        crossterm::cursor::MoveTo(0, 0),
        SetForegroundColor(Color::Yellow),
        Print(&game.board_string),
        SetForegroundColor(Color::Reset),
        crossterm::cursor::Hide,
    )
    .unwrap();

    loop {
        let mut buf = [0u8; 2];
        stream.read(&mut buf).unwrap();
        pos.x = buf[0] as usize;
        pos.y = buf[1] as usize;
        game.board[pos.x][pos.y] = pm1.move_num;
        execute!(
            stdout(),
            crossterm::cursor::MoveTo(pos.x as u16 * 6 + 3, pos.y as u16 * 2 + 1),
            SetForegroundColor(pm1.move_color),
            Print(pm1.move_char),
            SetForegroundColor(Color::Reset),
            crossterm::cursor::Show,
        )
        .unwrap();

        pos = game.make_move(pos).unwrap();
        stream.write(&[pos.x as u8, pos.y as u8]).unwrap();
        game.board[pos.x][pos.y] = pm2.move_num;
        execute!(
            stdout(),
            crossterm::cursor::MoveTo(pos.x as u16 * 6 + 3, pos.y as u16 * 2 + 1),
            SetForegroundColor(pm2.move_color),
            Print(pm2.move_char),
            SetForegroundColor(Color::Reset),
            crossterm::cursor::Hide,
        )
        .unwrap();
    }
}
