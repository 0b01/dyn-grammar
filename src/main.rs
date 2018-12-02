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
mod game;
mod prelude;
use crate::prelude::*;
// use self::animation::Animation;
use self::ingredients::Sprites;
use self::burger::{BurgerItem, Burger};
use self::game::{Game, GameBurgerRule};

extern crate quicksilver;

struct MainState {
    Sprites: Asset<Sprites>,

    game_ui: Asset<Image>,

    burger: Rc<RefCell<Burger>>,

    burger_seq: Rc<RefCell<BurgerAnimSeq>>,

    pos_x: f32,
    pos_y: f32,
    holding: Option<BurgerItem>,
    mouse_down: bool,

    play_pressed: bool,

    game: Rc<RefCell<Game>>,
}


impl MainState {

    fn draw_dragging(&mut self, window: &mut Window) -> Result<()> {
        if self.holding.is_some() {
            let pos_x = self.pos_x - 16.;
            let pos_y = self.pos_y - 16.;
            let item = self.holding.as_ref().unwrap().to_str();

            macro_rules! ing {
                ($name: expr) => {
                    self.Sprites.execute(|ing| {
                        let img = ing.get_img($name).unwrap();
                        window.draw_ex(&
                            Rectangle::new(
                                Vector::new(pos_x, pos_y),
                                Vector::new(32., 32.)
                            ),
                            Img(&img),
                            Transform::scale(Vector::new(3., 3.)),
                            1000,
                        );
                        Ok(())
                    })?;

                };
            }

            match self.holding.as_ref().unwrap() {
                BurgerItem::NonTermA => {
                    ing!("nontermA");
                },
                BurgerItem::NonTermB => {
                    ing!("nontermB");
                }
                BurgerItem::NonTermC => {
                    ing!("nontermC");
                }
                BurgerItem::NonTermEpsilon => {
                    ing!("nonterme");
                }
                BurgerItem::NonTermS => {
                    ing!("nontermS");
                }
                _ => {
                    self.Sprites.execute(|ing| {
                        let img = ing.get_img(item).unwrap();
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
        self.draw_Sprites(window)?;
        self.draw_btn(window)?;
        Ok(())
    }

    fn draw_btn(&mut self, window: &mut Window) -> Result<()> {
        let pressed = self.play_pressed;
        self.Sprites.execute(|ing| {
            let image = if pressed {
                ing.get_img("buttondown").unwrap()
            } else {
                ing.get_img("buttonup").unwrap()
            };
            window.draw_ex(&
                Rectangle::new(
                    Vector::new(600., 380.),
                    Vector::new(32., 32.)
                ),
                Img(&image),
                Transform::scale(Vector::new(1.5, 1.5)),
                100,
            );
            Ok(())
        })?;
        Ok(())
    }

    fn draw_Sprites(&mut self, window: &mut Window) -> Result<()> {
        let objheight = 40.;
        let objwidth = 100.;
        let init_x = 340.;
        let init_y = 425.;
        let n_per_line = 5;

        self.Sprites.execute(|ing| {
            let image = ing.get_img("nontermS").unwrap();
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

        self.Sprites.execute(|ing| {
            let image = ing.get_img("nonterme").unwrap();
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

        self.Sprites.execute(|ing| {
            let image = ing.get_img("nontermA").unwrap();
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

        self.Sprites.execute(|ing| {
            let image = ing.get_img("nontermB").unwrap();
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

        self.Sprites.execute(|ing| {
            let image = ing.get_img("nontermC").unwrap();
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



        self.Sprites.execute(
            |ing| {
                let srcs = Sprites::srcs();
                // draw slices
                for (i, src) in srcs.iter().enumerate() { let img = ing.get_img(src).unwrap();
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
                // draw bottles
                let bottles = vec!["ketchupbottle", "mayobottle", "bbqbottle" ];
                for (i, src) in bottles.iter().enumerate() {
                    let img = ing.get_img(src).unwrap();
                    let x = i as f32 * 74.;
                    window.draw_ex(&
                        Rectangle::new(
                            Vector::new(575. + x, 590.),
                            Vector::new(96., 96.)
                        ),
                        Img(&img),
                        Transform::scale(Vector::new(3., 3.)),
                        101,
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

        let game_ui = Asset::new(Image::load("game_ui.png"));
        let Sprites = Asset::new(Sprites::new());

        let burger = Rc::new(RefCell::new(Burger::new()));
        let burger_seq = Rc::new(RefCell::new( BurgerAnimSeq::new(burger.borrow().clone()) ));

        let pos_x = 0.;
        let pos_y = 0.;

        let mut grams = Vec::new();
        let mut g1 = GameBurgerRule::new(Vector::new(45., 20.  ), 0);
        g1.set_name(BurgerItem::NonTermS);
        g1.set_item(0, BurgerItem::TopBun);
        g1.set_item(1, BurgerItem::NonTermS);
        g1.set_item(2, BurgerItem::BottomBun);
        let mut g2 = GameBurgerRule::new(Vector::new(152., 20. ), 1);
        g2.set_name(BurgerItem::NonTermS);
        grams.push(g1);
        grams.push(g2);
        grams.push(GameBurgerRule::new(Vector::new(45., 310. ), 2));
        grams.push(GameBurgerRule::new(Vector::new(152., 310.), 3));

        let game = Rc::new(RefCell::new(Game::new(grams)));

        Ok(MainState {
            Sprites,
            burger,
            burger_seq,
            pos_x,
            pos_y,
            game_ui,
            game,
            holding: None,
            mouse_down: false,
            play_pressed: false,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        // self.animation.execute(|anim| anim.update(window))?;
        self.Sprites.execute(|ing| ing.update_anim(window))?;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::CYAN)?;
        self.draw_ui(window)?;
        self.draw_dragging(window)?;

        let burger_seq = Rc::clone(&self.burger_seq);
        let game = Rc::clone(&self.game);
        self.Sprites.execute(|ingr|{
            burger_seq.borrow_mut().draw(window, ingr)?;
            game.borrow_mut().draw(window, ingr)?;
            Ok(())
        })?;

        // self.burger.draw(window, &mut self.Sprites)?;

        Ok(())

    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::MouseButton(
                MouseButton::Left,
                ButtonState::Pressed
            ) => {

                let v = window.mouse().pos();

                println!("{:?}", v);
                self.pos_x = v.x;
                self.pos_y = v.y;

                self.holding = start_drag_item(&v);
                self.play_pressed = play_pressed(&v);

                let burger_seq = self.burger_seq.clone();
                let game = self.game.clone();
                if self.play_pressed {
                    self.Sprites.execute(|i| {
                        burger_seq.borrow_mut().step(i)?;
                        let mut g = game.borrow().as_grammar();
                        g.build().unwrap();
                        println!("{:#?}", g);
                        Ok(())
                        // burger_seq.borrow_mut().cont(i)
                    })?;
                }
                self.mouse_down = true;
            }

            Event::MouseButton(
                MouseButton::Left,
                ButtonState::Released
            ) => {
                let v = window.mouse().pos();
                self.game.borrow_mut().drop_item(&v, self.holding);

                self.mouse_down = false;
                self.holding = None;
                self.play_pressed = false;
            }

            Event::MouseMoved(v) => {
                // println!("{:#?}", v);
                if self.mouse_down {
                    self.pos_x = v.x;
                    self.pos_y = v.y;
                }
            }

            _ => (),
        }
        Ok(())
    }
}

fn play_pressed(mouse: &Vector) -> bool {
    let play = (597., 376., 638., 414.);
    if mouse.x > play.0 && mouse.y > play.1
    && mouse.x < play.2 && mouse.y < play.3 {
        true
    } else {
        false
    }
}

fn start_drag_item(mouse: &Vector) -> Option<BurgerItem> {
    use self::BurgerItem::*;
    let init_x = 308.;
    let fin_x = 399.;
    let init_y = 408.;
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
    Option::None
}

fn main() {
    run::<MainState>("Image Example", Vector::new(800, 600), Settings {
        // icon_path: Some("image.png"), // Set the window icon
        ..Settings::default()
    });
}
