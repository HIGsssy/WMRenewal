use super::WidgetBase;

/// A list item in a ListBox.
#[derive(Debug, Clone)]
pub struct ListItem {
    pub id: i32,
    pub columns: Vec<String>,
    pub selected: bool,
    pub color: ListColor,
}

/// Pre-defined row background colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListColor {
    Blue,
    Red,
    DarkBlue,
}

/// Column definition for a multi-column listbox.
#[derive(Debug, Clone)]
pub struct ColumnDef {
    pub name: String,
    pub header: String,
    pub offset: i32,
    pub skip: bool,
}

#[derive(Debug)]
pub struct ListBoxWidget {
    pub base: WidgetBase,
    pub items: Vec<ListItem>,
    pub columns: Vec<ColumnDef>,
    pub multi_select: bool,
    pub show_headers: bool,
    pub header_dividers: bool,
    pub header_clicks_sort: bool,
    pub scroll_position: i32,
    pub sorted_column: String,
    pub sorted_descending: bool,
    pub border_size: i32,
    pub element_height: i32,
}

impl ListBoxWidget {
    pub fn draw(&self) {
        todo!()
    }

    pub fn handle_click(&mut self, _x: i32, _y: i32) {
        todo!()
    }

    pub fn add_element(&mut self, _id: i32, _data: &str) {
        todo!()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn get_selected(&self) -> Option<i32> {
        todo!()
    }

    pub fn set_selected(&mut self, _id: i32) {
        todo!()
    }

    pub fn sort_by_column(&mut self, _column: &str, _descending: bool) {
        todo!()
    }

    pub fn scroll_up(&mut self, _amount: i32) {
        todo!()
    }

    pub fn scroll_down(&mut self, _amount: i32) {
        todo!()
    }
}
