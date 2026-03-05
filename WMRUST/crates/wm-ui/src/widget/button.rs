use sdl2::rect::Rect;

use super::RenderContext;
use super::WidgetBase;

#[derive(Debug)]
pub struct ButtonWidget {
    pub base: WidgetBase,
    pub image_off: String,
    pub image_on: String,
    pub image_disabled: String,
    pub transparency: bool,
    pub scale: bool,
    pub pressed: bool,
}

impl ButtonWidget {
    pub fn draw(&self, ctx: &mut RenderContext) {
        if self.base.hidden {
            return;
        }

        // Choose which image to show based on state
        let image_name = if self.base.disabled {
            &self.image_disabled
        } else if self.base.is_over(ctx.mouse_x, ctx.mouse_y) || self.pressed {
            &self.image_on
        } else {
            &self.image_off
        };

        let path = ctx.resources_path.join("Buttons").join(image_name);
        if let Ok(texture) = ctx.textures.load(ctx.texture_creator, &path) {
            let dst = if self.scale {
                self.base.rect
            } else {
                // Use original image size, positioned at widget location
                let query = texture.query();
                Rect::new(
                    self.base.rect.x(),
                    self.base.rect.y(),
                    query.width.min(self.base.rect.width()),
                    query.height.min(self.base.rect.height()),
                )
            };
            let _ = ctx.canvas.copy(texture, None, Some(dst));
        }
    }

    pub fn handle_click(&mut self, x: i32, y: i32) -> bool {
        if self.base.disabled || self.base.hidden {
            return false;
        }
        self.base.is_over(x, y)
    }

    pub fn is_over(&self, x: i32, y: i32) -> bool {
        self.base.is_over(x, y)
    }
}
