use super::traits::{Block, Borders, Draw, MouseBackend, Paragraph, Rect, Terminal, Widget};

pub struct FilterBox {
    input: String,
}

impl FilterBox {
    pub fn new() -> FilterBox {
        FilterBox {
            input: String::from(""),
        }
    }

    pub fn push(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn pop(&mut self) {
        self.input.pop();
    }
}

impl Draw for FilterBox {
    fn draw(self, t: &mut Terminal<MouseBackend>, layout: &Rect) {
        Paragraph::default()
            .block(Block::default().title("Filter").borders(Borders::ALL))
            .wrap(true)
            .text(&self.input)
            .render(t, layout)
    }
}
