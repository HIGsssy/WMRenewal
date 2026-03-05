use sdl2::pixels::Color;
use sdl2::rect::Rect;

use super::RenderContext;
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
    pub fn draw(&self, ctx: &mut RenderContext) {
        if self.base.hidden {
            return;
        }

        let r = self.base.rect;

        // Draw track
        ctx.canvas.set_draw_color(Color::RGB(30, 30, 40));
        let _ = ctx.canvas.fill_rect(r);

        // Draw up arrow area
        let arrow_h = 16u32.min(r.height() / 4);
        let up_rect = Rect::new(r.x(), r.y(), r.width(), arrow_h);
        ctx.canvas.set_draw_color(Color::RGB(60, 60, 80));
        let _ = ctx.canvas.fill_rect(up_rect);

        // Draw down arrow area
        let down_rect = Rect::new(
            r.x(),
            r.y() + r.height() as i32 - arrow_h as i32,
            r.width(),
            arrow_h,
        );
        ctx.canvas.set_draw_color(Color::RGB(60, 60, 80));
        let _ = ctx.canvas.fill_rect(down_rect);

        // Draw thumb
        if self.max_position > 0 {
            let track_height = r.height() as i32 - 2 * arrow_h as i32;
            let thumb_h = ((self.page_size as f64 / (self.max_position + self.page_size) as f64)
                * track_height as f64)
                .max(16.0) as u32;
            let frac = self.position as f64 / self.max_position.max(1) as f64;
            let thumb_y =
                r.y() + arrow_h as i32 + (frac * (track_height - thumb_h as i32) as f64) as i32;

            let thumb_rect = Rect::new(r.x() + 1, thumb_y, r.width() - 2, thumb_h);
            let thumb_color = if self.dragging {
                Color::RGB(160, 160, 200)
            } else {
                Color::RGB(100, 100, 140)
            };
            ctx.canvas.set_draw_color(thumb_color);
            let _ = ctx.canvas.fill_rect(thumb_rect);
        }
    }

    pub fn handle_click(&mut self, x: i32, y: i32) -> bool {
        if self.base.disabled || self.base.hidden {
            return false;
        }
        if !self.base.is_over(x, y) {
            return false;
        }

        let r = self.base.rect;
        let arrow_h = 16i32.min(r.height() as i32 / 4);

        if y < r.y() + arrow_h {
            // Up arrow
            self.scroll_to(self.position - 1);
        } else if y > r.y() + r.height() as i32 - arrow_h {
            // Down arrow
            self.scroll_to(self.position + 1);
        } else {
            self.dragging = true;
        }
        true
    }

    pub fn set_range(&mut self, max: i32, page_size: i32) {
        self.max_position = max;
        self.page_size = page_size;
    }

    pub fn scroll_to(&mut self, position: i32) {
        self.position = position.clamp(0, self.max_position);
    }
}
