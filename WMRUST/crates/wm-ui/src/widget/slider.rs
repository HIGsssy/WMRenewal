use sdl2::pixels::Color;
use sdl2::rect::Rect;

use super::WidgetBase;
use super::RenderContext;

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
    pub fn draw(&self, ctx: &mut RenderContext) {
        if self.base.hidden {
            return;
        }

        let r = self.base.rect;

        // Draw track background
        let track_y = r.y() + r.height() as i32 / 2 - 2;
        let track = Rect::new(r.x(), track_y, r.width(), 4);
        ctx.canvas.set_draw_color(Color::RGB(60, 60, 60));
        let _ = ctx.canvas.fill_rect(track);

        // Draw handle at current value position
        let range = (self.max_val - self.min_val).max(1);
        let frac = (self.value - self.min_val) as f64 / range as f64;
        let handle_x = r.x() + (frac * (r.width() as f64 - 16.0)) as i32;
        let handle = Rect::new(handle_x, r.y(), 16, r.height());

        let handle_color = if self.dragging {
            Color::RGB(200, 200, 255)
        } else {
            Color::RGB(150, 150, 200)
        };
        ctx.canvas.set_draw_color(handle_color);
        let _ = ctx.canvas.fill_rect(handle);
    }

    pub fn handle_click(&mut self, x: i32, y: i32) -> bool {
        if self.base.disabled || self.base.hidden {
            return false;
        }
        if self.base.is_over(x, y) {
            self.dragging = true;
            self.update_value_from_x(x);
            return true;
        }
        false
    }

    pub fn drag_move(&mut self, x: i32) {
        if self.dragging {
            self.update_value_from_x(x);
        }
    }

    fn update_value_from_x(&mut self, x: i32) {
        let r = self.base.rect;
        let rel = (x - r.x()).clamp(0, r.width() as i32) as f64;
        let frac = rel / r.width() as f64;
        let range = self.max_val - self.min_val;
        let raw = self.min_val + (frac * range as f64) as i32;

        // Snap to increment
        if self.increment > 0 {
            let snapped = ((raw - self.min_val + self.increment / 2) / self.increment)
                * self.increment
                + self.min_val;
            self.value = snapped.clamp(self.min_val, self.max_val);
        } else {
            self.value = raw.clamp(self.min_val, self.max_val);
        }
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
