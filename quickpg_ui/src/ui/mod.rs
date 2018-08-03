pub mod filter_box;
pub mod pg_table_list;
pub mod traits;

use self::filter_box::FilterBox;
use self::pg_table_list::PgTableList;

pub struct UiContext {
    filter_box: FilterBox,
    pg_table_list: PgTableList,
}

impl UiContext {
    pub fn new() -> UiContext {
        UiContext {
            filter_box: FilterBox::new(),
            pg_table_list: PgTableList::new(),
        }
    }

    pub fn filter_box(self) -> FilterBox {
        self.filter_box
    }

    pub fn pg_table_list(self) -> PgTableList {
        self.pg_table_list
    }
}
