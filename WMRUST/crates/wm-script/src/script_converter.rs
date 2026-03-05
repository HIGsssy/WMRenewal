use std::path::Path;

/// Convert a legacy binary .script file to Lua source.
pub fn convert_script_to_lua(_path: &Path) -> Result<String, ConvertError> {
    todo!()
}

#[derive(Debug, thiserror::Error)]
pub enum ConvertError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unknown opcode: {0}")]
    UnknownOpcode(u8),
}
