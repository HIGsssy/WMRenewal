use super::WidgetBase;

#[derive(Debug)]
pub struct ScrollBarWidget {
    pub base: WidgetBase,
    pub position: i32,
    pub max_position: i32,
    pub page_size: i32,
    pub dragging: bool,
}

impl ScrollBarWidget {
    pub fn draw(&self) {
        todo!()
    }

    pub fn handle_click(&mut self, _x: i32, _y: i32) -> bool {
        todo!()
    }

    pub fn set_range(&mut self, max: i32, page_size: i32) {
        self.max_position = max;
        self.page_size = page_size;
    }

    pub fn scroll_to(&mut self, position: i32) {
        self.position = position.clamp(0, self.max_position);
    }
}
