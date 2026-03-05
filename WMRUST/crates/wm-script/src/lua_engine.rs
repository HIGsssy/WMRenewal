use std::sync::{Arc, Mutex};

use mlua::Lua;

use crate::api::{self, ScriptContext, SharedContext};

/// Wraps the Lua VM and provides the game scripting environment.
pub struct LuaEngine {
    lua: Lua,
    ctx: SharedContext,
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
        let ctx = Arc::new(Mutex::new(ScriptContext::new()));
        api::register_api(&lua, ctx.clone())?;
        Ok(Self { lua, ctx })
    }

    /// Get a reference to the shared script context.
    pub fn context(&self) -> &SharedContext {
        &self.ctx
    }

    /// Reset transient context state before running a new script.
    pub fn reset_context(&self) {
        let mut ctx = self.ctx.lock().unwrap();
        ctx.reset_transient();
    }

    /// Execute a Lua script from a file path.
    pub fn exec_file(&self, path: &std::path::Path) -> Result<(), LuaEngineError> {
        let code = std::fs::read_to_string(path)?;
        self.exec_str(&code)
    }

    /// Execute a Lua string.
    pub fn exec_str(&self, code: &str) -> Result<(), LuaEngineError> {
        self.lua.load(code).exec()?;
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = LuaEngine::new().unwrap();
        // Verify the wm global is registered
        engine.exec_str("assert(wm ~= nil)").unwrap();
        engine.exec_str("assert(wm.message ~= nil)").unwrap();
        engine.exec_str("assert(wm.girl ~= nil)").unwrap();
        engine.exec_str("assert(wm.player ~= nil)").unwrap();
        engine.exec_str("assert(wm.global ~= nil)").unwrap();
        engine.exec_str("assert(wm.dungeon ~= nil)").unwrap();
    }

    #[test]
    fn test_message_api() {
        let engine = LuaEngine::new().unwrap();
        engine
            .exec_str(r#"wm.message("Hello world", 0)"#)
            .unwrap();
        engine
            .exec_str(r#"wm.message("Second message", 1)"#)
            .unwrap();

        let ctx = engine.context().lock().unwrap();
        assert_eq!(ctx.messages.len(), 2);
        assert_eq!(ctx.messages[0].text, "Hello world");
        assert_eq!(ctx.messages[0].color, 0);
        assert_eq!(ctx.messages[1].text, "Second message");
        assert_eq!(ctx.messages[1].color, 1);
    }

    #[test]
    fn test_girl_stat_api() {
        let engine = LuaEngine::new().unwrap();

        // Set initial stat value
        {
            let mut ctx = engine.context().lock().unwrap();
            ctx.girl_stats[1] = 50; // Happiness = 50
        }

        // Use stat API from Lua
        engine
            .exec_str(
                r#"
            local h = wm.girl.get_stat("Happiness")
            wm.girl.set_stat("Happiness", 10) -- delta +10
            wm.message("Happiness was " .. h, 0)
        "#,
            )
            .unwrap();

        let ctx = engine.context().lock().unwrap();
        assert_eq!(ctx.girl_stats[1], 60); // 50 + 10
        assert_eq!(ctx.messages[0].text, "Happiness was 50");
    }

    #[test]
    fn test_girl_trait_api() {
        let engine = LuaEngine::new().unwrap();

        engine
            .exec_str(
                r#"
            wm.girl.add_trait("Elegant")
            local has = wm.girl.has_trait("Elegant")
            wm.girl.remove_trait("Elegant")
            local has_after = wm.girl.has_trait("Elegant")
            wm.message(tostring(has) .. " " .. tostring(has_after), 0)
        "#,
            )
            .unwrap();

        let ctx = engine.context().lock().unwrap();
        assert_eq!(ctx.messages[0].text, "true false");
    }

    #[test]
    fn test_global_flag_api() {
        let engine = LuaEngine::new().unwrap();

        engine
            .exec_str(
                r#"
            wm.global.set_flag(0, true)
            local f = wm.global.get_flag(0)
            wm.message(tostring(f), 0)
        "#,
            )
            .unwrap();

        let ctx = engine.context().lock().unwrap();
        assert!(ctx.global_flags[0]);
        assert_eq!(ctx.messages[0].text, "true");
    }

    #[test]
    fn test_player_api() {
        let engine = LuaEngine::new().unwrap();
        engine.exec_str("wm.player.set_suspicion(5)").unwrap();
        engine.exec_str("wm.player.set_disposition(-3)").unwrap();

        let ctx = engine.context().lock().unwrap();
        assert_eq!(ctx.suspicion_delta, 5);
        assert_eq!(ctx.disposition_delta, -3);
    }

    #[test]
    fn test_game_over() {
        let engine = LuaEngine::new().unwrap();
        engine.exec_str("wm.game_over()").unwrap();
        let ctx = engine.context().lock().unwrap();
        assert!(ctx.game_over);
    }

    #[test]
    fn test_exec_intro_lua() {
        let intro_path = wm_core::resources_path().join("Scripts").join("Intro.lua");
        if !intro_path.exists() {
            eprintln!("Skipping: Intro.lua not found");
            return;
        }

        let engine = LuaEngine::new().unwrap();
        engine.exec_file(&intro_path).unwrap();

        let ctx = engine.context().lock().unwrap();
        assert!(
            !ctx.messages.is_empty(),
            "Intro.lua should produce messages"
        );
    }

    #[test]
    fn test_reset_context() {
        let engine = LuaEngine::new().unwrap();
        engine
            .exec_str(r#"wm.message("test", 0)"#)
            .unwrap();
        assert_eq!(engine.context().lock().unwrap().messages.len(), 1);

        engine.reset_context();
        assert_eq!(engine.context().lock().unwrap().messages.len(), 0);
    }
}

