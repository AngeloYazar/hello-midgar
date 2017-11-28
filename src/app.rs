use std::rc::Rc;

use cgmath::{self, Vector2};
use midgar::{self, KeyCode, Midgar, Surface};
use midgar::graphics::sprite::{DrawTexture, MagnifySamplerFilter, SpriteDrawParams, SpriteRenderer};
use midgar::graphics::text::{self, Font, TextRenderer};
use midgar::graphics::texture::TextureRegion;

const SCREEN_SIZE: Vector2<f32> = Vector2 {
    x: 640.0,
    y: 400.0,
};

pub struct GameApp<'a> {
    projection: cgmath::Matrix4<f32>,
    background: TextureRegion,
    sprite: SpriteRenderer,
    text_renderer: TextRenderer,
    font: Font<'a>
}

impl<'a> midgar::App for GameApp<'a> {
    fn create(midgar: &Midgar) -> Self {
        let projection = cgmath::ortho(0.0, SCREEN_SIZE.x,
                                            SCREEN_SIZE.y, 0.0,
                                            -1.0, 1.0);

        let background = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/textures/background.png", true));
            TextureRegion::new(texture)
        };

        GameApp {
            projection,            
            background,
            sprite: SpriteRenderer::new(midgar.graphics().display(), projection),
            text_renderer: TextRenderer::new(midgar.graphics().display()),
            font: text::load_font_from_path("assets/fonts/VeraMono.ttf")
        }
    }

    fn step(&mut self, midgar: &mut Midgar) {
        if midgar.input().was_key_pressed(KeyCode::Escape) {
            midgar.set_should_exit();
            return;
        }

        // Render!
        self.sprite.set_projection_matrix(self.projection);

        let mut target = midgar.graphics().display().draw();

        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let draw_params = SpriteDrawParams::new()
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .alpha(true);

        // Draw background
        self.sprite.draw(&self.background.draw(SCREEN_SIZE.x * 0.5, SCREEN_SIZE.y * 0.5),
                         draw_params, &mut target);

        self.text_renderer.draw_text("Hello Midgar!", &self.font, [1.0, 1.0, 1.0],
                                         20, 220.0, 140.0, 300, &self.projection, &mut target);

        target.finish()
            .expect("target.finish() failed");
    }
}