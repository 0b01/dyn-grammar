use crate::prelude::*;
use crate::burger::*;

pub struct Orders {
    pub orders: Vec<Burger>,
    pub selected: usize,
}

impl Orders {
    pub fn new() -> Self {
        let mut orders = vec![];
        Self {
            orders,
            selected: 0,
        }
    }

    pub fn draw(&mut self, window: &mut Window, ing: &mut Sprites) -> Result<()> {
        self.draw_orders(window, ing)?;
        self.draw_selected_burger(window, ing)?;
        Ok(())
    }

    pub fn draw_selected_burger(&mut self, window: &mut Window, ing: &mut Sprites) -> Result<()> {
        Ok(())
    }

    pub fn draw_orders(&mut self, window: &mut Window, ing: &mut Sprites) -> Result<()> {
        let timg = ing.get_img("orderhover").unwrap();
        let fimg = ing.get_img("order").unwrap();
        let selected = ing.get_img("orderselect").unwrap();
        for i in 0..10 {
            let image = if i == self.selected { selected }
                    else { fimg };

            window.draw_ex(&
                Rectangle::new(
                    Vector::new(250. + 50. * i as f32, 30.),
                    Vector::new(32., 32.)
                ),
                Img(&image),
                Transform::scale(Vector::new(1.5, 1.5)),
                100,
            );
        }
        Ok(())
    }

    pub fn mouse_move(&mut self, v: &Vector) {
        for i in 0..10 {
            if (v.y > 28.453894 && v.y < 66.80449)
            && v.x > (250.59584 + i as f32 * 50.)
            && v.x < (281.54004 + i as f32 * 50.) {
                self.selected = i;
            }
        }
    }

}