use crate::prelude::*;
use std::collections::HashMap;

pub struct Ingredients {
    items: HashMap<String, Image>,
}

impl Ingredients {

    pub fn srcs() -> Vec<&'static str> {
        let srcs = vec![
            "top_bun", "fish", "onion", "beef_patty", "cheese",
            "bottom_bun", "chicken",
            ];
        srcs
    }

    pub fn new() -> impl Future<Item=Self, Error=Error> {
        let mut srcs = Ingredients::srcs();
        srcs.extend(vec!["ketchupbottle", "mayobottle", "bbqbottle" ]);

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