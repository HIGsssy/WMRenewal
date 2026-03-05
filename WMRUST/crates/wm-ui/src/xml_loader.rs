use std::path::Path;

use serde::Deserialize;

use crate::widget::button::ButtonWidget;
use crate::widget::checkbox::CheckBoxWidget;
use crate::widget::editbox::EditBoxWidget;
use crate::widget::image_item::ImageItemWidget;
use crate::widget::listbox::{ColumnDef, ListBoxWidget};
use crate::widget::slider::SliderWidget;
use crate::widget::text_item::TextItemWidget;
use crate::widget::{Widget, WidgetBase, WidgetStore};

/// Error type for XML screen loading.
#[derive(Debug, thiserror::Error)]
pub enum XmlLoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("XML parse error: {0}")]
    Xml(#[from] quick_xml::DeError),
}

// -- XML structures matching Resources/Interface/*.xml --

/// Individual screen element — captures all child elements in XML document order.
#[derive(Debug, Deserialize)]
pub enum ScreenElement {
    Window(WindowXml),
    Text(TextXml),
    Button(ButtonXml),
    Image(ImageXml),
    ListBox(ListBoxXml),
    CheckBox(CheckBoxXml),
    Checkbox(CheckBoxXml),
    EditBox(EditBoxXml),
    Slider(SliderXml),
    Define(DefineXml),
    Widget(WidgetRefXml),
}

#[derive(Debug, Deserialize)]
pub struct ScreenXml {
    /// All child elements in XML document order, preserving draw-order.
    #[serde(rename = "$value", default)]
    pub elements: Vec<ScreenElement>,
}

#[derive(Debug, Deserialize)]
pub struct WindowXml {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@XPos", default)]
    pub x: i32,
    #[serde(rename = "@YPos", default)]
    pub y: i32,
    #[serde(rename = "@Width", default)]
    pub width: u32,
    #[serde(rename = "@Height", default)]
    pub height: u32,
    #[serde(rename = "@Border", default)]
    pub border: i32,
}

#[derive(Debug, Deserialize)]
pub struct TextXml {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Text", default)]
    pub text: String,
    #[serde(rename = "@XPos", default)]
    pub x: i32,
    #[serde(rename = "@YPos", default)]
    pub y: i32,
    #[serde(rename = "@Width", default)]
    pub width: u32,
    #[serde(rename = "@Height", default)]
    pub height: u32,
    #[serde(rename = "@FontSize", default)]
    pub font_size: u16,
    #[serde(rename = "@Hidden", default)]
    pub hidden: String,
}

#[derive(Debug, Deserialize)]
pub struct ButtonXml {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Image", default)]
    pub image: String,
    #[serde(rename = "@XPos", default)]
    pub x: i32,
    #[serde(rename = "@YPos", default)]
    pub y: i32,
    #[serde(rename = "@Width", default)]
    pub width: u32,
    #[serde(rename = "@Height", default)]
    pub height: u32,
    #[serde(rename = "@Transparency", default)]
    pub transparency: String,
    #[serde(rename = "@Scale", default)]
    pub scale: String,
    #[serde(rename = "@Disabled", default)]
    pub disabled: String,
    #[serde(rename = "@Hidden", default)]
    pub hidden: String,
    #[serde(rename = "@Cache", default)]
    pub cache: String,
}

#[derive(Debug, Deserialize)]
pub struct ImageXml {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@File", default)]
    pub file: String,
    #[serde(rename = "@XPos", default)]
    pub x: i32,
    #[serde(rename = "@YPos", default)]
    pub y: i32,
    #[serde(rename = "@Width", default)]
    pub width: u32,
    #[serde(rename = "@Height", default)]
    pub height: u32,
    #[serde(rename = "@Hidden", default)]
    pub hidden: String,
}

#[derive(Debug, Deserialize)]
pub struct ListBoxXml {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@XPos", default)]
    pub x: i32,
    #[serde(rename = "@YPos", default)]
    pub y: i32,
    #[serde(rename = "@Width", default)]
    pub width: u32,
    #[serde(rename = "@Height", default)]
    pub height: u32,
    #[serde(rename = "@Border", default)]
    pub border: i32,
    #[serde(rename = "@Events", default)]
    pub events: String,
    #[serde(rename = "@MultiSelect", default)]
    pub multi_select: String,
    #[serde(rename = "@Multi", default)]
    pub multi: String,
    #[serde(rename = "@ShowHeaders", default)]
    pub show_headers: String,
    #[serde(rename = "@HeaderDividers", default)]
    pub header_dividers: String,
    #[serde(rename = "@HeaderDiv", default)]
    pub header_div: String,
    #[serde(rename = "@HeaderClicksSort", default)]
    pub header_clicks_sort: String,
    #[serde(rename = "Column", default)]
    pub columns: Vec<ColumnXml>,
}

#[derive(Debug, Deserialize)]
pub struct ColumnXml {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Header", default)]
    pub header: String,
    #[serde(rename = "@Offset", default)]
    pub offset: i32,
    #[serde(rename = "@Skip", default)]
    pub skip: String,
}

#[derive(Debug, Deserialize)]
pub struct EditBoxXml {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@XPos", default)]
    pub x: i32,
    #[serde(rename = "@YPos", default)]
    pub y: i32,
    #[serde(rename = "@Width", default)]
    pub width: u32,
    #[serde(rename = "@Height", default)]
    pub height: u32,
    #[serde(rename = "@MaxLength", default)]
    pub max_length: usize,
}

#[derive(Debug, Deserialize)]
pub struct CheckBoxXml {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@XPos", default)]
    pub x: i32,
    #[serde(rename = "@YPos", default)]
    pub y: i32,
    #[serde(rename = "@Width", default)]
    pub width: u32,
    #[serde(rename = "@Height", default)]
    pub height: u32,
    #[serde(rename = "@FontSize", default)]
    pub font_size: u16,
    #[serde(rename = "@Text", default)]
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct SliderXml {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@XPos", default)]
    pub x: i32,
    #[serde(rename = "@YPos", default)]
    pub y: i32,
    #[serde(rename = "@Width", default)]
    pub width: u32,
    #[serde(rename = "@Min", default)]
    pub min: i32,
    #[serde(rename = "@Max", default)]
    pub max: i32,
    #[serde(rename = "@MinValue", default)]
    pub min_value: i32,
    #[serde(rename = "@MaxValue", default)]
    pub max_value: i32,
    #[serde(rename = "@Value", default)]
    pub value: i32,
    #[serde(rename = "@Increment", default)]
    pub increment: i32,
    #[serde(rename = "@Disabled", default)]
    pub disabled: String,
    #[serde(rename = "@Hidden", default)]
    pub hidden: String,
    #[serde(rename = "@LiveUpdate", default)]
    pub live_update: String,
}

/// Ignored: <Define> elements define reusable widget templates in the original engine.
#[derive(Debug, Deserialize)]
pub struct DefineXml {
    #[serde(rename = "@Widget", default)]
    pub widget: String,
    #[serde(rename = "$value", default)]
    pub children: Vec<serde::de::IgnoredAny>,
}

/// Ignored: <Widget> elements reference a <Define> template.
#[derive(Debug, Deserialize)]
pub struct WidgetRefXml {
    #[serde(rename = "@Definition", default)]
    pub definition: String,
    #[serde(rename = "@XPos", default)]
    pub x: i32,
    #[serde(rename = "@YPos", default)]
    pub y: i32,
    #[serde(rename = "@Seq", default)]
    pub seq: String,
    #[serde(rename = "@Sequence", default)]
    pub sequence: String,
}

fn is_hidden(val: &str) -> bool {
    matches!(val.to_lowercase().as_str(), "true" | "1")
}

/// Load a screen definition from an XML file and populate a WidgetStore.
/// Logs errors to stderr if loading fails.
pub fn load_screen_xml(path: &Path, widgets: &mut WidgetStore) -> Result<(), XmlLoadError> {
    let xml_str = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[UI] Failed to read screen XML {:?}: {}", path, e);
            return Err(e.into());
        }
    };
    let screen: ScreenXml = match quick_xml::de::from_str(&xml_str) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[UI] Failed to parse screen XML {:?}: {}", path, e);
            return Err(e.into());
        }
    };

    // Process all elements in XML document order to preserve draw/z-order.
    for element in screen.elements {
        match element {
            ScreenElement::Window(_) => {
                // Window defines screen bounds; not rendered as a widget.
            }
            ScreenElement::Define(_) | ScreenElement::Widget(_) => {
                // Define/Widget template system not yet implemented.
            }
            ScreenElement::Text(text) => {
                let id = widgets.allocate_id();
                let mut base =
                    WidgetBase::new(id, &text.name, text.x, text.y, text.width, text.height);
                base.hidden = is_hidden(&text.hidden);
                let w = TextItemWidget {
                    base,
                    text: text.text,
                    font_size: if text.font_size > 0 {
                        text.font_size
                    } else {
                        12
                    },
                    scroll_offset: 0,
                    total_height: 0,
                };
                widgets.add(&text.name, Widget::TextItem(w));
            }
            ScreenElement::Button(btn) => {
                let id = widgets.allocate_id();
                let mut base =
                    WidgetBase::new(id, &btn.name, btn.x, btn.y, btn.width, btn.height);
                base.hidden = is_hidden(&btn.hidden);
                let transparency = btn.transparency.to_lowercase() == "true";
                let scale = btn.scale.to_lowercase() == "true";
                let w = ButtonWidget {
                    base,
                    image_off: format!("{}Off.png", btn.image),
                    image_on: format!("{}On.png", btn.image),
                    image_disabled: format!("{}Disabled.png", btn.image),
                    transparency,
                    scale,
                    pressed: false,
                };
                widgets.add(&btn.name, Widget::Button(w));
            }
            ScreenElement::Image(img) => {
                let id = widgets.allocate_id();
                let mut base =
                    WidgetBase::new(id, &img.name, img.x, img.y, img.width, img.height);
                base.hidden = is_hidden(&img.hidden);
                let w = ImageItemWidget {
                    base,
                    file: img.file,
                };
                widgets.add(&img.name, Widget::ImageItem(w));
            }
            ScreenElement::ListBox(lb) => {
                let id = widgets.allocate_id();
                let base = WidgetBase::new(id, &lb.name, lb.x, lb.y, lb.width, lb.height);
                let columns: Vec<ColumnDef> = lb
                    .columns
                    .iter()
                    .map(|c| ColumnDef {
                        name: c.name.clone(),
                        header: c.header.clone(),
                        offset: c.offset,
                        skip: c.skip.to_lowercase() == "true",
                    })
                    .collect();
                let is_multi = lb.multi_select.to_lowercase() == "true"
                    || lb.multi.to_lowercase() == "true";
                let has_headers = lb.show_headers.to_lowercase() == "true";
                let has_dividers = lb.header_dividers.to_lowercase() == "true"
                    || lb.header_div.to_lowercase() == "true";
                let w = ListBoxWidget {
                    base,
                    items: Vec::new(),
                    columns,
                    multi_select: is_multi,
                    show_headers: has_headers,
                    header_dividers: has_dividers,
                    header_clicks_sort: lb.header_clicks_sort.to_lowercase() == "true",
                    scroll_position: 0,
                    sorted_column: String::new(),
                    sorted_descending: false,
                    border_size: lb.border,
                    element_height: 18,
                };
                widgets.add(&lb.name, Widget::ListBox(w));
            }
            ScreenElement::CheckBox(cb) | ScreenElement::Checkbox(cb) => {
                let id = widgets.allocate_id();
                let base = WidgetBase::new(id, &cb.name, cb.x, cb.y, cb.width, cb.height);
                let w = CheckBoxWidget {
                    base,
                    checked: false,
                };
                widgets.add(&cb.name, Widget::CheckBox(w));
            }
            ScreenElement::Slider(sl) => {
                let id = widgets.allocate_id();
                let mut base = WidgetBase::new(id, &sl.name, sl.x, sl.y, sl.width, 20);
                base.hidden = is_hidden(&sl.hidden);
                let min = if sl.min_value != 0 {
                    sl.min_value
                } else {
                    sl.min
                };
                let max = if sl.max_value != 0 {
                    sl.max_value
                } else {
                    sl.max
                };
                let w = SliderWidget {
                    base,
                    min_val: min,
                    max_val: if max > min { max } else { 100 },
                    value: sl.value,
                    increment: if sl.increment > 0 { sl.increment } else { 1 },
                    live_update: sl.live_update.to_lowercase() == "true",
                    dragging: false,
                };
                widgets.add(&sl.name, Widget::Slider(w));
            }
            ScreenElement::EditBox(eb) => {
                let id = widgets.allocate_id();
                let base = WidgetBase::new(id, &eb.name, eb.x, eb.y, eb.width, eb.height);
                let w = EditBoxWidget {
                    base,
                    text: String::new(),
                    max_length: if eb.max_length > 0 {
                        eb.max_length
                    } else {
                        256
                    },
                    focused: false,
                };
                widgets.add(&eb.name, Widget::EditBox(w));
            }
        }
    }

    Ok(())
}
