use super::filter_box::FilterBox;
use super::pg_table_list::PgTableList;
use super::traits::{Direction, Draw, Group, MouseBackend, Size, Terminal};

pub struct UiContext {
    pub filter_box: FilterBox,
    pub pg_table_list: PgTableList,
}

impl UiContext {
    pub fn new() -> UiContext {
        UiContext {
            filter_box: FilterBox::new(),
            pg_table_list: PgTableList::new(),
        }
    }

    pub fn draw_main(&self, t: &mut Terminal<MouseBackend>) {
        let size = t.size().unwrap();

        Group::default()
            .direction(Direction::Vertical)
            .margin(1)
            .sizes(&[Size::Percent(50), Size::Percent(50)])
            .render(t, &size, |t, chunks| {
                self.filter_box.draw(t, &chunks[0]);
                self.pg_table_list.draw(t, &chunks[1]);
            });
        t.draw().unwrap()
    }
}
