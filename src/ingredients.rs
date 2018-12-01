use crate::prelude::*;
use std::collections::HashMap;

pub struct Ingredients {
    items: HashMap<String, Image>,
}

impl Ingredients {

    pub fn srcs() -> Vec<&'static str> {
        vec![
            "top_bun", "fish", "onion", "beef_patty", "cheese",
            "bottom_bun", "chicken", "lettuce"
        ]
    }

    pub fn new() -> impl Future<Item=Self, Error=Error> {
        let mut srcs = Ingredients::srcs();
        srcs.extend(vec![
            "ketchupbottle", "mayobottle", "bbqbottle", "ketchup", "mayo", "bbq"
        ]);

        let futs = srcs.into_iter().map(move |src| {
            load_file(src.to_owned() + ".png")
                .map(move |data|
                    (src, Image::from_bytes(data.as_slice()).unwrap())
                )
        });

        let ret = join_all(futs)
            .map(|vec| {
                let mut items = HashMap::new();
                for (src, img) in vec.into_iter() {
                    items.insert(src.to_string(), img);
                }

                Ingredients {
                    items,
                }
            });
        ret
    }

    pub fn get(&self, name: &str) -> Option<&Image> {
        self.items.get(name)
    }

}


pub struct IngredientAnimations {
    items: HashMap<String, Animation>,
}

impl IngredientAnimations {

    pub fn srcs() -> Vec<&'static str> {
        vec![
            "place_beef", "place_bottom_bun", "place_cheese", "place_chicken",
            "place_fish", "place_lettuce", "place_onion", "place_top_bun",
            "cutbread",
        ]
    }

    pub fn new() -> impl Future<Item=Self, Error=Error> {
        let mut srcs = IngredientAnimations::srcs();
        srcs.extend(vec![
            "squeeze_bbq", "squeeze_ketchup", "squeeze_mayo",
        ]);

        let futs = srcs.into_iter().map(move |src| {
            load_file(src.to_owned() + ".png")
                .map(move |data|
                    (src, Image::from_bytes(data.as_slice()).unwrap())
                )
        });

        let ret = join_all(futs)
            .map(|vec| {
                let mut items = HashMap::new();
                for (src, img) in vec.into_iter() {
                    let anim = Animation::from_image(img, 96, 1.2);
                    items.insert(src.to_string(), anim);
                }

                IngredientAnimations {
                    items,
                }
            });
        ret
    }

    pub fn get(&self, name: &str) -> Option<&Animation> {
        self.items.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Animation> {
        self.items.get_mut(name)
    }

    pub fn update(&mut self, window: &mut Window) -> Result<()> {
        for i in self.items.values_mut() {
            i.update(window)?;
        }
        Ok(())
    }

    pub fn draw(&mut self, window: &mut Window, pos_x: f32, pos_y: f32, scale: f32) -> Result<()> {
        for i in self.items.values_mut() {
            i.draw(window, pos_x, pos_y, SCALE);
        }
        Ok(())
    }

}