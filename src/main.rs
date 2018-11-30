//! Save WackDonald's from bankrupcy by putting your compiler construction skills to good use.
//!
//! The famouse burger chain is entering the catering space. Help rewrite the recipes with clear and concise grammar.

#[macro_use]
mod grammar;
mod burger;
mod animation;
mod prelude;
use crate::prelude::*;
use self::animation::Animation;

extern crate quicksilver;

struct MainState {
    animation: Asset<Animation>,
    font: Asset<Image>,
    pos_x: f32,
    pos_y: f32,
    // mouse_down: bool,
}


impl State for MainState {
    fn new() -> Result<MainState> {
        let animation = Asset::new(Animation::new("cutbread.png", 96, 1.2));
        let font = Asset::new(Font::load("fonts/CourierPrime.ttf")
            .and_then(|font| {
                let style = FontStyle::new(72.0, Color::BLACK);
                result(font.render("Sample Text", &style))
            }));

        let pos_x = 0.;
        let pos_y = 0.;
        Ok(MainState { font, animation, pos_x, pos_y, })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.animation.execute(|anim| anim.update(window) )
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::MAGENTA)?;
        let pos_x = self.pos_x;
        let pos_y = self.pos_y;
        self.animation.execute(|anim| {
            anim.draw(window, pos_x, pos_y, 3.);
            Ok(())
        })?;

        self.font.execute(|image| {
            window.draw(&image.area().with_center((400, 300)), Img(&image));
            Ok(())
        })?;

        Ok(())

    }

    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        match event {
            Event::MouseButton(
                MouseButton::Left,
                ButtonState::Pressed
            ) =>
                self.animation.execute(|anim|anim.play())?,
            _ => (),
        }
        Ok(())
    }
}

fn main() {
    run::<MainState>("Image Example", Vector::new(800, 600), Settings {
        // icon_path: Some("image.png"), // Set the window icon
        ..Settings::default()
    });
}
