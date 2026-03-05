use sdl2::pixels::Color;
use sdl2::rect::Rect;

use super::RenderContext;
use super::WidgetBase;

#[derive(Debug)]
pub struct CheckBoxWidget {
    pub base: WidgetBase,
    pub checked: bool,
}

impl CheckBoxWidget {
    pub fn draw(&self, ctx: &mut RenderContext) {
        if self.base.hidden {
            return;
        }

        let r = self.base.rect;
        let box_size = r.height().min(20);

        // Draw checkbox border
        let box_rect = Rect::new(r.x(), r.y(), box_size, box_size);
        ctx.canvas.set_draw_color(Color::RGB(180, 180, 180));
        let _ = ctx.canvas.draw_rect(box_rect);

        // Draw check mark if checked
        if self.checked {
            let inner = Rect::new(r.x() + 3, r.y() + 3, box_size - 6, box_size - 6);
            ctx.canvas.set_draw_color(Color::RGB(0, 200, 0));
            let _ = ctx.canvas.fill_rect(inner);
        }
    }

    pub fn handle_click(&mut self, x: i32, y: i32) -> bool {
        if self.base.disabled || self.base.hidden {
            return false;
        }
        if self.base.is_over(x, y) {
            self.checked = !self.checked;
            return true;
        }
        false
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }
}
