use serde::{Deserialize, Serialize};

/// A room/facility in a brothel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub name: String,
    pub desc: String,
    pub space: i32,
    pub provides: i32,
    pub price: i32,
    pub glitz: i32,
    pub min_glitz: i32,
    pub max_glitz: i32,
    pub functions: Vec<RoomFunction>,
}

/// A function/capability provided by a room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomFunction {
    pub name: String,
    pub factor: Option<f32>,
    pub success: Option<f32>,
}

// -- XML deserialization structs --

#[derive(Debug, Deserialize)]
pub struct FacilitiesXml {
    #[serde(rename = "Facility", default)]
    pub facilities: Vec<FacilityXml>,
}

#[derive(Debug, Deserialize)]
pub struct FacilityXml {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Desc", default)]
    pub desc: String,
    #[serde(rename = "@Space", default)]
    pub space: i32,
    #[serde(rename = "@Provides", default)]
    pub provides: i32,
    #[serde(rename = "@Price", default)]
    pub price: i32,
    #[serde(rename = "@Glitz", default)]
    pub glitz: i32,
    #[serde(rename = "@MinGlitz", default)]
    pub min_glitz: i32,
    #[serde(rename = "@MaxGlitz", default)]
    pub max_glitz: i32,
    #[serde(rename = "Function", default)]
    pub functions: Vec<FunctionXml>,
}

#[derive(Debug, Deserialize)]
pub struct FunctionXml {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Factor", default)]
    pub factor: Option<String>,
    #[serde(rename = "@Success", default)]
    pub success: Option<String>,
}

fn parse_opt_f32_percent(s: &Option<String>) -> Option<f32> {
    s.as_deref().map(|v| {
        let stripped = v.trim_end_matches('%');
        stripped.parse::<f32>().unwrap_or(0.0)
    })
}

impl FacilityXml {
    /// Convert from XML representation to domain Room.
    pub fn into_room(self) -> Room {
        let functions = self
            .functions
            .into_iter()
            .map(|f| RoomFunction {
                name: f.name,
                factor: parse_opt_f32_percent(&f.factor),
                success: parse_opt_f32_percent(&f.success),
            })
            .collect();

        Room {
            name: self.name,
            desc: self.desc,
            space: self.space,
            provides: self.provides,
            price: self.price,
            glitz: self.glitz,
            min_glitz: self.min_glitz,
            max_glitz: self.max_glitz,
            functions,
        }
    }
}
