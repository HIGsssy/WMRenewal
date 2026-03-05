use sdl2::pixels::Color;

use super::RenderContext;
use super::WidgetBase;

#[derive(Debug)]
pub struct TextItemWidget {
    pub base: WidgetBase,
    pub text: String,
    pub font_size: u16,
    pub scroll_offset: i32,
    pub total_height: i32,
}

impl TextItemWidget {
    pub fn draw(&self, ctx: &mut RenderContext) {
        if self.base.hidden {
            return;
        }

        ctx.fonts.render_multiline(
            ctx.canvas,
            ctx.texture_creator,
            &self.text,
            self.base.rect,
            self.font_size,
            Color::RGB(255, 255, 255),
            self.scroll_offset,
        );
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.scroll_offset = 0;
    }

    pub fn scroll(&mut self, delta: i32) {
        self.scroll_offset = (self.scroll_offset + delta).max(0);
        let max_scroll = (self.total_height - self.base.rect.height() as i32).max(0);
        self.scroll_offset = self.scroll_offset.min(max_scroll);
    }
}
