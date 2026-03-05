use sdl2::pixels::Color;
use sdl2::rect::Rect;

use super::RenderContext;
use super::WidgetBase;

/// A list item in a ListBox.
#[derive(Debug, Clone)]
pub struct ListItem {
    pub id: i32,
    pub columns: Vec<String>,
    pub selected: bool,
    pub color: ListColor,
}

/// Pre-defined row background colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListColor {
    Blue,
    Red,
    DarkBlue,
}

impl ListColor {
    pub fn to_sdl_color(self) -> Color {
        match self {
            ListColor::Blue => Color::RGB(49, 49, 134),
            ListColor::Red => Color::RGB(134, 49, 49),
            ListColor::DarkBlue => Color::RGB(29, 29, 94),
        }
    }

    pub fn to_sdl_color_selected(self) -> Color {
        match self {
            ListColor::Blue => Color::RGB(89, 89, 194),
            ListColor::Red => Color::RGB(194, 89, 89),
            ListColor::DarkBlue => Color::RGB(69, 69, 154),
        }
    }
}

/// Column definition for a multi-column listbox.
#[derive(Debug, Clone)]
pub struct ColumnDef {
    pub name: String,
    pub header: String,
    pub offset: i32,
    pub skip: bool,
}

#[derive(Debug)]
pub struct ListBoxWidget {
    pub base: WidgetBase,
    pub items: Vec<ListItem>,
    pub columns: Vec<ColumnDef>,
    pub multi_select: bool,
    pub show_headers: bool,
    pub header_dividers: bool,
    pub header_clicks_sort: bool,
    pub scroll_position: i32,
    pub sorted_column: String,
    pub sorted_descending: bool,
    pub border_size: i32,
    pub element_height: i32,
}

impl ListBoxWidget {
    pub fn draw(&self, ctx: &mut RenderContext) {
        if self.base.hidden {
            return;
        }

        let r = self.base.rect;

        // Draw background
        ctx.canvas.set_draw_color(Color::RGB(20, 20, 35));
        let _ = ctx.canvas.fill_rect(r);

        // Draw border
        if self.border_size > 0 {
            ctx.canvas.set_draw_color(Color::RGB(80, 80, 120));
            let _ = ctx.canvas.draw_rect(r);
        }

        let content_x = r.x() + self.border_size;
        let content_w = (r.width() as i32 - 2 * self.border_size) as u32;
        let mut cur_y = r.y() + self.border_size;

        // Draw column headers
        if self.show_headers && !self.columns.is_empty() {
            let header_rect = Rect::new(content_x, cur_y, content_w, self.element_height as u32);
            ctx.canvas.set_draw_color(Color::RGB(40, 40, 70));
            let _ = ctx.canvas.fill_rect(header_rect);

            for col in &self.columns {
                if col.skip {
                    continue;
                }
                ctx.fonts.render_text(
                    ctx.canvas,
                    ctx.texture_creator,
                    &col.header,
                    content_x + col.offset,
                    cur_y + 2,
                    11,
                    Color::RGB(200, 200, 220),
                );
            }

            if self.header_dividers {
                ctx.canvas.set_draw_color(Color::RGB(80, 80, 120));
                let _ = ctx.canvas.draw_line(
                    (content_x, cur_y + self.element_height),
                    (content_x + content_w as i32, cur_y + self.element_height),
                );
            }

            cur_y += self.element_height;
        }

        // Draw items with clipping
        let clip = Rect::new(
            content_x,
            cur_y,
            content_w,
            (r.y() + r.height() as i32 - cur_y).max(0) as u32,
        );
        ctx.canvas.set_clip_rect(Some(clip));

        let visible_height = clip.height() as i32;
        let start_idx = self.scroll_position;
        let max_items = visible_height / self.element_height.max(1) + 1;

        for i in 0..max_items {
            let idx = (start_idx + i) as usize;
            if idx >= self.items.len() {
                break;
            }

            let item = &self.items[idx];
            let item_y = cur_y + i * self.element_height;

            // Row background
            let row_color = if item.selected {
                item.color.to_sdl_color_selected()
            } else {
                item.color.to_sdl_color()
            };
            let row_rect = Rect::new(content_x, item_y, content_w, self.element_height as u32);
            ctx.canvas.set_draw_color(row_color);
            let _ = ctx.canvas.fill_rect(row_rect);

            // Draw columns
            if self.columns.is_empty() {
                // Single-column mode: draw first value
                if let Some(text) = item.columns.first() {
                    ctx.fonts.render_text(
                        ctx.canvas,
                        ctx.texture_creator,
                        text,
                        content_x + 4,
                        item_y + 2,
                        11,
                        Color::RGB(220, 220, 220),
                    );
                }
            } else {
                for (ci, col) in self.columns.iter().enumerate() {
                    if col.skip {
                        continue;
                    }
                    if let Some(text) = item.columns.get(ci) {
                        ctx.fonts.render_text(
                            ctx.canvas,
                            ctx.texture_creator,
                            text,
                            content_x + col.offset,
                            item_y + 2,
                            11,
                            Color::RGB(220, 220, 220),
                        );
                    }
                }
            }
        }

        ctx.canvas.set_clip_rect(None);
    }

    pub fn handle_click(&mut self, _x: i32, y: i32) {
        if self.base.disabled || self.base.hidden {
            return;
        }

        let r = self.base.rect;
        let header_offset = if self.show_headers {
            self.element_height
        } else {
            0
        };
        let content_y = r.y() + self.border_size + header_offset;
        let rel_y = y - content_y;

        if rel_y < 0 {
            return;
        }

        let clicked_idx = (self.scroll_position + rel_y / self.element_height.max(1)) as usize;
        if clicked_idx >= self.items.len() {
            return;
        }

        if !self.multi_select {
            for item in &mut self.items {
                item.selected = false;
            }
        }

        if let Some(item) = self.items.get_mut(clicked_idx) {
            item.selected = !item.selected;
        }
    }

    pub fn add_element(&mut self, id: i32, data: &str) {
        let columns: Vec<String> = data.split('\t').map(|s| s.to_string()).collect();
        self.items.push(ListItem {
            id,
            columns,
            selected: false,
            color: ListColor::Blue,
        });
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.scroll_position = 0;
    }

    pub fn get_selected(&self) -> Option<i32> {
        self.items.iter().find(|i| i.selected).map(|i| i.id)
    }

    pub fn set_selected(&mut self, id: i32) {
        for item in &mut self.items {
            item.selected = item.id == id;
        }
    }

    pub fn sort_by_column(&mut self, column: &str, descending: bool) {
        let col_idx = self
            .columns
            .iter()
            .position(|c| c.name == column)
            .unwrap_or(0);

        self.items.sort_by(|a, b| {
            let a_val = a.columns.get(col_idx).map(|s| s.as_str()).unwrap_or("");
            let b_val = b.columns.get(col_idx).map(|s| s.as_str()).unwrap_or("");
            let cmp = a_val.cmp(b_val);
            if descending {
                cmp.reverse()
            } else {
                cmp
            }
        });

        self.sorted_column = column.to_string();
        self.sorted_descending = descending;
    }

    pub fn scroll_up(&mut self, amount: i32) {
        self.scroll_position = (self.scroll_position - amount).max(0);
    }

    pub fn scroll_down(&mut self, amount: i32) {
        let max = (self.items.len() as i32 - 1).max(0);
        self.scroll_position = (self.scroll_position + amount).min(max);
    }

    /// Visible item count based on widget height.
    pub fn visible_count(&self) -> i32 {
        let header_offset = if self.show_headers {
            self.element_height
        } else {
            0
        };
        let avail = self.base.rect.height() as i32 - 2 * self.border_size - header_offset;
        avail / self.element_height.max(1)
    }
}
