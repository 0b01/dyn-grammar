use crate::prelude::*;
use crate::grammar::Token;
use super::BurgerItem;

#[derive(Clone)]
pub struct BurgerAnimSeq {
    current_t: f64,
    pub burger: Burger,
    idx: usize,
    static_idx: usize,
    pub drawing: Option<BurgerItem>,
    pub play_continuous: bool,
}

impl BurgerAnimSeq {
    pub fn new(burger: Burger) -> Self {
        // let played = vec![false; burger.toks.len()];
        Self {
            current_t: 0.,
            burger,
            idx: 0,
            static_idx: 0,
            drawing: None,
            play_continuous: false,
        }
    }

    pub fn draw(
        &mut self,
        window: &mut Window,
        Sprites: &mut Sprites,
    ) -> Result<()> {

        let dy = 13.;
        Sprites.draw_anim(window, 561., 210. - self.idx as f32 * dy, 3.)?;

        if self.drawing.is_some() {
            let anim = self.drawing.as_ref().unwrap().to_anim_str();
            let played = Sprites.get_anim_mut(anim).unwrap().played;
            if played {
                println!("Done playing", );
                self.static_idx += 1;
                self.drawing = None;
                if self.static_idx == self.burger.toks.len() {
                    self.play_continuous = false;
                }
                if self.play_continuous {
                    self.step(Sprites)?;
                }
            }
        }
        self.burger.draw(window, Sprites, Some(self.static_idx))?;

        Ok(())
    }

    pub fn step(
        &mut self,
        ingr: &mut Sprites,
    ) -> Result<()> {
        if self.drawing.is_some() { return Ok(()); }
        if let Token::Terminal(itm) = &self.burger.toks[self.idx] {
            self.drawing = Some(itm.clone());
            let anim = itm.to_anim_str();
            ingr.get_anim_mut(anim).unwrap().play()?;
        }

        self.idx += 1;
        Ok(())
    }

    pub fn cont(
        &mut self,
        ing: &mut Sprites,
    ) -> Result<()> {
        self.idx = 0;
        self.static_idx = 0;
        self.play_continuous = true;
        ing.set_duration(0.3)?;
        self.step(ing)
    }

    pub fn stop(&mut self, ingr: &mut Sprites) {
        self.play_continuous = false;
        self.idx = 0;
        self.static_idx = 0;
        self.drawing = None;
        let _ =ingr.anims.values_mut().map(|a| {
            a.played = true;
            a.current_t = 0.;
        }).collect::<Vec<_>>();
    }

}