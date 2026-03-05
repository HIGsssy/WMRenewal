/// A trait definition parsed from CoreTraits.traits (alternating name/description lines).
#[derive(Debug, Clone)]
pub struct TraitDef {
    pub name: String,
    pub description: String,
}

/// Parse trait definitions from the CoreTraits.traits plain-text format.
/// Format: alternating lines — name, then description, repeating.
pub fn parse_traits(text: &str) -> Vec<TraitDef> {
    let lines: Vec<&str> = text.lines().collect();
    let mut traits = Vec::new();
    let mut i = 0;
    while i + 1 < lines.len() {
        let name = lines[i].trim();
        let desc = lines[i + 1].trim();
        if !name.is_empty() {
            traits.push(TraitDef {
                name: name.to_string(),
                description: desc.to_string(),
            });
        }
        i += 2;
    }
    traits
}
