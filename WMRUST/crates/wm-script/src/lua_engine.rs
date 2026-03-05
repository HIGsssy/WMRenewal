use mlua::Lua;

/// Wraps the Lua VM and provides the game scripting environment.
pub struct LuaEngine {
    lua: Lua,
}

impl std::fmt::Debug for LuaEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LuaEngine").finish_non_exhaustive()
    }
}

impl LuaEngine {
    /// Create a new Lua engine with the game API registered.
    pub fn new() -> Result<Self, LuaEngineError> {
        let lua = Lua::new();
        // TODO: register wm.* API functions
        Ok(Self { lua })
    }

    /// Execute a Lua script from a file path.
    pub fn exec_file(&self, _path: &std::path::Path) -> Result<(), LuaEngineError> {
        todo!()
    }

    /// Execute a Lua string.
    pub fn exec_str(&self, _code: &str) -> Result<(), LuaEngineError> {
        todo!()
    }

    /// Get the underlying Lua state (for advanced usage).
    pub fn lua(&self) -> &Lua {
        &self.lua
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LuaEngineError {
    #[error("Lua error: {0}")]
    Lua(#[from] mlua::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}


