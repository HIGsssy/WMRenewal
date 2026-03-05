use std::path::Path;

use serde::Deserialize;

use crate::widget::button::ButtonWidget;
use crate::widget::image_item::ImageItemWidget;
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
    #[serde(rename = "@ShowHeaders", default)]
    pub show_headers: String,
    #[serde(rename = "@HeaderDividers", default)]
    pub header_dividers: String,
    #[serde(rename = "@HeaderClicksSort", default)]
    pub header_clicks_sort: String,
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
        };
        widgets.add(&text.name, Widget::TextItem(w));
    }

    // Add buttons
    for btn in screen.buttons {
        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, &btn.name, btn.x, btn.y, btn.width, btn.height);
        let transparency = btn.transparency.to_lowercase() == "true";
        let scale = btn.scale.to_lowercase() == "true";
        let w = ButtonWidget {
            base,
            image_off: format!("{}.png", btn.image),
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

    // ListBoxes, CheckBoxes, Sliders — stubs for now; builder will fill in
    // We intentionally skip complex widget creation here and leave it for the builder phase.

    Ok(())
}
