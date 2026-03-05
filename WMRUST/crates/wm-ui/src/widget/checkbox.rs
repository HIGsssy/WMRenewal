use super::WidgetBase;

#[derive(Debug)]
pub struct CheckBoxWidget {
    pub base: WidgetBase,
    pub checked: bool,
}

impl CheckBoxWidget {
    pub fn draw(&self) {
        todo!()
    }

    pub fn handle_click(&mut self, _x: i32, _y: i32) -> bool {
        todo!()
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }
}
