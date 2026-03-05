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

#[derive(Debug, Deserialize)]
pub struct ScreenXml {
    #[serde(rename = "Window", default)]
    pub windows: Vec<WindowXml>,
    #[serde(rename = "Text", default)]
    pub texts: Vec<TextXml>,
    #[serde(rename = "Button", default)]
    pub buttons: Vec<ButtonXml>,
    #[serde(rename = "Image", default)]
    pub images: Vec<ImageXml>,
    #[serde(rename = "ListBox", default)]
    pub listboxes: Vec<ListBoxXml>,
    #[serde(rename = "CheckBox", default)]
    pub checkboxes: Vec<CheckBoxXml>,
    #[serde(rename = "Checkbox", default)]
    pub checkboxes_alt: Vec<CheckBoxXml>,
    #[serde(rename = "EditBox", default)]
    pub editboxes: Vec<EditBoxXml>,
    #[serde(rename = "Slider", default)]
    pub sliders: Vec<SliderXml>,
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
    #[serde(rename = "@Value", default)]
    pub value: i32,
    #[serde(rename = "@Increment", default)]
    pub increment: i32,
}

/// Load a screen definition from an XML file and populate a WidgetStore.
pub fn load_screen_xml(path: &Path, widgets: &mut WidgetStore) -> Result<(), XmlLoadError> {
    let xml_str = std::fs::read_to_string(path)?;
    let screen: ScreenXml = quick_xml::de::from_str(&xml_str)?;

    // Add text items
    for text in screen.texts {
        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, &text.name, text.x, text.y, text.width, text.height);
        let w = TextItemWidget {
            base,
            text: text.text,
            font_size: if text.font_size > 0 { text.font_size } else { 12 },
            scroll_offset: 0,
            total_height: 0,
        };
        widgets.add(&text.name, Widget::TextItem(w));
    }

    // Add buttons — file naming: {Image}Off.png, {Image}On.png, {Image}Disabled.png
    for btn in screen.buttons {
        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, &btn.name, btn.x, btn.y, btn.width, btn.height);
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

    // Add images
    for img in screen.images {
        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, &img.name, img.x, img.y, img.width, img.height);
        let w = ImageItemWidget {
            base,
            file: img.file,
        };
        widgets.add(&img.name, Widget::ImageItem(w));
    }

    // Add list boxes
    for lb in screen.listboxes {
        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, &lb.name, lb.x, lb.y, lb.width, lb.height);
        let columns: Vec<ColumnDef> = lb.columns.iter().map(|c| ColumnDef {
            name: c.name.clone(),
            header: c.header.clone(),
            offset: c.offset,
            skip: c.skip.to_lowercase() == "true",
        }).collect();
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

    // Add check boxes (both <CheckBox> and <Checkbox> variants)
    let all_checkboxes = screen.checkboxes.into_iter().chain(screen.checkboxes_alt);
    for cb in all_checkboxes {
        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, &cb.name, cb.x, cb.y, cb.width, cb.height);
        let w = CheckBoxWidget {
            base,
            checked: false,
        };
        widgets.add(&cb.name, Widget::CheckBox(w));
    }

    // Add sliders
    for sl in screen.sliders {
        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, &sl.name, sl.x, sl.y, sl.width, 20);
        let w = SliderWidget {
            base,
            min_val: sl.min,
            max_val: if sl.max > sl.min { sl.max } else { 100 },
            value: sl.value,
            increment: if sl.increment > 0 { sl.increment } else { 1 },
            live_update: false,
            dragging: false,
        };
        widgets.add(&sl.name, Widget::Slider(w));
    }

    // Add edit boxes
    for eb in screen.editboxes {
        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, &eb.name, eb.x, eb.y, eb.width, eb.height);
        let w = EditBoxWidget {
            base,
            text: String::new(),
            max_length: if eb.max_length > 0 { eb.max_length } else { 256 },
            focused: false,
        };
        widgets.add(&eb.name, Widget::EditBox(w));
    }

    Ok(())
}
