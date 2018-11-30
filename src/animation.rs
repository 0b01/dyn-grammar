use ggez::{timer, graphics};
use ggez::graphics::{DrawMode, Point2, DrawParam};
use ggez::Context;

pub struct Animation {
    sprite: graphics::Image,
    pub played: bool,
    nframes: usize,
    /// in seconds
    duration: f64,
    quads: Vec<graphics::Rect>,
    current_t: f64,
}

impl Animation {
    pub fn new(ctx: &mut Context, src: &str, frame_w: usize, duration: f64) -> Self {
        let mut sprite = graphics::Image::new(ctx, src).unwrap();
        sprite.set_filter(graphics::FilterMode::Nearest);
        let nframes = sprite.width() as usize / frame_w;
        let mut quads = Vec::new();
        for i in 0..nframes {
            quads.push(graphics::Rect {
                x: i as f32 * 1./12.,
                y: 0.,
                w: 1./12.,
                h: 1.,
            });
        }
        Animation {
            played: false,
            sprite,
            nframes,
            duration,
            quads,
            current_t: 0.,
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        self.current_t += timer::duration_to_f64(timer::get_delta(ctx));
        if self.current_t >= self.duration {
            self.current_t -= self.duration
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, pos_x: f32, pos_y: f32) {
        if self.played {
            return;
        }

        let frame = (self.current_t / self.duration * self.nframes as f64).floor() as usize + 1;
        let nth = frame % self.nframes;
        let src = self.quads[nth];

        let dest = graphics::Point2::new( pos_x, pos_y );
        let scale = graphics::Point2::new(5., 5.);
        graphics::draw_ex(ctx, &self.sprite, DrawParam {
            src,
            dest,
            rotation: 0.,
            scale,
            ..Default::default()
        }).unwrap();

        if nth == self.nframes - 1 {
            self.played = true;
        }
    }

    pub fn play(&mut self) {
        self.played = false;
        self.current_t = 0.;
    }
}
