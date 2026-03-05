use serde::Deserialize;

/// Parsed screen layout from an Interface/*.xml file.
#[derive(Debug, Clone)]
pub struct ScreenLayout {
    pub widgets: Vec<WidgetDef>,
}

/// Definition of a single widget from XML.
#[derive(Debug, Clone)]
pub enum WidgetDef {
    Window(WindowDef),
    Button(ButtonDef),
    Text(TextDef),
    Image(ImageDef),
    ListBox(ListBoxDef),
    CheckBox(CheckBoxDef),
    Slider(SliderDef),
    EditBox(EditBoxDef),
}

#[derive(Debug, Clone)]
pub struct WindowDef {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border: i32,
}

#[derive(Debug, Clone)]
pub struct ButtonDef {
    pub name: String,
    pub image: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub transparency: bool,
    pub scale: bool,
    pub disabled: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TextDef {
    pub name: String,
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub font_size: i32,
}

#[derive(Debug, Clone)]
pub struct ImageDef {
    pub name: String,
    pub file: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone)]
pub struct ListBoxDef {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border: i32,
    pub events: bool,
    pub multi: bool,
    pub show_headers: bool,
    pub header_div: bool,
    pub header_clicks_sort: bool,
    pub columns: Vec<ColumnDef>,
}

#[derive(Debug, Clone)]
pub struct ColumnDef {
    pub name: String,
    pub header: String,
    pub offset: i32,
}

#[derive(Debug, Clone)]
pub struct CheckBoxDef {
    pub name: String,
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub font_size: i32,
}

#[derive(Debug, Clone)]
pub struct SliderDef {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub min_value: i32,
    pub max_value: i32,
    pub value: i32,
    pub increment: i32,
    pub live_update: bool,
    pub hidden: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone)]
pub struct EditBoxDef {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

// -- XML deserialization structs --

#[derive(Debug, Deserialize)]
pub struct ScreenXml {
    #[serde(rename = "$value", default)]
    pub elements: Vec<ScreenElement>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ScreenElement {
    Window(WindowXml),
    Button(ButtonXml),
    Text(TextXml),
    Image(ImageXml),
    ListBox(ListBoxXml),
    Checkbox(CheckBoxXml),
    Slider(SliderXml),
    EditBox(EditBoxXml),
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
    pub width: i32,
    #[serde(rename = "@Height", default)]
    pub height: i32,
    #[serde(rename = "@Border", default)]
    pub border: i32,
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
    pub width: i32,
    #[serde(rename = "@Height", default)]
    pub height: i32,
    #[serde(rename = "@Transparency", default)]
    pub transparency: String,
    #[serde(rename = "@Scale", default)]
    pub scale: String,
    #[serde(rename = "@Disabled")]
    pub disabled: Option<String>,
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
    pub width: i32,
    #[serde(rename = "@Height", default)]
    pub height: i32,
    #[serde(rename = "@FontSize", default)]
    pub font_size: i32,
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
    pub width: i32,
    #[serde(rename = "@Height", default)]
    pub height: i32,
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
    pub width: i32,
    #[serde(rename = "@Height", default)]
    pub height: i32,
    #[serde(rename = "@Border", default)]
    pub border: i32,
    #[serde(rename = "@Events", default)]
    pub events: String,
    #[serde(rename = "@Multi", default)]
    pub multi: String,
    #[serde(rename = "@ShowHeaders", default)]
    pub show_headers: String,
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
}

#[derive(Debug, Deserialize)]
pub struct CheckBoxXml {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Text", default)]
    pub text: String,
    #[serde(rename = "@XPos", default)]
    pub x: i32,
    #[serde(rename = "@YPos", default)]
    pub y: i32,
    #[serde(rename = "@Width", default)]
    pub width: i32,
    #[serde(rename = "@Height", default)]
    pub height: i32,
    #[serde(rename = "@FontSize", default)]
    pub font_size: i32,
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
    pub width: i32,
    #[serde(rename = "@MinValue", default)]
    pub min_value: i32,
    #[serde(rename = "@MaxValue", default)]
    pub max_value: i32,
    #[serde(rename = "@Value", default)]
    pub value: i32,
    #[serde(rename = "@Increment", default)]
    pub increment: i32,
    #[serde(rename = "@LiveUpdate", default)]
    pub live_update: String,
    #[serde(rename = "@Hidden", default)]
    pub hidden: String,
    #[serde(rename = "@Disabled", default)]
    pub disabled: String,
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
    pub width: i32,
    #[serde(rename = "@Height", default)]
    pub height: i32,
}

fn parse_bool_str(s: &str) -> bool {
    matches!(s.to_lowercase().as_str(), "true" | "1")
}

impl ScreenXml {
    /// Convert from XML representation to domain ScreenLayout.
    pub fn into_layout(self) -> ScreenLayout {
        let widgets = self
            .elements
            .into_iter()
            .map(|e| match e {
                ScreenElement::Window(w) => WidgetDef::Window(WindowDef {
                    name: w.name,
                    x: w.x,
                    y: w.y,
                    width: w.width,
                    height: w.height,
                    border: w.border,
                }),
                ScreenElement::Button(b) => WidgetDef::Button(ButtonDef {
                    name: b.name,
                    image: b.image,
                    x: b.x,
                    y: b.y,
                    width: b.width,
                    height: b.height,
                    transparency: parse_bool_str(&b.transparency),
                    scale: parse_bool_str(&b.scale),
                    disabled: b.disabled,
                }),
                ScreenElement::Text(t) => WidgetDef::Text(TextDef {
                    name: t.name,
                    text: t.text,
                    x: t.x,
                    y: t.y,
                    width: t.width,
                    height: t.height,
                    font_size: t.font_size,
                }),
                ScreenElement::Image(i) => WidgetDef::Image(ImageDef {
                    name: i.name,
                    file: i.file,
                    x: i.x,
                    y: i.y,
                    width: i.width,
                    height: i.height,
                }),
                ScreenElement::ListBox(l) => WidgetDef::ListBox(ListBoxDef {
                    name: l.name,
                    x: l.x,
                    y: l.y,
                    width: l.width,
                    height: l.height,
                    border: l.border,
                    events: parse_bool_str(&l.events),
                    multi: parse_bool_str(&l.multi),
                    show_headers: parse_bool_str(&l.show_headers),
                    header_div: parse_bool_str(&l.header_div),
                    header_clicks_sort: parse_bool_str(&l.header_clicks_sort),
                    columns: l
                        .columns
                        .into_iter()
                        .map(|c| ColumnDef {
                            name: c.name,
                            header: c.header,
                            offset: c.offset,
                        })
                        .collect(),
                }),
                ScreenElement::Checkbox(c) => WidgetDef::CheckBox(CheckBoxDef {
                    name: c.name,
                    text: c.text,
                    x: c.x,
                    y: c.y,
                    width: c.width,
                    height: c.height,
                    font_size: c.font_size,
                }),
                ScreenElement::Slider(s) => WidgetDef::Slider(SliderDef {
                    name: s.name,
                    x: s.x,
                    y: s.y,
                    width: s.width,
                    min_value: s.min_value,
                    max_value: s.max_value,
                    value: s.value,
                    increment: s.increment,
                    live_update: parse_bool_str(&s.live_update),
                    hidden: parse_bool_str(&s.hidden),
                    disabled: parse_bool_str(&s.disabled),
                }),
                ScreenElement::EditBox(e) => WidgetDef::EditBox(EditBoxDef {
                    name: e.name,
                    x: e.x,
                    y: e.y,
                    width: e.width,
                    height: e.height,
                }),
            })
            .collect();

        ScreenLayout { widgets }
    }
}
