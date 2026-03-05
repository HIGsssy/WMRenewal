pub mod button;
pub mod checkbox;
pub mod editbox;
pub mod image_item;
pub mod listbox;
pub mod scrollbar;
pub mod slider;
pub mod text_item;

use std::collections::HashMap;
use std::path::Path;

use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use crate::font::FontCache;
use crate::texture_cache::TextureCache;

pub type WidgetId = u32;

/// Rendering context passed to widget draw methods.
pub struct RenderContext<'a> {
    pub canvas: &'a mut WindowCanvas,
    pub textures: &'a mut TextureCache,
    pub fonts: &'a mut FontCache,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub resources_path: &'a Path,
    pub mouse_x: i32,
    pub mouse_y: i32,
}

/// Union of all widget types.
#[derive(Debug)]
pub enum Widget {
    Button(button::ButtonWidget),
    TextItem(text_item::TextItemWidget),
    ListBox(listbox::ListBoxWidget),
    EditBox(editbox::EditBoxWidget),
    CheckBox(checkbox::CheckBoxWidget),
    Slider(slider::SliderWidget),
    ScrollBar(scrollbar::ScrollBarWidget),
    ImageItem(image_item::ImageItemWidget),
}

impl Widget {
    /// Draw this widget using the provided render context.
    pub fn draw(&self, ctx: &mut RenderContext) {
        match self {
            Widget::Button(w) => w.draw(ctx),
            Widget::TextItem(w) => w.draw(ctx),
            Widget::ListBox(w) => w.draw(ctx),
            Widget::EditBox(w) => w.draw(ctx),
            Widget::CheckBox(w) => w.draw(ctx),
            Widget::Slider(w) => w.draw(ctx),
            Widget::ScrollBar(w) => w.draw(ctx),
            Widget::ImageItem(w) => w.draw(ctx),
        }
    }

    /// Get the base properties of this widget.
    pub fn base(&self) -> &WidgetBase {
        match self {
            Widget::Button(w) => &w.base,
            Widget::TextItem(w) => &w.base,
            Widget::ListBox(w) => &w.base,
            Widget::EditBox(w) => &w.base,
            Widget::CheckBox(w) => &w.base,
            Widget::Slider(w) => &w.base,
            Widget::ScrollBar(w) => &w.base,
            Widget::ImageItem(w) => &w.base,
        }
    }
}

/// Base properties shared by all widgets.
#[derive(Debug, Clone)]
pub struct WidgetBase {
    pub id: WidgetId,
    pub name: String,
    pub rect: Rect,
    pub hidden: bool,
    pub disabled: bool,
}

impl WidgetBase {
    pub fn new(id: WidgetId, name: &str, x: i32, y: i32, w: u32, h: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            rect: Rect::new(x, y, w, h),
            hidden: false,
            disabled: false,
        }
    }

    pub fn is_over(&self, x: i32, y: i32) -> bool {
        !self.hidden && !self.disabled && self.rect.contains_point((x, y))
    }
}

/// Store/registry for all widgets on the current screen.
#[derive(Debug)]
pub struct WidgetStore {
    widgets: HashMap<WidgetId, Widget>,
    name_to_id: HashMap<String, WidgetId>,
    next_id: WidgetId,
    draw_order: Vec<WidgetId>,
}

impl WidgetStore {
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            name_to_id: HashMap::new(),
            next_id: 1,
            draw_order: Vec::new(),
        }
    }

    pub fn allocate_id(&mut self) -> WidgetId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn add(&mut self, name: &str, widget: Widget) -> WidgetId {
        let id = self.allocate_id();
        self.name_to_id.insert(name.to_string(), id);
        self.draw_order.push(id);
        self.widgets.insert(id, widget);
        id
    }

    pub fn get(&self, id: WidgetId) -> Option<&Widget> {
        self.widgets.get(&id)
    }

    pub fn get_mut(&mut self, id: WidgetId) -> Option<&mut Widget> {
        self.widgets.get_mut(&id)
    }

    pub fn get_id(&self, name: &str) -> Option<WidgetId> {
        self.name_to_id.get(name).copied()
    }

    pub fn clear(&mut self) {
        self.widgets.clear();
        self.name_to_id.clear();
        self.draw_order.clear();
        self.next_id = 1;
    }

    /// Draw all widgets in insertion order.
    pub fn draw_all(&self, ctx: &mut RenderContext) {
        for &id in &self.draw_order {
            if let Some(widget) = self.widgets.get(&id) {
                if !widget.base().hidden {
                    widget.draw(ctx);
                }
            }
        }
    }

    /// Returns an iterator over all widget IDs in draw order.
    pub fn iter_ids(&self) -> impl Iterator<Item = WidgetId> + '_ {
        self.draw_order.iter().copied()
    }
}

impl Default for WidgetStore {
    fn default() -> Self {
        Self::new()
    }
}
