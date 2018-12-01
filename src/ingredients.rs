use crate::prelude::*;
use std::collections::HashMap;

pub struct Ingredients {
    items: HashMap<String, Image>,
    anims: HashMap<String, Animation>,
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
            "ketchupbottle", "mayobottle", "bbqbottle", "ketchup", "mayo", "bbq",
            "nonterme", "nontermA", "nontermS", "nontermB", "nontermC", "buttonup", "buttondown",

        ]);

        let img_futs = srcs.into_iter().map(move |src| {
            load_file(src.to_owned() + ".png")
                .map(move |data|
                    (src, Image::from_bytes(data.as_slice()).unwrap())
                )
        });

        let mut anims = Ingredients::anims();
        anims.extend(vec![
            "squeeze_bbq", "squeeze_ketchup", "squeeze_mayo",
        ]);

        let anim_futs = anims.into_iter().map(move |src| {
            load_file(src.to_owned() + ".png")
                .map(move |data|
                    (src, Image::from_bytes(data.as_slice()).unwrap())
                )
        });

        let fut_anim = join_all(anim_futs)
            .map(|vec| {
                let mut anims = HashMap::new();
                for (src, img) in vec.into_iter() {
                    let anim = Animation::from_image(img, 96, 1.2);
                    anims.insert(src.to_string(), anim);
                }
                anims
            });

        let fut_items = join_all(img_futs)
            .map(|vec| {
                let mut items = HashMap::new();
                for (src, img) in vec.into_iter() {
                    items.insert(src.to_string(), img);
                }
                items
            });

        let ret = fut_anim.join(fut_items)
            .map(|(anims,items)| Ingredients {
                items, anims
            });
        ret
    }

    pub fn get_img(&self, name: &str) -> Option<&Image> {
        self.items.get(name)
    }

}

impl Ingredients {

    pub fn anims() -> Vec<&'static str> {
        vec![
            "place_beef", "place_bottom_bun", "place_cheese", "place_chicken",
            "place_fish", "place_lettuce", "place_onion", "place_top_bun",
            "cutbread",
        ]
    }

    pub fn get_anim(&self, name: &str) -> Option<&Animation> {
        self.anims.get(name)
    }

    pub fn get_anim_mut(&mut self, name: &str) -> Option<&mut Animation> {
        self.anims.get_mut(name)
    }

    pub fn update_anim(&mut self, window: &mut Window) -> Result<()> {
        for i in self.anims.values_mut() {
            i.update(window)?;
        }
        Ok(())
    }

    pub fn set_duration(&mut self, duration: f64) -> Result<()> {
        for i in self.anims.values_mut() {
            i.set_duration(duration);
        }
        Ok(())
    }


    pub fn draw_anim(&mut self, window: &mut Window, pos_x: f32, pos_y: f32, scale: f32) -> Result<()> {
        for i in self.anims.values_mut() {
            i.draw(window, pos_x, pos_y, scale);
        }
        Ok(())
    }

}