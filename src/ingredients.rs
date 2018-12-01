use std::collections::HashMap;

struct Ingredients {
    items: HashMap<&'static str, Image>,
}

impl Ingredients {

    pub fn new() -> impl Future<Item=Self, Error=Error> {
        let mut items = HashMap::new();
        load_file(src)
            .map(|data| Image::from_bytes(data.as_slice()))
            .map(move |sheet| {
                let sheet = sheet.unwrap();
                let nframes = sheet.area().width() as usize / frame_w;
                let mut imgs = Vec::new();
                for i in 0..nframes {
                    let region = Rectangle::new(
                        Vector { x: i as f32 * 96., y: 0. },
                        Vector { x: 96., y: 96. },
                    );
                    imgs.push(sheet.subimage(region));
                }

                Animation {
                    imgs,
                    played: false,
                    nframes,
                    duration,
                    current_t: 0.,
                }
            })
            // .and_then(result)
    }
