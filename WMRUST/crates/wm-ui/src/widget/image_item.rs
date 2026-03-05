use super::RenderContext;
use super::WidgetBase;

#[derive(Debug)]
pub struct ImageItemWidget {
    pub base: WidgetBase,
    pub file: String,
}

impl ImageItemWidget {
    pub fn draw(&self, ctx: &mut RenderContext) {
        if self.base.hidden || self.file.is_empty() {
            return;
        }

        let path = ctx.resources_path.join("Images").join(&self.file);
        if let Ok(texture) = ctx.textures.load(ctx.texture_creator, &path) {
            let _ = ctx.canvas.copy(texture, None, Some(self.base.rect));
        }
    }
}
