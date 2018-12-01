use crate::prelude::*;
// use ggez::{timer, graphics};
// use ggez::graphics::{DrawMode, Point2, DrawParam};
// use ggez::Context;

pub struct Animation {
    imgs: Vec<Image>,
    played: bool,
    nframes: usize,
    /// in seconds
    duration: f64,
    current_t: f64,
}

impl Animation {

    pub fn new(src: &'static str, frame_w: usize, duration: f64) -> impl Future<Item=Self, Error=Error> {
        load_file(src)
            .map(|data| Image::from_bytes(data.as_slice()))
            .map(move |sheet| {
                Animation::from_image(sheet.unwrap(), frame_w, duration)
            })
            // .and_then(result)
    }

    pub fn from_image(image: Image, frame_w: usize, duration: f64) -> Animation {
        let nframes = image.area().width() as usize / frame_w;
        let mut imgs = Vec::new();
        for i in 0..nframes {
            let region = Rectangle::new(
                Vector { x: i as f32 * 96., y: 0. },
                Vector { x: 96., y: 96. },
            );
            imgs.push(image.subimage(region));
        }

        Animation {
            imgs,
            played: false,
            nframes,
            duration,
            current_t: 0.,
        }
    }

    pub fn update(&mut self, window: &mut Window) -> Result<()> {
        self.current_t += window.update_rate() * 0.001;
        if self.current_t >= self.duration {
            self.current_t -= self.duration
        }

        if self.nth() == self.nframes - 1 {
            self.played = true;
        }

        Ok(())
    }

    pub fn nth(&self) -> usize {
        let frame = (self.current_t / self.duration * self.nframes as f64).floor() as usize + 1;
        let nth = frame % self.nframes;
        nth
    }

    pub fn draw(&self, window: &mut Window, pos_x: f32, pos_y: f32, scale: f32) {
        if self.played {
            return;
        }
        let src = &self.imgs[self.nth()];

        // let dest = Point2::new( pos_x, pos_y );
        // let scale = Point2::new(5., 5.);
        window.draw_ex(&
            Rectangle::new(
                Vector::new(pos_x, pos_y),
                Vector::new(96., 96.)
            ),
            Img(&src),
            Transform::scale(Vector::new(scale, scale)),
            100,
        );
    }

    pub fn play(&mut self) -> Result<()> {
        self.played = false;
        self.current_t = 0.;
        Ok(())
    }
}
