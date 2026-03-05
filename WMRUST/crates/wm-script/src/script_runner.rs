//! ScriptRunner — coroutine-based script execution with yield/resume.
//!
//! Scripts can yield at RESTART points (e.g. to show messages or wait
//! for a choice). The caller resumes once the UI has handled the
//! pending messages/choices.

use mlua::prelude::*;

use crate::api::{self, ScriptContext, SharedContext};

/// Status of the script runner after a step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunnerStatus {
    /// Script yielded — there are pending messages/choices to show.
    Yielded,
    /// Script finished executing.
    Done,
}

/// Runs a Lua script as a coroutine, supporting yield/resume.
pub struct ScriptRunner {
    lua: Lua,
    ctx: SharedContext,
    thread_key: LuaRegistryKey,
    status: RunnerStatus,
}

impl std::fmt::Debug for ScriptRunner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScriptRunner")
            .field("status", &self.status)
            .finish_non_exhaustive()
    }
}

impl ScriptRunner {
    /// Create a new runner and start the script as a coroutine.
    ///
    /// The script is loaded and the coroutine is created but NOT yet resumed.
    /// Call `resume()` to begin execution.
    pub fn new(code: &str) -> Result<Self, ScriptRunnerError> {
        let lua = Lua::new();
        let ctx = std::sync::Arc::new(std::sync::Mutex::new(ScriptContext::new()));
        api::register_api(&lua, ctx.clone())?;

        let func = lua.load(code).into_function()?;
        let thread = lua.create_thread(func)?;
        let thread_key = lua.create_registry_value(thread)?;

        Ok(Self {
            lua,
            ctx,
            thread_key,
            status: RunnerStatus::Yielded, // not started yet
        })
    }

    /// Resume the coroutine. Returns the new status.
    ///
    /// Before calling resume, the caller should have handled any pending
    /// messages/choices from the previous yield (and written choice
    /// selections back into the context).
    pub fn resume(&mut self) -> Result<RunnerStatus, ScriptRunnerError> {
        if self.status == RunnerStatus::Done {
            return Ok(RunnerStatus::Done);
        }

        let thread: LuaThread = self.lua.registry_value(&self.thread_key)?;
        match thread.resume::<LuaMultiValue>(()) {
            Ok(_) => {
                // Check if the coroutine is finished
                if thread.status() == LuaThreadStatus::Resumable {
                    self.status = RunnerStatus::Yielded;
                } else {
                    self.status = RunnerStatus::Done;
                }
            }
            Err(e) => {
                eprintln!("[script_runner] Lua error: {}", e);
                self.status = RunnerStatus::Done;
            }
        }
        Ok(self.status.clone())
    }

    /// Get the current runner status.
    pub fn status(&self) -> &RunnerStatus {
        &self.status
    }

    /// Get a reference to the shared script context.
    pub fn context(&self) -> &SharedContext {
        &self.ctx
    }

    /// Check if the script is done.
    pub fn is_done(&self) -> bool {
        self.status == RunnerStatus::Done
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ScriptRunnerError {
    #[error("Lua error: {0}")]
    Lua(#[from] mlua::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_script_runs_to_done() {
        let mut runner = ScriptRunner::new(r#"wm.message("hello", 0)"#).unwrap();
        let status = runner.resume().unwrap();
        assert_eq!(status, RunnerStatus::Done);

        let ctx = runner.context().lock().unwrap();
        assert_eq!(ctx.messages.len(), 1);
        assert_eq!(ctx.messages[0].text, "hello");
    }

    #[test]
    fn test_yield_and_resume() {
        let code = r#"
            wm.message("part 1", 0)
            coroutine.yield("restart")
            wm.message("part 2", 0)
        "#;
        let mut runner = ScriptRunner::new(code).unwrap();

        // First resume: runs until yield
        let status = runner.resume().unwrap();
        assert_eq!(status, RunnerStatus::Yielded);
        {
            let ctx = runner.context().lock().unwrap();
            assert_eq!(ctx.messages.len(), 1);
            assert_eq!(ctx.messages[0].text, "part 1");
        }

        // Second resume: runs to completion
        let status = runner.resume().unwrap();
        assert_eq!(status, RunnerStatus::Done);
        {
            let ctx = runner.context().lock().unwrap();
            assert_eq!(ctx.messages.len(), 2);
            assert_eq!(ctx.messages[1].text, "part 2");
        }
    }

    #[test]
    fn test_choice_box_flow() {
        let code = r#"
            wm.choice_box(1, {"Stay", "Leave"})
            coroutine.yield("restart")
            local c = wm.get_choice(1)
            if c == 0 then
                wm.message("You stayed", 0)
            else
                wm.message("You left", 0)
            end
        "#;
        let mut runner = ScriptRunner::new(code).unwrap();

        // First resume: sets up choice box, yields
        runner.resume().unwrap();
        assert_eq!(runner.status(), &RunnerStatus::Yielded);

        // Simulate UI selecting option 1 (Leave)
        {
            let mut ctx = runner.context().lock().unwrap();
            ctx.choices.insert(1, 1);
        }

        // Second resume: reads choice, produces message
        runner.resume().unwrap();
        assert_eq!(runner.status(), &RunnerStatus::Done);

        let ctx = runner.context().lock().unwrap();
        assert_eq!(ctx.messages.last().unwrap().text, "You left");
    }
}
