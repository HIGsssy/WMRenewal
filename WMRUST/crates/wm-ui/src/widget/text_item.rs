use super::WidgetBase;

#[derive(Debug)]
pub struct TextItemWidget {
    pub base: WidgetBase,
    pub text: String,
    pub font_size: u16,
}

impl TextItemWidget {
    pub fn draw(&self) {
        todo!()
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}
