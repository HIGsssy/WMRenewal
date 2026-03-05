use super::WidgetBase;

#[derive(Debug)]
pub struct EditBoxWidget {
    pub base: WidgetBase,
    pub text: String,
    pub max_length: usize,
    pub focused: bool,
}

impl EditBoxWidget {
    pub fn draw(&self) {
        todo!()
    }

    pub fn handle_key(&mut self, _key: char) {
        todo!()
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}
