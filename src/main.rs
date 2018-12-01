//! Save WackDonald's from bankrupcy by putting your compiler construction skills to good use.
//!
//! The famouse burger chain is entering the catering space. Help rewrite the recipes with clear and concise grammar.

#[macro_use]
mod grammar;
mod burger;
mod animation;
mod ingredients;
mod prelude;
use crate::prelude::*;
use self::animation::Animation;
use self::ingredients::Ingredients;

const SCALE: f32 = 3.75;

extern crate quicksilver;

struct MainState {
    animation: Asset<Animation>,
    ingredients: Asset<Ingredients>,
    font: Asset<Image>,
    game_ui: Asset<Image>,
    pos_x: f32,
    pos_y: f32,
    // mouse_down: bool,
}

impl MainState {

    fn draw_ui(&mut self, window: &mut Window) -> Result<()> {
        // draw main bg
        self.game_ui.execute(|image| {
            window.draw(&image.area(), Img(&image));
            Ok(())
        })?;
        self.draw_ingredients(window);
        Ok(())
    }

    fn draw_ingredients(&mut self, window: &mut Window) -> Result<()> {
        let objheight = 40.;
        let objwidth = 100.;
        let n_per_line = 5;
        self.ingredients.execute(
            |ing| {
                let srcs = Ingredients::srcs();
                // draw slices
                for (i, src) in srcs.iter().enumerate() {
                    let img = ing.get(src).unwrap();
                    let x = (i % n_per_line) as f32 * objwidth;
                    let y = (i / n_per_line) as f32 * objheight;
                    window.draw_ex(&
                        Rectangle::new(
                            Vector::new(340. + x, 450. + y ),
                            Vector::new(32., 32.)
                        ),
                        Img(&img),
                        Transform::scale(Vector::new(3., 3.)),
                        100,
                    );
                }
                let bottles = vec!["ketchupbottle", "mayobottle", "bbqbottle" ];
                for (i, src) in bottles.iter().enumerate() {
                    let img = ing.get(src).unwrap();
                    let x = i as f32 * 74.;
                    window.draw_ex(&
                        Rectangle::new(
                            Vector::new(575. + x, 590.),
                            Vector::new(96., 96.)
                        ),
                        Img(&img),
                        Transform::scale(Vector::new(3., 3.)),
                        100,
                    );
                }
                Ok(())
        })?;
        Ok(())
    }
}

impl State for MainState {
    fn new() -> Result<MainState> {
        let animation = Asset::new(Animation::new("cutbread.png", 96, 1.2));
        let font = Asset::new(Font::load("fonts/CourierPrime.ttf")
            .and_then(|font| {
                let style = FontStyle::new(72.0, Color::BLACK);
                result(font.render("Sample Text", &style))
            }));
        let game_ui = Asset::new(Image::load("gameui.png"));
        let ingredients = Asset::new(Ingredients::new());

        let pos_x = 0.;
        let pos_y = 0.;
        Ok(MainState {
            font,
            animation,
            ingredients,
            pos_x,
            pos_y,
            game_ui,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.animation.execute(|anim| anim.update(window) )
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::CYAN)?;
        self.draw_ui(window);


        self.animation.execute(|anim| {
            anim.draw(window, 575., 170., SCALE);
            Ok(())
        })?;

        // self.font.execute(|image| {
        //     window.draw(&image.area().with_center((400, 300)), Img(&image));
        //     Ok(())
        // })?;

        Ok(())

    }

    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        match event {
            Event::MouseButton(
                MouseButton::Left,
                ButtonState::Pressed
            ) =>
                self.animation.execute(|anim|anim.play())?,

            Event::MouseMoved(v) => {
                println!("{:#?}", v);
                self.pos_x = v.x;
                self.pos_y = v.y;
            }

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
