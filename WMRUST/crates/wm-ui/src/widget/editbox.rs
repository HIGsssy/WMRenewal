use sdl2::pixels::Color;

use super::WidgetBase;
use super::RenderContext;

#[derive(Debug)]
pub struct EditBoxWidget {
    pub base: WidgetBase,
    pub text: String,
    pub max_length: usize,
    pub focused: bool,
}

impl EditBoxWidget {
    pub fn draw(&self, ctx: &mut RenderContext) {
        if self.base.hidden {
            return;
        }

        let r = self.base.rect;

        // Draw background
        let bg_color = if self.focused {
            Color::RGB(40, 40, 60)
        } else {
            Color::RGB(20, 20, 30)
        };
        ctx.canvas.set_draw_color(bg_color);
        let _ = ctx.canvas.fill_rect(r);

        // Draw border
        let border_color = if self.focused {
            Color::RGB(100, 100, 200)
        } else {
            Color::RGB(80, 80, 80)
        };
        ctx.canvas.set_draw_color(border_color);
        let _ = ctx.canvas.draw_rect(r);

        // Draw text
        if !self.text.is_empty() {
            ctx.fonts.render_text(
                ctx.canvas,
                ctx.texture_creator,
                &self.text,
                r.x() + 4,
                r.y() + 2,
                12,
                Color::RGB(255, 255, 255),
            );
        }
    }

    pub fn handle_key(&mut self, key: char) {
        if !self.focused {
            return;
        }
        if key == '\x08' {
            // Backspace
            self.text.pop();
        } else if self.text.len() < self.max_length && !key.is_control() {
            self.text.push(key);
        }
    }

    pub fn handle_click(&mut self, x: i32, y: i32) -> bool {
        self.focused = self.base.is_over(x, y);
        self.focused
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}
