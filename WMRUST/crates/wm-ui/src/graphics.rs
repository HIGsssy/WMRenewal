use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::Sdl;

/// Core SDL2 graphics context. Manages window, canvas, and texture creation.
pub struct Graphics {
    pub sdl_context: Sdl,
    pub canvas: WindowCanvas,
    pub texture_creator: TextureCreator<WindowContext>,
}

impl std::fmt::Debug for Graphics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Graphics").finish_non_exhaustive()
    }
}

/// Error type for graphics operations.
#[derive(Debug, thiserror::Error)]
pub enum GraphicsError {
    #[error("SDL initialization error: {0}")]
    SdlInit(String),
    #[error("Window creation error: {0}")]
    WindowCreation(String),
    #[error("Canvas creation error: {0}")]
    CanvasCreation(String),
    #[error("Texture error: {0}")]
    Texture(String),
}

impl Graphics {
    /// Create a new Graphics context with an SDL2 window.
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, GraphicsError> {
        let sdl_context = sdl2::init().map_err(GraphicsError::SdlInit)?;
        let video_subsystem = sdl_context.video().map_err(GraphicsError::SdlInit)?;

        // Initialize SDL2_ttf (hold context to keep it alive)
        let _ttf = sdl2::ttf::init().map_err(|e| GraphicsError::SdlInit(e.to_string()))?;

        // Initialize SDL2_image for PNG/JPG
        sdl2::image::init(sdl2::image::InitFlag::PNG | sdl2::image::InitFlag::JPG)
            .map_err(|e| GraphicsError::SdlInit(e.to_string()))?;

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| GraphicsError::WindowCreation(e.to_string()))?;

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| GraphicsError::CanvasCreation(e.to_string()))?;

        let texture_creator = canvas.texture_creator();

        Ok(Self {
            sdl_context,
            canvas,
            texture_creator,
        })
    }

    /// Clear the screen to black at the start of a frame.
    pub fn begin_frame(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    /// Present the rendered frame.
    pub fn end_frame(&mut self) {
        self.canvas.present();
    }

    /// Draw a filled rectangle.
    pub fn draw_rect(&mut self, rect: Rect, color: Color) {
        self.canvas.set_draw_color(color);
        let _ = self.canvas.fill_rect(rect);
    }

    /// Draw a texture at the given destination rectangle.
    pub fn draw_texture(&mut self, texture: &Texture, dst: Rect) {
        let _ = self.canvas.copy(texture, None, Some(dst));
    }

    /// Draw a texture with a source rectangle (for sprite sheets).
    pub fn draw_texture_src(&mut self, texture: &Texture, src: Rect, dst: Rect) {
        let _ = self.canvas.copy(texture, Some(src), Some(dst));
    }

    /// Draw a rectangle outline.
    pub fn draw_rect_outline(&mut self, rect: Rect, color: Color) {
        self.canvas.set_draw_color(color);
        let _ = self.canvas.draw_rect(rect);
    }
}
