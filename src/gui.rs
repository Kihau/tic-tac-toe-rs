use crossterm::{
    execute,
    style::*,
    terminal::{Clear, ClearType},
};
use std::io;

/* --- Gui Characters ---
    │ ─ └ ┐ ┘ ┌

    Example menu list:
    ┌THE GAME────┐
    │> Item 1    │
    │  Item 2    │
    │  Item 3    │
    └────────────┘

    Example input field:
    ┌INPUT BOARD SIZE────┐
    │  SIZE: 123         │
    └────────────────────┘
*/

// pub struct InputField {
//     pub text: String,
//     pub frame: String,
// }

pub struct MenuList {
    pub items: Vec<String>,
    pub frame: String,
    pub current: usize,
}

impl MenuList {
    pub fn new(items: Vec<String>, title: String, mut width: usize) -> Self {
        let mut frame = String::new();

        // Check if table width is correct
        if width < title.chars().count() + 2 {
            width = title.chars().count() + 2
        }

        for i in items.iter() {
            if width < i.chars().count() + 5 {
                width = i.chars().count() + 5
            }
        }

        // Generate and add top panels
        let mut top = format!("┌{}", title);
        for _ in 0..width - title.chars().count() - 2 {
            top.push('─');
        }
        top.push_str("┐\n");
        frame.push_str(&top);

        // Generate and add middle panels
        let mut middle = String::from("│");
        for _ in 0..width - 2 {
            middle.push(' ');
        }
        middle.push_str("│\n");

        for _ in items.iter() {
            frame.push_str(&middle);
        }

        // Generate and add bottom panels
        let mut bottom = String::from("└");
        for _ in 0..width - 2 {
            bottom.push('─');
        }
        bottom.push_str("┘\n");
        frame.push_str(&bottom);

        Self {
            items,
            frame,
            current: 0,
        }
    }

    pub fn generate_list(&self) {
        execute!(
            io::stdout(),
            crossterm::cursor::Hide,
            Clear(ClearType::All),
            crossterm::cursor::MoveTo(0, 0),
            Print(&self.frame),
        )
        .unwrap();

        for (i, item) in self.items.iter().enumerate() {
            if i == self.current {
                execute!(
                    io::stdout(),
                    crossterm::cursor::MoveTo(1, i as u16 + 1),
                    SetForegroundColor(Color::Yellow),
                    Print(format!("> {}", &item)),
                    SetForegroundColor(Color::Reset),
                )
                .unwrap();
            } else {
                execute!(
                    io::stdout(),
                    crossterm::cursor::MoveTo(1, i as u16 + 1),
                    Print(format!("  {}", &item)),
                )
                .unwrap();
            }
        }
    }

    pub fn update_list(&mut self, current: i32) {
        let current = if current >= self.items.len() as i32 {
            self.items.len() - 1
        } else if current < 0 {
            0usize
        } else {
            current as usize
        };

        if current == self.current {
            return;
        }

        execute!(
            io::stdout(),
            crossterm::cursor::MoveTo(1, self.current as u16 + 1),
            Print(format!("  {}", &self.items[self.current])),
        )
        .unwrap();

        execute!(
            io::stdout(),
            crossterm::cursor::MoveTo(1, current as u16 + 1),
            SetForegroundColor(Color::Yellow),
            Print(format!("> {}", self.items[current])),
            SetForegroundColor(Color::Reset),
        )
        .unwrap();

        self.current = current;
    }
}
