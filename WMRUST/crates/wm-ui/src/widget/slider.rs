use super::WidgetBase;

#[derive(Debug)]
pub struct SliderWidget {
    pub base: WidgetBase,
    pub min_val: i32,
    pub max_val: i32,
    pub value: i32,
    pub increment: i32,
    pub live_update: bool,
    pub dragging: bool,
}

impl SliderWidget {
    pub fn draw(&self) {
        todo!()
    }

    pub fn handle_click(&mut self, _x: i32, _y: i32) -> bool {
        todo!()
    }

    pub fn drag_move(&mut self, _x: i32) {
        todo!()
    }

    pub fn end_drag(&mut self) {
        self.dragging = false;
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value.clamp(self.min_val, self.max_val);
    }

    pub fn set_range(&mut self, min: i32, max: i32, value: i32, increment: i32) {
        self.min_val = min;
        self.max_val = max;
        self.increment = increment;
        self.set_value(value);
    }
}
