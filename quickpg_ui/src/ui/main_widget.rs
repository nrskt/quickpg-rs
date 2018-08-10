use super::data_view::DataView;
use super::filter_box::FilterBox;
use super::pg_table_list::PgTableList;
use super::status_bar::StatusBar;
use super::traits::{Direction, Draw, Group, MouseBackend, Size, Terminal};

pub struct UiContext {
    pub filter_box: FilterBox,
    pub pg_table_list: PgTableList,
    pub status_bar: StatusBar,
    pub data_view: DataView,
}

impl UiContext {
    pub fn new() -> UiContext {
        UiContext {
            filter_box: FilterBox::new(),
            pg_table_list: PgTableList::new(),
            status_bar: StatusBar::new(),
            data_view: DataView::new(),
        }
    }

    pub fn draw_main(self, t: &mut Terminal<MouseBackend>) {
        let size = t.size().unwrap();

        Group::default()
            .direction(Direction::Vertical)
            .margin(0)
            .sizes(&[Size::Percent(10), Size::Percent(85), Size::Percent(5)])
            .render(t, &size, |t, chunks| {
                &self.filter_box.draw(t, &chunks[0]);
                Group::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .sizes(&[Size::Percent(20), Size::Percent(80)])
                    .render(t, &chunks[1], |t, chunks| {
                        &self.pg_table_list.draw(t, &chunks[0]);
                        &self.data_view.draw(t, &chunks[1]);
                    });
                &self.status_bar.draw(t, &chunks[2]);
            });
        t.draw().unwrap()
    }
}
