use super::traits::{
    Block, Borders, Color, Draw, Modifier, MouseBackend, Rect, SelectableList, Style, Terminal,
    Widget,
};

pub struct PgTableList {
    displayed_tables: Vec<String>,
    selected: usize,
}

impl PgTableList {
    pub fn new() -> PgTableList {
        PgTableList {
            displayed_tables: vec![
                String::from("public.Test1"),
                String::from("public.Test2"),
                String::from("public.Test3"),
            ],
            selected: 0,
        }
    }

    pub fn displayed_tables(&self) -> &Vec<String> {
        &self.displayed_tables
    }

    pub fn selected(&self) -> &usize {
        &self.selected
    }

    pub fn next(&mut self) {
        self.selected += 1;
        if self.selected > self.displayed_tables.len() - 1 {
            self.selected = 0;
        }
    }

    pub fn prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.displayed_tables.len() - 1;
        }
    }
}

impl Draw for PgTableList {
    fn draw(&self, t: &mut Terminal<MouseBackend>, layout: &Rect) {
        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("Tables"))
            .items(self.displayed_tables())
            .select(*self.selected())
            .style(default_style())
            .highlight_style(highlight_style())
            .highlight_symbol(">")
            .render(t, layout);
    }
}

fn default_style() -> Style {
    Style::default().fg(Color::White).bg(Color::Black)
}

fn highlight_style() -> Style {
    default_style()
        .clone()
        .fg(Color::LightGreen)
        .modifier(Modifier::Bold)
}
