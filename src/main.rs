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
use self::burger::BurgerItem;

const SCALE: f32 = 3.75;

extern crate quicksilver;

struct MainState {
    animation: Asset<Animation>,
    ingredients: Asset<Ingredients>,
    // font: Asset<Image>,
    game_ui: Asset<Image>,
    pos_x: f32,
    pos_y: f32,
    holding: Option<BurgerItem>,
    mouse_down: bool,
    init_down: bool,
}

impl MainState {

    fn draw_dragging(&mut self, window: &mut Window) -> Result<()> {
        if self.holding.is_some() {
            let pos_x = self.pos_x;
            let pos_y = self.pos_y;
            let item = self.holding.as_ref().unwrap().to_str();
            self.ingredients.execute(|ing| {
                let img = ing.get(item).unwrap();
                window.draw_ex(&
                    Rectangle::new(
                        Vector::new( pos_x, pos_y ),
                        Vector::new(32., 32.)
                    ),
                    Img(&img),
                    Transform::scale(Vector::new(3., 3.)),
                    100,
                );
                Ok(())
            })?;
        }
        Ok(())
    }
    fn draw_ui(&mut self, window: &mut Window) -> Result<()> {
        // draw main bg
        self.game_ui.execute(|image| {
            window.draw(&image.area(), Img(&image));
            Ok(())
        })?;
        self.draw_ingredients(window)?;
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
        // let font = Asset::new(Font::load("fonts/CourierPrime.ttf")
        //     .and_then(|font| {
        //         let style = FontStyle::new(72.0, Color::BLACK);
        //         result(font.render("Sample Text", &style))
        //     }));
        let game_ui = Asset::new(Image::load("gameui.png"));
        let ingredients = Asset::new(Ingredients::new());

        let pos_x = 0.;
        let pos_y = 0.;
        Ok(MainState {
            // font,
            animation,
            ingredients,
            pos_x,
            pos_y,
            game_ui,
            holding: None,
            mouse_down: false,
            init_down: false,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.animation.execute(|anim| anim.update(window) )
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::CYAN)?;
        self.draw_ui(window)?;
        self.draw_dragging(window)?;


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
            ) => {
                self.animation.execute(|anim|anim.play())?;
                self.mouse_down = true;
                self.init_down = true;
            }

            Event::MouseMoved(v) => {
                println!("{:#?}", v);
                if self.mouse_down {
                    self.pos_x = v.x;
                    self.pos_y = v.y;
                }

                if self.init_down {
                    println!("start dragging");
                    let itm = drag_item(&v);
                }
                self.init_down = false
            }

            _ => (),
        }
        Ok(())
    }
}

fn drag_item(mouse: &Vector) -> Option<BurgerItem> {
    use self::BurgerItem::*;
    let items = vec![
        // x0, y0, x1, y1
        ((415., 444., 500., 475.), TopBun),
        // ((515., 444., 400., 475), Fish),
        // ((615., 444., 400., 475), Onion),
        // ((715., 444., 400., 475), Beef),
        // ((815., 444., 400., 475), Cheese),

        // ((315., 444., 400., 475), BottomBun),
        // ((315., 444., 400., 475), Chicken),
    ];
    for &(ref pos, ref item) in &items {
        if mouse.x > pos.0 && mouse.x < pos.2
        && mouse.y > pos.1 && mouse.y < pos.3 {
            return Some(item.clone());
        }
    }
    None
}

fn main() {
    run::<MainState>("Image Example", Vector::new(800, 600), Settings {
        // icon_path: Some("image.png"), // Set the window icon
        ..Settings::default()
    });
}
