use super::WidgetBase;

#[derive(Debug)]
pub struct ButtonWidget {
    pub base: WidgetBase,
    pub image_off: String,
    pub image_on: String,
    pub image_disabled: String,
    pub transparency: bool,
    pub scale: bool,
    pub pressed: bool,
}

impl ButtonWidget {
    pub fn draw(&self) {
        todo!()
    }

    pub fn handle_click(&mut self, _x: i32, _y: i32) -> bool {
        todo!()
    }

    pub fn is_over(&self, x: i32, y: i32) -> bool {
        self.base.is_over(x, y)
    }
}
