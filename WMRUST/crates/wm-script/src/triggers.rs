use wm_core::enums::TriggerType;

/// A single trigger definition.
#[derive(Debug, Clone)]
pub struct Trigger {
    pub script: String,
    pub trigger_type: TriggerType,
    pub triggered: bool,
    pub chance: u8,
    pub once: bool,
    pub values: [i32; 2],
}

/// Manages loading and evaluating triggers (per-girl and global).
#[derive(Debug)]
pub struct TriggerSystem {
    pub triggers: Vec<Trigger>,
    pub queue: Vec<usize>,
}

impl TriggerSystem {
    pub fn new() -> Self {
        Self {
            triggers: Vec::new(),
            queue: Vec::new(),
        }
    }

    /// Load triggers from a trigger list file.
    pub fn load(&mut self, _path: &std::path::Path) -> Result<(), TriggerError> {
        todo!()
    }

    /// Check all trigger conditions and queue those that fire.
    pub fn evaluate(&mut self) {
        todo!()
    }

    /// Process the next queued trigger.
    pub fn process_next(&mut self) -> Option<&Trigger> {
        todo!()
    }
}

impl Default for TriggerSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TriggerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("XML error: {0}")]
    Xml(#[from] quick_xml::DeError),
}
