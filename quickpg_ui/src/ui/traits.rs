pub use tui::backend::{Backend, MouseBackend};
pub use tui::layout::{Direction, Group, Rect, Size};
pub use tui::style::{Color, Modifier, Style};
pub use tui::widgets::{Block, Borders, SelectableList, Widget};
pub use tui::Terminal;

pub trait Draw {
    fn draw(&self, t: &mut Terminal<MouseBackend>, layout: &Rect);
}
