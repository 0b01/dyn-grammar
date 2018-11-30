//!
//! Save WackDonald's from bankrupcy by putting your compiler construction skills to good use.
//!
//! The famouse burger chain is entering the catering space. Help rewrite the recipes with clear and concise grammar.

#[macro_use]
mod grammar;
mod burger;
mod animation;

extern crate ggez;

use ggez::conf;
use ggez::timer;
use ggez::event::{self, MouseButton, MouseState};
use ggez::graphics;
use ggez::{Context, GameResult};
use std::env;
use std::path;
use self::animation::Animation;

struct MainState {
    image1: graphics::Image,
    animation: Animation,
    pos_x: i32,
    pos_y: i32,
    mouse_down: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> MainState {
        let mut image1 = graphics::Image::new(ctx, "/lettuce.png").unwrap();
        image1.set_filter(graphics::FilterMode::Nearest);
        let animation = Animation::new(ctx, "/cut bread.png", 96, 2.);
        MainState {
            image1,
            animation,
            pos_x: 100,
            pos_y: 100,
            mouse_down: false,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        self.animation.update(ctx);

        // let dst = graphics::Point2::new(self.pos_x as f32, self.pos_y as f32);
        // let scale = graphics::Point2::new(5., 5.);
        // graphics::draw_ex(ctx, &self.image1, DrawParam {
        //     dest: dst,
        //     rotation: 0.,
        //     scale,
        //     ..Default::default()
        // })?;
        self.animation.draw(ctx, self.pos_x as f32, self.pos_y as f32);

        graphics::present(ctx);
        timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        self.mouse_down = true;
        self.animation.play();
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        self.mouse_down = false;
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _state: MouseState,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    ) {
        if self.mouse_down {
            self.pos_x = x;
            self.pos_y = y;
        }
    }
}

pub fn main() {
    let c = conf::Conf {
        window_setup: conf::WindowSetup {
            title: "Game".to_owned(),
            resizable: false,
            allow_highdpi: true,
            samples: conf::NumSamples::One,
            ..Default::default()
        },
        ..Default::default()
    };
    let mut ctx = &mut Context::load_from_conf("input_test", "ggez", c).unwrap();

    // We add the CARGO_MANIFEST_DIR/resources to the filesystem paths so
    // that ggez will look in our cargo project directory for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(&mut ctx);
    event::run(ctx, state).unwrap();
}