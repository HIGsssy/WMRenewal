use std::collections::HashMap;
use std::path::{Path, PathBuf};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;

/// Manages loaded TTF fonts and renders text to textures.
pub struct FontCache {
    ttf_context: Sdl2TtfContext,
    default_font_path: PathBuf,
    /// Cache of rendered text lines → textures for the current frame.
    /// Cleared each frame to avoid stale textures.
    line_cache: HashMap<LineCacheKey, Texture>,
}

#[derive(Hash, Eq, PartialEq)]
struct LineCacheKey {
    text: String,
    size: u16,
    r: u8,
    g: u8,
    b: u8,
}

impl std::fmt::Debug for FontCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FontCache")
            .field("default_font", &self.default_font_path)
            .finish()
    }
}

impl FontCache {
    pub fn new(default_font_path: &Path) -> Result<Self, String> {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        Ok(Self {
            ttf_context,
            default_font_path: default_font_path.to_path_buf(),
            line_cache: HashMap::new(),
        })
    }

    /// Clear per-frame text cache.
    pub fn clear_cache(&mut self) {
        self.line_cache.clear();
    }

    /// Render a single line of text directly to the canvas at (x, y).
    #[allow(clippy::too_many_arguments)]
    pub fn render_text(
        &mut self,
        canvas: &mut WindowCanvas,
        texture_creator: &TextureCreator<WindowContext>,
        text: &str,
        x: i32,
        y: i32,
        size: u16,
        color: Color,
    ) {
        if text.is_empty() {
            return;
        }

        let font = match self.ttf_context.load_font(&self.default_font_path, size) {
            Ok(f) => f,
            Err(_) => return,
        };

        let surface = match font.render(text).blended(color) {
            Ok(s) => s,
            Err(_) => return,
        };

        let texture = match texture_creator.create_texture_from_surface(&surface) {
            Ok(t) => t,
            Err(_) => return,
        };

        let query = texture.query();
        let dst = Rect::new(x, y, query.width, query.height);
        let _ = canvas.copy(&texture, None, Some(dst));
    }

    /// Render multi-line text with word wrapping within the given rect.
    /// Returns the total rendered height in pixels.
    #[allow(clippy::too_many_arguments)]
    pub fn render_multiline(
        &mut self,
        canvas: &mut WindowCanvas,
        texture_creator: &TextureCreator<WindowContext>,
        text: &str,
        rect: Rect,
        size: u16,
        color: Color,
        scroll_offset: i32,
    ) -> i32 {
        if text.is_empty() {
            return 0;
        }

        let font = match self.ttf_context.load_font(&self.default_font_path, size) {
            Ok(f) => f,
            Err(_) => return 0,
        };

        let line_skip = font.recommended_line_spacing();
        let max_width = rect.width() as i32 - 10; // 5px padding each side

        // Word-wrap the text into lines
        let lines = word_wrap(text, max_width, &font);

        let total_height = lines.len() as i32 * line_skip;
        let mut cur_y = rect.y() - scroll_offset;

        // Set clip rect to prevent drawing outside the widget
        canvas.set_clip_rect(Some(rect));

        for line in &lines {
            // Skip lines above visible area
            if cur_y + line_skip < rect.y() {
                cur_y += line_skip;
                continue;
            }
            // Stop if below visible area
            if cur_y >= rect.y() + rect.height() as i32 {
                break;
            }

            if !line.is_empty() {
                let surface = match font.render(line).blended(color) {
                    Ok(s) => s,
                    Err(_) => {
                        cur_y += line_skip;
                        continue;
                    }
                };

                let texture = match texture_creator.create_texture_from_surface(&surface) {
                    Ok(t) => t,
                    Err(_) => {
                        cur_y += line_skip;
                        continue;
                    }
                };

                let query = texture.query();
                let dst = Rect::new(rect.x() + 5, cur_y, query.width, query.height);
                let _ = canvas.copy(&texture, None, Some(dst));
            }
            cur_y += line_skip;
        }

        canvas.set_clip_rect(None);
        total_height
    }

    /// Get the line height for a given font size.
    pub fn line_height(&self, size: u16) -> i32 {
        self.ttf_context
            .load_font(&self.default_font_path, size)
            .map(|f| f.recommended_line_spacing())
            .unwrap_or(size as i32 + 4)
    }
}

/// Word-wrap text to fit within max_width pixels using the given font.
fn word_wrap(text: &str, max_width: i32, font: &sdl2::ttf::Font) -> Vec<String> {
    let mut lines = Vec::new();

    for paragraph in text.split('\n') {
        if paragraph.is_empty() {
            lines.push(String::new());
            continue;
        }

        let words: Vec<&str> = paragraph.split_whitespace().collect();
        if words.is_empty() {
            lines.push(String::new());
            continue;
        }

        let mut current_line = String::new();

        for word in words {
            let test = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };

            let (w, _) = font.size_of(&test).unwrap_or((0, 0));

            if (w as i32) > max_width && !current_line.is_empty() {
                lines.push(current_line);
                current_line = word.to_string();
            } else {
                current_line = test;
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}
