use crossterm::{
    event::{read, Event, KeyCode},
    execute,
};

use std::io::stdout;

mod online;
mod offline;
mod gui;
mod game;

// Menu -> Input options: [Online (Host / Connect)] / [Offline (AI / PvP)]
// Establish Tcp connection - host starts first
fn main() {
    let items = vec![
        "Online: Host a Game".to_string(),
        "Online: Connect to a Game".to_string(),
        "Offline: Two Players".to_string(),
        "Offline: Player vs MinMax".to_string(),
        "Offline: Player vs NeuralNetwork".to_string(),
        "This item does nothing".to_string(),
        "Exit The Program".to_string(),
    ];

    let mut list = gui::MenuList::new(items, "Game Mode Menu".to_string(), 26);

    let calls = vec![
        online::online_host as fn(),
        online::online_client as fn(),
        offline::offline_player as fn(),
        offline::offline_minmax as fn(),
        offline::offline_neuralnetwork as fn(),
        || println!("lol"),
    ];

    list.generate_list();
    execute!(stdout(), crossterm::cursor::MoveTo(0, 0)).unwrap();
    loop {
        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Up | KeyCode::Char('w') => list.update_list(list.current as i32 - 1),
                KeyCode::Down | KeyCode::Char('s') => list.update_list(list.current as i32 + 1),
                KeyCode::Enter if list.current == list.items.len() - 1 => break,
                KeyCode::Enter => {
                    calls[list.current]();
                    list.generate_list()
                }
                _ => {}
            }
        }
    }
    execute!(
        stdout(),
        crossterm::cursor::MoveTo(0, list.items.len() as u16 + 3)
    )
    .unwrap();
}

