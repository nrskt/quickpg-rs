use std::fmt;

use super::traits::{Block, Borders, Draw, MouseBackend, Paragraph, Rect, Terminal, Widget};

pub enum Mode {
    Normal,
    Filter,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::Normal => write!(f, "{}", "Normal"),
            Mode::Filter => write!(f, "{}", "Filter"),
        }
    }
}

pub struct StatusBar {
    pub status: Mode,
}

impl StatusBar {
    pub fn new() -> StatusBar {
        StatusBar {
            status: Mode::Normal,
        }
    }

    pub fn toggled_filter(&mut self) {
        match self.status {
            Mode::Filter => self.status = Mode::Normal,
            Mode::Normal => self.status = Mode::Filter,
        }
    }
}

impl Draw for StatusBar {
    fn draw(self, t: &mut Terminal<MouseBackend>, layout: &Rect) {
        Paragraph::default()
            .block(Block::default().title("Status").borders(Borders::ALL))
            .wrap(true)
            .text(&format!("Mode: {}  F4: Filter", &self.status))
            .render(t, layout)
    }
}
