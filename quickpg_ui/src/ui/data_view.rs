use std::iter::Iterator;
use std::slice::Iter;

use super::traits::{Block, Color, Draw, MouseBackend, Rect, Row, Style, Table, Terminal, Widget};

pub struct DataView {
    items: Vec<DataItem>,
}

impl DataView {
    pub fn new() -> DataView {
        DataView {
            items: vec![DataItem::new(); 50],
        }
    }

    pub fn rows(self) -> Vec<Vec<String>> {
        self.items.into_iter().map(|item| item.record()).collect()
    }
}

#[derive(Clone)]
pub struct DataItem {
    field_name: String,
    value: String,
}

impl DataItem {
    pub fn new() -> DataItem {
        DataItem {
            field_name: String::from("hogefield"),
            value: String::from("valuevalue"),
        }
    }

    pub fn record(self) -> Vec<String> {
        let field = self.field_name;
        let val = self.value;
        vec![field, val]
    }
}

impl Draw for DataView {
    fn draw(self, t: &mut Terminal<MouseBackend>, layout: &Rect) {
        let row_style = Style::default().fg(Color::White);

        let fields = ["field_name", "value"].into_iter();
        let rows = self.items.into_iter().map(|item| item.record());
        let rows = rows.map(|row| Row::Data(row.into_iter()));
        // let rows: Vec<Row<'_, Iter<'_, &str>, &&str>> =
        //     &self.rows().into_iter().map(|row| Row::Data(row)).collect();
        // let rows = &self
        //     .rows()
        //     .iter()
        //     .map(|row| Row::StyledData(row.into_iter(), &row_style));
        // let rows = &self
        //     .rows()
        //     .iter()
        //     .map(|row| Row::StyledData(row.iter(), &row_style));

        Table::new(fields, rows)
            .block(Block::default().title("Table"))
            .header_style(Style::default().fg(Color::Yellow))
            .widths(&[10, 30])
            .style(Style::default().fg(Color::White))
            .column_spacing(1)
            .render(t, &layout);
    }
}
