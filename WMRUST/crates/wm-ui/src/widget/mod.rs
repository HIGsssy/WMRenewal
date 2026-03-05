pub mod button;
pub mod checkbox;
pub mod editbox;
pub mod image_item;
pub mod listbox;
pub mod scrollbar;
pub mod slider;
pub mod text_item;

use std::collections::HashMap;

use sdl2::rect::Rect;

pub type WidgetId = u32;

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
        self.rect.contains_point((x, y))
    }
}

/// Store/registry for all widgets on the current screen.
#[derive(Debug)]
pub struct WidgetStore {
    widgets: HashMap<WidgetId, Widget>,
    name_to_id: HashMap<String, WidgetId>,
    next_id: WidgetId,
}

impl WidgetStore {
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            name_to_id: HashMap::new(),
            next_id: 1,
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
        self.next_id = 1;
    }
}

impl Default for WidgetStore {
    fn default() -> Self {
        Self::new()
    }
}
