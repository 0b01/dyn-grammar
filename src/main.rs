//! Save WackDonald's from bankrupcy by putting your compiler construction skills to good use.
//!
//! The famouse burger chain is entering the catering space. Help rewrite the recipes with clear and concise grammar.

#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
mod grammar;
mod burger;
mod animation;
mod ingredients;
mod prelude;
use crate::prelude::*;
// use self::animation::Animation;
use self::ingredients::{Ingredients, IngredientAnimations};
use self::burger::{BurgerItem, Burger};

extern crate quicksilver;

struct MainState {
    S: Asset<Image>,
    A: Asset<Image>,
    B: Asset<Image>,
    C: Asset<Image>,
    Epsilon: Asset<Image>,

    ingredients: Asset<Ingredients>,
    ing_anim: Asset<IngredientAnimations>,

    game_ui: Asset<Image>,

    burger: Burger,

    burger_seq: BurgerAnimSeq,

    pos_x: f32,
    pos_y: f32,
    holding: Option<BurgerItem>,
    mouse_down: bool,
    init_down: bool,
    init_up: bool,
}

impl MainState {

    fn draw_dragging(&mut self, window: &mut Window) -> Result<()> {
        if self.holding.is_some() {
            let pos_x = self.pos_x - 16.;
            let pos_y = self.pos_y - 16.;
            let item = self.holding.as_ref().unwrap().to_str();

            match self.holding.as_ref().unwrap() {
                BurgerItem::NonTermA => {
                    self.A.execute(|image| {
                        window.draw_ex(&
                            Rectangle::new(
                                Vector::new(pos_x, pos_y),
                                Vector::new(32., 32.)
                            ),
                            Img(&image),
                            Transform::scale(Vector::new(3., 3.)),
                            1000,
                        );
                        Ok(())
                    })?;
                },
                BurgerItem::NonTermB => {
                    self.B.execute(|image| {
                        window.draw_ex(&
                            Rectangle::new(
                                Vector::new(pos_x, pos_y),
                                Vector::new(32., 32.)
                            ),
                            Img(&image),
                            Transform::scale(Vector::new(3., 3.)),
                            1000,
                        );
                        Ok(())
                    })?;
                }
                BurgerItem::NonTermC => {
                    self.C.execute(|image| {
                        window.draw_ex(&
                            Rectangle::new(
                                Vector::new(pos_x, pos_y),
                                Vector::new(32., 32.)
                            ),
                            Img(&image),
                            Transform::scale(Vector::new(3., 3.)),
                            1000,
                        );
                        Ok(())
                    })?;
                }
                BurgerItem::NonTermEpsilon => {
                    self.Epsilon.execute(|image| {
                        window.draw_ex(&
                            Rectangle::new(
                                Vector::new(pos_x, pos_y),
                                Vector::new(32., 32.)
                            ),
                            Img(&image),
                            Transform::scale(Vector::new(3., 3.)),
                            1000,
                        );
                        Ok(())
                    })?;
                }
                BurgerItem::NonTermS => {
                    self.S.execute(|image| {
                        window.draw_ex(&
                            Rectangle::new(
                                Vector::new(pos_x, pos_y),
                                Vector::new(32., 32.)
                            ),
                            Img(&image),
                            Transform::scale(Vector::new(3., 3.)),
                            1000,
                        );
                        Ok(())
                    })?;
                }
                _ => {
                    self.ingredients.execute(|ing| {
                        let img = ing.get(item).unwrap();
                        window.draw_ex(&
                            Rectangle::new(
                                Vector::new( pos_x, pos_y ),
                                Vector::new(32., 32.)
                            ),
                            Img(&img),
                            Transform::scale(Vector::new(3., 3.)),
                            1000,
                        );
                        Ok(())
                    })?;
                }
            }
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
        let init_x = 340.;
        let init_y = 425.;
        let n_per_line = 5;

        self.S.execute(|image| {
            window.draw_ex(&
                Rectangle::new(
                    Vector::new(init_x, init_y + 2. * objheight),
                    Vector::new(32., 32.)
                ),
                Img(&image),
                Transform::scale(Vector::new(3., 3.)),
                100,
            );
            Ok(())
        })?;

        self.Epsilon.execute(|image| {
            window.draw_ex(&
                Rectangle::new(
                    Vector::new(init_x, init_y + 3. * objheight),
                    Vector::new(32., 32.)
                ),
                Img(&image),
                Transform::scale(Vector::new(3., 3.)),
                100,
            );
            Ok(())
        })?;

        self.A.execute(|image| {
            window.draw_ex(&
                Rectangle::new(
                    Vector::new(init_x + objwidth, init_y + 2. * objheight),
                    Vector::new(32., 32.)
                ),
                Img(&image),
                Transform::scale(Vector::new(3., 3.)),
                100,
            );
            Ok(())
        })?;

        self.B.execute(|image| {
            window.draw_ex(&
                Rectangle::new(
                    Vector::new(init_x + 2. * objwidth, init_y + 2. * objheight),
                    Vector::new(32., 32.)
                ),
                Img(&image),
                Transform::scale(Vector::new(3., 3.)),
                100,
            );
            Ok(())
        })?;

        self.C.execute(|image| {
            window.draw_ex(&
                Rectangle::new(
                    Vector::new(init_x + 3. * objwidth, init_y + 2. * objheight),
                    Vector::new(32., 32.)
                ),
                Img(&image),
                Transform::scale(Vector::new(3., 3.)),
                100,
            );
            Ok(())
        })?;



        self.ingredients.execute(
            |ing| {
                let srcs = Ingredients::srcs();
                // draw slices
                for (i, src) in srcs.iter().enumerate() { let img = ing.get(src).unwrap();
                    let x = (i % n_per_line) as f32 * objwidth;
                    let y = (i / n_per_line) as f32 * objheight;
                    window.draw_ex(&
                        Rectangle::new(
                            Vector::new(init_x + x, init_y + y ),
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
        // macro_rules! font {
        //     ($i: expr) => {
        //         Asset::new(Font::load("fonts/CourierPrime.ttf")
        //         .and_then(|font| {
        //             let style = FontStyle::new(42.0, Color::BLACK);
        //             result(font.render($i, &style))
        //         }));
        //     };
        // }

        let S = Asset::new(Image::load("nontermS.png"));
        let A = Asset::new(Image::load("nontermA.png"));
        let B = Asset::new(Image::load("nontermB.png"));
        let C = Asset::new(Image::load("nontermC.png"));
        let Epsilon = Asset::new(Image::load("nonterme.png"));

        let game_ui = Asset::new(Image::load("gameui.png"));
        let ingredients = Asset::new(Ingredients::new());
        let ing_anim = Asset::new(IngredientAnimations::new());

        let burger = Burger::new();
        let burger_seq = BurgerAnimSeq::new(burger.clone());

        let pos_x = 0.;
        let pos_y = 0.;
        Ok(MainState {
            A, B, C, S, Epsilon,
            ing_anim,
            ingredients,
            burger,
            burger_seq,
            pos_x,
            pos_y,
            game_ui,
            holding: None,
            mouse_down: false,
            init_down: false,
            init_up: false,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        // self.animation.execute(|anim| anim.update(window))?;
        self.ing_anim.execute(|ing| ing.update(window))?;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::CYAN)?;
        self.draw_ui(window)?;
        self.draw_dragging(window)?;

        self.burger_seq.draw(window, &mut self.ingredients, &mut self.ing_anim)?;

        // self.burger.draw(window, &mut self.ingredients)?;

        Ok(())

    }

    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        match event {
            Event::MouseButton(
                MouseButton::Left,
                ButtonState::Pressed
            ) => {

                self.burger_seq.step(&mut self.ing_anim)?;

                self.mouse_down = true;
                self.init_down = true;
            }

            Event::MouseButton(
                MouseButton::Left,
                ButtonState::Released
            ) => {
                self.mouse_down = false;
                self.init_up = true;
                self.holding = None;
            }

            Event::MouseMoved(v) => {
                // println!("{:#?}", v);
                if self.mouse_down {
                    self.pos_x = v.x;
                    self.pos_y = v.y;
                }

                if self.init_down {
                    println!("start dragging");
                    self.holding = start_drag_item(&v);
                }
                self.init_down = false;

                if self.init_up {
                }
                self.init_up = false;
            }

            _ => (),
        }
        Ok(())
    }
}

struct Dropzone;

fn drop_zone(mouse: &Vector) -> Option<Dropzone> {
    unimplemented!()
}

fn start_drag_item(mouse: &Vector) -> Option<BurgerItem> {
    use self::BurgerItem::*;
    let init_x = 308.;
    let fin_x = 399.;
    let init_y = 380.;
    let fin_y = 458.;
    let line_h = 40.;
    let items = vec![

        ((init_x, init_y, fin_x, fin_y), TopBun),
        ((1. * 100. + init_x, init_y, 1.* 100. + fin_x, fin_y), Fish),
        ((2. * 100. + init_x, init_y, 2.* 100. + fin_x, fin_y), Onion),
        ((3. * 100. + init_x, init_y, 3.* 100. + fin_x, fin_y), Beef),
        ((4. * 100. + init_x, init_y, 4.* 100. + fin_x, fin_y), Cheese),

        ((            init_x, 1. * line_h + init_y, 400., 1. * line_h + fin_y), BottomBun),
        ((1. * 100. + init_x, 1. * line_h + init_y, 500., 1. * line_h + fin_y), Chicken),
        ((2. * 100. + init_x, 1. * line_h + init_y, 600., 1. * line_h + fin_y), Lettuce),

        ((            init_x, 2. * line_h + init_y, 400., 2. * line_h + fin_y), NonTermS),
        ((1. * 100. + init_x, 2. * line_h + init_y, 500., 2. * line_h + fin_y), NonTermA),
        ((2. * 100. + init_x, 2. * line_h + init_y, 600., 2. * line_h + fin_y), NonTermB),
        ((3. * 100. + init_x, 2. * line_h + init_y, 700., 2. * line_h + fin_y), NonTermC),

        ((            init_x, 3. * line_h + init_y, 400., 3. * line_h + fin_y), NonTermEpsilon),

        ((580., 550., 580. + 70., 550. + 60.), Ketchup),
        ((580. + 70. * 1., 550., 580. + 70. * 2., 550. + 60.), Mayo),
        ((580. + 70. * 2., 550., 580. + 70. * 3., 550. + 60.), BBQ),

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
