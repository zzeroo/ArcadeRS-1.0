use phi::{Phi, View, ViewAction};
use phi::data::Rectangle;
use std::path::Path;
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureQuery};
use sdl2::image::LoadTexture;

/// Pixels traveled by the player's ship every second, when it is moving.
const PLAYER_SPEED: f64 = 180.0;

const SHIP_W: f64 = 43.0;
const SHIP_H: f64 = 39.0;


struct Ship {
    rect: Rectangle,
    tex: Texture,
}


pub struct ShipView {
    player: Ship,
}

impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        //? Load the texture from the filesystem.
        //? If it cannot be found, then there is no point in continuing: panic!
        let tex = phi.renderer.load_texture(Path::new("assets/spaceship.png")).unwrap();

        //? Destructure some properties of the texture, notably width and
        //? height, which will used for the ship's bounding box.
        let TextureQuery { width, height, .. } = tex.query();

        ShipView {
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: SHIP_W,
                    h: SHIP_H,
                },
                tex: tex,
            }
        }
    }
}

impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }


        // Move the player's ship

        let diagonal =
            (phi.events.key_up ^ phi.events.key_down) &&
            (phi.events.key_left ^ phi.events.key_right);

        let moved =
            if diagonal { 1.0 / 2.0f64.sqrt() }
            else { 1.0 } * PLAYER_SPEED * elapsed;

        let dx = match (phi.events.key_left, phi.events.key_right) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        let dy = match (phi.events.key_up, phi.events.key_down) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        self.player.rect.x += dx;
        self.player.rect.y += dy;

        // The movable region spans the entire height of the window and 70% of its
        // width. This way, the player cannot get to the far right of the screen, where
        // we will spawn the asteroids, and get immediately eliminated.
        //
        // We restrain the width because most screens are wider than they are high.
        let movable_region = Rectangle {
            x: 0.0,
            y: 0.0,
            w: phi.output_size().0 * 0.70,
            h: phi.output_size().1,
        };

        // If the player cannot fit in the screen, then there is a problem and
        // the game should be promptly aborted.
        self.player.rect = self.player.rect.move_inside(movable_region).unwrap();


        // Clear the scene
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
        phi.renderer.fill_rect(self.player.rect.to_sdl());

        // Render the ship
        //? The texture to render is `self.player.tex` (we borrow it mutably)
        phi.renderer.copy(&mut self.player.tex,
            //? The "source region" of the image. Here, we take the entire image, from
            //? the top left corner (0,0) to the bottom-right one (rect.w, rect.h).
            Some(Rectangle {
                x: 43.0 * 0.0,
                y: 39.0 * 1.0,
                w: self.player.rect.w,
                h: self.player.rect.h,
            }.to_sdl()),
            //? The destination of the image. We simply provide the bouning box, the
            //? renderer takes care of the rest.
            Some(self.player.rect.to_sdl()));

        ViewAction::None
    }
}
